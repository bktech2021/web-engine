mod web_engine;
use web_engine::HttpServer;

fn main() -> std::io::Result<()> {
    let web_server = HttpServer::new();
    web_server.handle();
    Ok(())
}