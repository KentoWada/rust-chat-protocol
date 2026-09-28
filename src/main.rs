use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;


const MAX_FRAME: usize = 4096;

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

            'read_loop: loop {
                let bytes_read = match stream.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };

                pending.extend_from_slice(&buffer[..bytes_read]);

                loop {
                    if pending.len() < 4 {
                        break;
                    }

                    let len = u32::from_be_bytes([pending[0], pending[1], pending[2], pending[3]]) as usize;

                    if len > MAX_FRAME {
                        break 'read_loop;
                    }

                    if pending.len() < 4 + len {
                        break;
                    }

                    let frame: Vec<u8> = pending.drain(..4+len).collect();
                    let mut list = connections_clone.lock().unwrap();
                    for(client_addr, stream) in list.iter_mut() {
                        if *client_addr != addr {
                            if let  Err(e) = stream.write_all(&frame){
                                eprintln!("Failed to write to {}: {}", client_addr, e);
                            }
                        }
                    }
                }
            }

            let mut list = connections_clone.lock().unwrap();
            list.retain(|(client_addr, _)| *client_addr != addr);
        });
   }
}
