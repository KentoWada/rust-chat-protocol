use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
   let connections = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
   loop {
        let (mut stream, addr) = listener.accept().unwrap();
        let connections_clone = Arc::clone(&connections);
 
         thread::spawn(move || {
           let stream_for_list = stream.try_clone().unwrap();
            {
                let mut list = connections_clone.lock().unwrap();
                list.push(stream_for_list);
            }

            let mut buffer = [0; 512];
            loop {
                let bytes_read = stream.read(&mut buffer).unwrap();
                if bytes_read==0 {
                    break;
                }
            }
        });
   }

}
