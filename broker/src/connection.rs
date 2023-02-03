use std::str::from_utf8;

use bytes::BytesMut;
use tokio::{io::{BufWriter, AsyncWriteExt, self, AsyncReadExt}, net::TcpStream};

#[derive(Debug)]
pub struct Connection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    stream: BufWriter<TcpStream>,

    // The buffer for reading frames.
    _buffer: BytesMut,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            _buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn write_string(&mut self, val: &String) -> io::Result<()>{

        self.stream.write_all(val.as_bytes()).await?;
        self.stream.flush().await
    }

    pub async fn read_string(&mut self) -> io::Result<[u8; 2048]>{
        let mut buf = [0; 2048];
        let readed = self.stream.read(&mut buf).await?;
        let s = match from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{} size:{} bytes", s, readed);
        self.stream.flush().await?;
       
        Ok(buf)
    }
}
