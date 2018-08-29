extern crate futures;
extern crate tokio;
extern crate tokio_io;

use tokio::executor::current_thread;
use tokio::net::TcpListener;
use tokio_io::io;
use futures::{Future, Stream};
use tokio_io::AsyncRead;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            println!("accepted socket: addr={:?}", socket.peer_addr().unwrap());

            let (r, w) = socket.split();

            let connection = io::copy(r, w).then(|res| {
                println!("wrote message; success={:?}", res);
                Ok(())
            });
            current_thread::spawn(connection);

            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    current_thread::run(|_| {
        current_thread::spawn(server);
        println!("server running on localhost:8080");
    });
}
