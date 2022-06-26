use tcp_stream_echo::dkubiszewski::TcpEcho;
fn main() {
    let port = std::env::args()
        .nth(1)
        .expect("port not provided")
        .parse::<usize>()
        .expect("Port should be positive number");
    let tcp_echo = TcpEcho::new(port, 1024);
    tcp_echo.serve_with_peek(|bytes: &[u8]| {
        print!("> ");
        for byte in bytes {
            print!("{:#02x} ", byte);
        }
        println!();
    });
}
