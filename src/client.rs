use std::{thread::{self, JoinHandle}, net::{TcpStream}, io::{Read, Write}, str::from_utf8};

pub fn make_request() -> JoinHandle<()> {
  return thread::spawn(|| {
    match TcpStream::connect("localhost:3333") {
      Ok(mut stream) => {
        println!("Successfully connected to server in port 3333");

        let msg = b"Hello!";

        stream.write(msg).unwrap();
        println!("Sent Hello, awaiting reply...");

        let mut data = [0 as u8; 6]; // using 6 byte buffer
        match stream.read_exact(&mut data) {
          Ok(_) => {
            println!("Reply is ok!");
            println!("{:?}", from_utf8(&data).unwrap())
          },
          Err(e) => {
              println!("Failed to receive data: {}", e);
          }
        }
      },
      Err(e) => {
        println!("Failed to connect: {}", e);
      }
  }

  println!("Terminated.");
  });
}