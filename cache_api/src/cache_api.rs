use async_trait::async_trait;
use bytes::Bytes;
use http::{Response, StatusCode};
use once_cell::sync::Lazy;
use pingora::apps::http_app::ServeHttp;
use pingora::protocols::http::ServerSession;
use pingora::services::listening::Service;
use pingora_timeout::timeout;
use prometheus::{register_int_counter, IntCounter};
use std::time::Duration;
use pingora_memory_cache::MemoryCache;
use serde::Deserialize;
use log::debug;

#[derive(Deserialize, Debug)]
struct CacheOpRequest {
    operation: String,
    key: String,
    data: Option<String>,
}

enum Operation {
    Put,
    Get,
}

impl Operation {
    fn from_string(value: String) -> Option<Operation> {
        match value.to_lowercase().as_str() {
            "put" => Some(Operation::Put),
            "get" => Some(Operation::Get),
            _ => None
        }
    }
}

// Metrics
static CACHE_PUT_COUNTER: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!("cache_put_counter", "Number of PUT operations").unwrap());
static CACHE_GET_COUNTER: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!("cache_get_counter", "Number of GET operations").unwrap());

// Cache Backend
static MEM_CACHE: Lazy<MemoryCache<String, String>> =
    Lazy::new(|| MemoryCache::new(1024));

// Cache Service
pub struct CacheApi;

#[async_trait]
impl ServeHttp for CacheApi {
    async fn response(&self, http_stream: &mut ServerSession) -> Response<Vec<u8>> {
        // read timeout of 1s
        let read_timeout = 1000;
        let body = match timeout(
            Duration::from_millis(read_timeout),
            http_stream.read_request_body(),
        )
        .await
        {
            Ok(res) => match res.unwrap() {
                Some(bytes) => bytes,
                None => Bytes::from("no body!"),
            },
            Err(_) => {
                panic!("Timed out after {:?}ms", read_timeout);
            }
        };

        match deserialize_from_bytes(&body) {
            Ok(cache_op_req) => {
                debug!("##### Received data: {:#?}", cache_op_req);
               
               match Operation::from_string(cache_op_req.operation) {
                  Some(Operation::Get) => {
                    debug!("##### Running GET operation.");
                    CACHE_GET_COUNTER.inc();

                    let key = cache_op_req.key;
                    let (res, cache_status) = MEM_CACHE.get(&key);

                    let data: String = format!(
                        "key = {} ; res = {} ; cache_status = {}",
                        key, res.unwrap_or("None".to_string()), cache_status.as_str()
                    );
                    debug!("##### {}", data);
                    
                    generate_response(StatusCode::OK, Bytes::copy_from_slice(data.as_bytes()))
                  },
                  Some(Operation::Put) => {
                    debug!("##### Running PUT operation.");
                    CACHE_PUT_COUNTER.inc();

                    let key = cache_op_req.key;
                    if let Some(content) = cache_op_req.data {
                        let ttl = Some(Duration::from_secs(5));
                        
                        MEM_CACHE.put(&key, content, ttl);

                        let msg = format!("Put OK for key {}", key);
                        debug!("##### {}", msg);

                        generate_response(StatusCode::OK, Bytes::copy_from_slice(msg.as_bytes()))

                    } else {
                        return_bad_request("Content to cache is required (data field).")
                    }
                  },
                  None => {
                    return_bad_request("Invalid operation.")
                  }
               }
            },
            Err(_err) => {
                return_bad_request("Problem with input data. Available fields: operation, key and data (optional).")
            },
        }
    }
}

fn deserialize_from_bytes(bytes: &[u8]) -> Result<CacheOpRequest, serde_json::Error> {
    serde_json::from_slice(bytes) 
}

fn generate_response(status: StatusCode, body: Bytes) -> Response<Vec<u8>> {
    Response::builder()
        .status(status)
        .header(http::header::CONTENT_TYPE, "text/html")
        .header(http::header::CONTENT_LENGTH, body.len())
        .body(body.to_vec())
        .unwrap()
}

fn return_bad_request(msg: &str) -> Response<Vec<u8>> {
    debug!("##### Error: {:#?}", msg);
    let body = Bytes::copy_from_slice(msg.as_bytes()); 
    
    generate_response(StatusCode::BAD_REQUEST, body)
}

pub fn create_cache_api_service() -> Service<CacheApi> {
    Service::new("Cache API".to_string(), CacheApi)
}
