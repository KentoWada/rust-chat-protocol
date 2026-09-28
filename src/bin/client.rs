use std::net::TcpStream;

fn make_frame(text: &str) -> Vec<u8> {
    let payload = text.as_bytes();
    let mut frame = Vec::with_capacity(4 + payload.len());
    frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    frame.extend_from_slice(payload);
    frame
}

fn main(){
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
}
