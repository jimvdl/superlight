use crate::message::Message;
use serde::Serialize;
use std::net::TcpStream;
use tungstenite::{client::IntoClientRequest, stream::MaybeTlsStream, Message as TungMessage, WebSocket};

#[derive(Debug)]
pub struct Socket {
    connection: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl Socket {
    pub fn from_ws(path: &str) -> anyhow::Result<Self> {
        let mut request = path.into_client_request()?;
        let headers = request.headers_mut();
        headers.insert("Sec-WebSocket-Protocol", "json".parse()?);

        let (socket, _) = tungstenite::connect(request)?;

        Ok(Self { connection: socket })
    }

    pub fn send<D>(&mut self, data: D) -> anyhow::Result<()>
    where
        D: Serialize,
    {
        let serialized = serde_json::to_string(&data)?;
        self.connection.write_message(TungMessage::Text(serialized))?;

        Ok(())
    }

    pub fn receive(&mut self) -> anyhow::Result<Message> {
        let message = self.connection.read_message().expect("incoming message corrupted");
        let message: Message = serde_json::from_str(&message.into_text()?)?;

        Ok(message)
    }
}
