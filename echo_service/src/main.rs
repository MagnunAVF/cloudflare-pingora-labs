mod echo_service;

use pingora::server::configuration::Opt;
use pingora::server::Server;
use pingora::services::listening::Service;

use echo_service::create_echo_service_http;

fn main() {
    env_logger::init();

    let opt = Opt::parse_args();
    let mut server = Server::new(Some(opt)).unwrap();
    server.bootstrap();

    let mut echo_service_http = create_echo_service_http();
    echo_service_http.add_uds("/tmp/echo.sock", None);

    let mut prometheus_service_http = Service::prometheus_http_service();
    prometheus_service_http.add_tcp("127.0.0.1:6150");

    server.add_service(echo_service_http);
    server.add_service(prometheus_service_http);

    server.run_forever();
}
