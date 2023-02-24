fn main() {
    let server = Server::new("127.0.0.1:9090");
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    fn run(self) {}
}
