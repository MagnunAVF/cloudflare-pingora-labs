mod cache_api;

use pingora::server::configuration::Opt;
use pingora::server::Server;
use pingora::services::listening::Service;

use cache_api::create_cache_api_service;

fn main() {
    env_logger::init();

    let opt = Opt::parse_args();
    let mut server = Server::new(Some(opt)).unwrap();
    server.bootstrap();

    let mut cache_api_service = create_cache_api_service();
    cache_api_service.add_uds("/tmp/cache_api.sock", None);

    let mut prometheus_service_http = Service::prometheus_http_service();
    prometheus_service_http.add_tcp("127.0.0.1:6150");

    server.add_service(cache_api_service);
    server.add_service(prometheus_service_http);

    server.run_forever();
}
