use std::{thread::{self, JoinHandle}, net::{TcpStream, Shutdown, TcpListener}, io::{Read, Write}};

fn handle_read(mut stream: &TcpStream) -> [u8; 4096] {
  let mut buf = [0u8 ;4096];
  match stream.read(&mut buf) {
    Ok(_) => {
      let req_str = String::from_utf8_lossy(&buf);
      println!("Received Req: {}", req_str);
      buf
    },
    Err(e) => panic!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: &TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body><h1>Hello world</h1></body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(mut stream: TcpStream) {
    handle_read(&stream);
    handle_write(&stream);

    // stream.shutdown(Shutdown::Both).unwrap();    
}


pub fn listen() -> JoinHandle<()> {
  return thread::spawn(|| {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          println!("New connection: {}", stream.peer_addr().unwrap());
          thread::spawn(move|| {
              // connection succeeded
              handle_client(stream)
          });
        }
        Err(e) => {
          println!("Error: {}", e);
          /* connection failed */
        }
      }
    }
    // close the socket server
    drop(listener);
  });
}