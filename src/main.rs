extern crate futures;
extern crate udpproxy;

use udpproxy::UdpSocket;

fn main() -> std::io::Result<()> {

    use futures::{Future, Stream};
    use futures::future::ok;

    let socket = UdpSocket::bind("0.0.0.0:8050")?;

    tokio::run(ok(()).and_then(move |_| {

        tokio::spawn(
            socket.incoming()
                .for_each(|datagram| { println!("{:?}", datagram); Ok(()) })
                .map_err(|_| ())
        );

        tokio::spawn(
            socket.send_to(&[0xde, 0xad, 0xbe, 0xef], "192.168.82.8:8050")?
                .map_err(|_| ())
        );

        Ok(())

    }).map_err(|_: std::io::Error| ()));

    Ok(())
}