use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
   let connections = Arc::new(Mutex::new(Vec::<(SocketAddr, TcpStream)>::new()));
   loop {
        let (mut stream, addr) = listener.accept().unwrap();
        let connections_clone = Arc::clone(&connections);
 
         thread::spawn(move || {
           let stream_for_list = stream.try_clone().unwrap();
            {
                let mut list = connections_clone.lock().unwrap();
                list.push((addr, stream_for_list));
            }

            let mut buffer = [0; 512];
            let mut pending: Vec<u8> = Vec::new();

            loop {

                let bytes_read = match stream.read(&mut buffer) {
                    Ok(0) | Err(_) => {
                        let mut list = connections_clone.lock().unwrap();
                        list.retain(|(client_addr, _)| *client_addr != addr);
                        break;
                    }

                    Ok(n) => n,
                };

                pending.extend_from_slice(&buffer[..bytes_read]);

                while let Some(i) = pending.iter().position(|&b| b==b'\n') {
                    let message: Vec<u8> = pending.drain(..=i).collect();
                    let mut list = connections_clone.lock().unwrap();

                    for (client_addr, stream) in list.iter_mut() {
                        if *client_addr != addr {
                            if let Err(e) = stream.write_all(&message) {
                                eprintln!("Failed to write to {}: {}", client_addr, e);
                            }
                        }
                    }
                }

            }
        });
   }
}
