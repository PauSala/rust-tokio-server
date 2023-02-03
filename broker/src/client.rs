use tokio::net::{ToSocketAddrs, TcpStream};

use crate::{Connection, Result};

/// Established connection with the server.
///
/// Backed by a single `TcpStream`, `Client` provides basic network client
///
/// Requests are issued using the various methods of `Client`.

pub struct Client {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    ///
    /// When `Listener` receives an inbound connection, the `TcpStream` is
    /// passed to `Connection::new`, which initializes the associated buffers.
    /// `Connection` allows the handler to operate at the "frame" level and keep
    /// the byte level protocol parsing details encapsulated in `Connection`.
    connection: Connection,
}


/// Establish a connection with the Server located at `addr`.
///
/// `addr` may be any type that can be asynchronously converted to a
/// `SocketAddr`. This includes `SocketAddr` and strings. The `ToSocketAddrs`
/// trait is the Tokio version and not the `std` version.
pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
    // The `addr` argument is passed directly to `TcpStream::connect`. This
    // performs any asynchronous DNS lookup and attempts to establish the TCP
    // connection. An error at either step returns an error, which is then
    // bubbled up to the caller of `mini_redis` connect.
    let socket = TcpStream::connect(addr).await?;

    // Initialize the connection state. This allocates read/write buffers to
    // perform redis protocol frame parsing.
    let connection = Connection::new(socket);

    Ok(Client { connection })
}

impl Client {
    pub async fn write_something(&mut self, val: &String) -> Result<()>{
        self.connection.write_string(val).await?;
        Ok(())
    }

    pub async fn read_response(&mut self) -> Result<()>{
        self.connection.read_string().await?;
        Ok(())
    }
}
