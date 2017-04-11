extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
mod codec;
mod protocol;
mod service;
use tokio_proto::TcpServer;
use protocol::LineProto;
use service::Echo;

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:6767".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}

