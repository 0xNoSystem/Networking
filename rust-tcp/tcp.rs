//Imports
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_client(mut stream: TcpStream){
    //This is a buffer to read data from the client
    let mut buffer = [0; 1024];
    //This line reads data from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from Client!");
    //this line converts the data in the buffer into a utf-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to wrtie response");
}


//Entry point
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").
    expect("Failed to bind to address");
    println!("Server is listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                std::thread::spawn(|| handle_client(stream));
            }

            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            //srderr - standard error stream
            }
        }
    }
}