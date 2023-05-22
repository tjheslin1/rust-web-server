use async_std::{
    io::{Read, ReadExt, Write, WriteExt},
    net::{TcpListener},
    task::sleep,
    // task::spawn,
};
use futures::StreamExt;
use std::{fs, time::Duration};

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();

            handle_connection(stream).await;

            // spawn tasks onto separate threads;
            // because handle_connection is both Send and non-blocking
            // spawn(handle_conection(stream));
        })
        .await;

    println!("Shutting down.");
}

const GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";
const SLEEP: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";

// fork/join model or the multi-threaded async I/O model.
async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let (status_line, filename) = if buffer.starts_with(GET) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(SLEEP) {
        sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::handle_connection;

    use async_std::{
        io::{Error, Read, Write},
    };
    use futures::task::{Context, Poll};
    use std::cmp::min;
    use std::fs;
    use std::pin::Pin;

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];

        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;

        let expected_contents = fs::read_to_string("hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);

        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Unpin for MockTcpStream {}

    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);

            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }
}
