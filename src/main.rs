mod server;
mod client;

fn main() {
  let server_handle = server::listen();

  // client::make_request();

  server_handle.join().unwrap();
} 