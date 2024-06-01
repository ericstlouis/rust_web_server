use std::io::prelude::*;
use std::net::{TcpListener, TcpStream}; //brings TcpListener struct and functions into to scope
use std::time::Duration;
use std::{fs, thread};
use web_server::ThreadPool;

fn main() {
    //watches the address to see if anything is happening on that address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //create a pool threads(4) that is ready to handle incoming requests
    let pool = ThreadPool::new(4);

    //loop over incoming connections
    //
    for stream in listener.incoming() {
        //unwrap the stream to handle connections errors
        let stream = stream.unwrap();

        //|| - this is symbol that tells rust its is a closure
        //A closure is self contained code(Function) that has a variable in it and can be exectued at a later time/anytime
        //Basically a Closure is a callback function that is turn into a variable 

        //this is a closure that takes in a stream and returns a string
        //pool.execute asigns the streams to the threads in the threadpool
        //basically it gives the threads in the threadpool a job to do
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    //Reading TCP Stream
    //creates a mutable buffer(buffer in programming is a place to temporaliy store data)
    //the size is 1024 bytes and its filled with 0
    let mut buffer = [0; 1024];

    //reads data from the stream and stores in buffer
    stream.read(&mut buffer).unwrap();

    //prints it
    //from_utf8_lossy - converts the bytes into a string
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //Responding to the TCP Stream

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    //conditionally statement returns the status and filename based off the request the buffer starts with
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        //this is simulating a thread taking 5 seconds to handle this a previous request
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    //store the index.html file as a variable
    let content: String = fs::read_to_string(filename).unwrap();

    let reponse = format!(
        "{}\r\nContent-length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    //converting reponse into bytes and sending it into the stream
    //this usually accumalate data over time and send to the stream that why we need flush
    stream.write(reponse.as_bytes()).unwrap();
    //transmit the buffered data above is sent imedaitly
    stream.flush().unwrap();
}
