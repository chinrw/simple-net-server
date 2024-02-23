use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    // create a udp socket
    let socket = UdpSocket::bind("0.0.0.0:9090")
        .await
        .expect("Could not bind to address");

    // create a udp server handler
    let mut buf = [0; 1024];
    let mut count = 0;
    loop {
        let (len, addr) = socket
            .recv_from(&mut buf)
            .await
            .expect("Failed to receive from a socket");
        // print the received data
        // println!("Received {} bytes from {}", len, addr);
        count += 1;
        // echo the data back
        let len = socket
            .send_to(b"1", addr)
            .await
            .expect("Failed to send a response");
        // println!("Sent {} bytes back to {}", len, addr);

        println!("Received {} messages", count);
    }
    // hanlde the signal to stop the server
}
