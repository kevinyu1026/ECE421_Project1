use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
use std::io;

struct GameMessage {
    action: String,
    amount: i32,
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:1234").unwrap();
    println!("Connected to the server!");

    let mut message = GameMessage {
        action: "Join".to_string(),
        amount: 0,
    };
    stream.write(&message.action.as_bytes()).expect("Failed to write to stream");

    let mut buffer = [0; 1024];
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    let handle = std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        // loop for reading from the server
        loop {
            buffer.fill(0); // Clear the buffer before each read
            match stream_clone.read(&mut buffer) {
                Ok(0) => {
                    println!("Connection closed by server.");
                    break;
                }
                Ok(_) => {
                    let response = String::from_utf8_lossy(&buffer[..]);
                    println!("Received: {}", response);
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    break;
                }
            }
        }
    });

    // loop for std in
    loop {
        let mut input = String::new();
        println!("Enter a message to send to the server:");
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        message.action = input.trim().to_string();
        stream.write(&message.action.as_bytes()).expect("Failed to write to stream");
    }

    // tbh no idea how both of these send and recieve at the same time
    
    handle.join().expect("Failed to join thread");
    // stream.read(&mut buffer).expect("Failed to read from stream client");
    // let response = String::from_utf8_lossy(&buffer[..]);
    // println!("Received: {}", response);
}
