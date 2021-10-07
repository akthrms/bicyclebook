use std::str::FromStr;

trait Server {
    type Response;
    type Request: FromStr;

    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;

impl Server for EchoServer {
    type Response = String;
    type Request = String;

    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

#[allow(dead_code)]
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

fn main() {
    let server = EchoServer;
    assert_eq!(server.handle("Hello".to_string()), "Hello".to_string());
}
