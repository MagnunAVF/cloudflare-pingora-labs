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

pub struct HttpEchoApp;

static REQ_COUNTER: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!("reg_counter", "Number of requests").unwrap());

#[async_trait]
impl ServeHttp for HttpEchoApp {
    async fn response(&self, http_stream: &mut ServerSession) -> Response<Vec<u8>> {
        REQ_COUNTER.inc();
        // read timeout of 2s
        let read_timeout = 2000;
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

        Response::builder()
            .status(StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/html")
            .header(http::header::CONTENT_LENGTH, body.len())
            .body(body.to_vec())
            .unwrap()
    }
}

pub fn create_echo_service_http() -> Service<HttpEchoApp> {
    Service::new("Echo Service HTTP".to_string(), HttpEchoApp)
}
