pub mod executor;
pub mod timer_future;

use executor::{Spawner, new_executor_and_spawner};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// use timer_future::TimerFuture;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let (executor, spawner) = new_executor_and_spawner();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let spawner_clone = spawner.clone();

		spawner_clone.spawn(async {
			println!("Spawning request:");
		    handle_connection(stream);
		    println!("Connection handled.");
		});
		// handle(&spawner.clone(), stream);

        // Drop the spawner so that our executor knows it is finished and won't
        // receive more incoming tasks to run.
        println!("Dropping spawner clone");
        drop(spawner_clone);

        // Run the executor until the task queue is empty.
        executor.run();
    }

    println!("Shutting down.");
}

fn handle(spawner: &Spawner, stream: TcpStream) {
	spawner.spawn(async {
	    handle_connection(stream);
	});
}

// fork/join model, the single-threaded async I/O model,
// or the multi-threaded async I/O model.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

async fn do_something() {
    println!("Something!");
}
