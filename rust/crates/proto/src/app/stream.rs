/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use super::{extractor::Extractor, session::Session};
use crate::error::{OtherError, Result};
use futures_util::{Future, Sink, SinkExt, StreamExt};
use serde::Serialize;
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

/// The reader extension trait.
trait ReaderExt: Send + Sync + 'static {
    /// Read a message.
    fn read(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'static>>;
}

/// The writer extension trait.
trait WriterExt: Send + Sync + 'static {
    /// Write a message.
    fn write(
        &self,
        data: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>>;

    /// Disconnect the writer.
    fn disconnect(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>>;
}

/// Reader holds the stream reader.
struct Reader<T>(Arc<RwLock<T>>);

impl<T, E> Reader<T>
where
    T: StreamExt<Item = std::result::Result<Message, E>> + Unpin + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    /// Create a new `Reader`.
    fn new(t: T) -> Self {
        Self(Arc::new(RwLock::new(t)))
    }

    /// Read a message.
    async fn receive(&self) -> Result<String> {
        let data = {
            let mut guard = self.0.write().await;
            guard.next().await
        };
        log::debug!("Received: {:?}", data);
        match data {
            Some(Ok(data)) => Ok(data
                .into_text()
                .map_err(|e| crate::error::Error::Processing(e.to_string()))?),
            Some(Err(e)) => Err(crate::error::Error::Connection(e.to_string())),
            None => Err(crate::error::Error::Connection("No data".to_string())),
        }
    }

    /// Clone the reader.
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, E> ReaderExt for Reader<T>
where
    T: StreamExt<Item = std::result::Result<Message, E>> + Unpin + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn read(&self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'static>> {
        let reader = self.clone();
        Box::pin(async move { reader.receive().await })
    }
}

/// Writer holds the stream writer.
struct Writer<T>(Arc<RwLock<T>>);

impl<T, E> Writer<T>
where
    T: SinkExt<Message, Error = E> + Unpin + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    /// Create a new `Writer`.
    fn new(t: T) -> Self {
        Self(Arc::new(RwLock::new(t)))
    }

    /// Send a message.
    async fn send(&self, msg: String) -> Result<()> {
        {
            log::debug!("Sending: {:?}", msg);
            let mut guard = self.0.write().await;
            guard
                .send(Message::Text(msg))
                .await
                .map_err(|e| crate::error::Error::Connection(e.to_string()))?;
        }
        Ok(())
    }

    /// Disconnect the writer.
    async fn disconnect(&self) -> Result<()> {
        {
            let mut guard = self.0.write().await;
            guard
                .close()
                .await
                .map_err(|e| crate::error::Error::Connection(e.to_string()))?;
        }
        Ok(())
    }

    /// Clone the writer.
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, E> WriterExt for Writer<T>
where
    T: Sink<Message, Error = E> + Unpin + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn write(
        &self,
        data: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>> {
        let writer = self.clone();
        let data = data.to_string();
        Box::pin(async move { writer.send(data).await })
    }

    fn disconnect(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>> {
        let writer = self.clone();
        Box::pin(async move { writer.disconnect().await })
    }
}

/// The message stream.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct StreamMessage {
    event: String,
    data: String,
}

impl StreamMessage {
    /// Create a new `StreamMessage`.
    pub fn new(event: String, data: String) -> Self {
        Self { event, data }
    }

    /// Event name.
    pub fn event(&self) -> &str {
        &self.event
    }

    /// Data.
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Serialize the message.
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| crate::error::Error::Processing(e.to_string()))
    }

    /// Deserialize the message.
    pub fn deserialize(data: &str) -> Result<Self> {
        serde_json::from_str(data).map_err(|e| crate::error::Error::Processing(e.to_string()))
    }
}

/// Stream reader and writer.
pub struct Stream {
    id: String,
    reader: Arc<Box<dyn ReaderExt>>,
    writer: Arc<Box<dyn WriterExt>>,
}

impl std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream").field("id", &self.id).finish()
    }
}

impl Clone for Stream {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            reader: self.reader.clone(),
            writer: self.writer.clone(),
        }
    }
}

impl Extractor for Stream {
    fn extract(sess: &Session) -> Result<Self> {
        match sess.get::<Stream>() {
            Some(s) => Ok(s.clone()),
            None => Err(OtherError::String("Missing stream".to_string()).into()),
        }
    }
}

impl Stream {
    /// Create a new `Stream`.
    pub fn new<T, E>(s: T) -> Self
    where
        T: Sink<Message, Error = E>
            + StreamExt<Item = std::result::Result<Message, E>>
            + Unpin
            + Send
            + Sync
            + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        let (sink, stream) = s.split();
        let reader = Reader::new(stream);
        let writer = Writer::new(sink);
        let id: [u8; 16] = rand::random();
        Self {
            id: hex::encode(id),
            reader: Arc::new(Box::new(reader)),
            writer: Arc::new(Box::new(writer)),
        }
    }

    /// Read a message.
    pub(crate) async fn read(&self) -> Result<StreamMessage> {
        let res = self.reader.read().await?;
        Ok(StreamMessage::deserialize(&res)?)
    }

    /// Write a message.
    pub async fn write<T: Serialize>(&self, event: &str, data: T) -> Result<()> {
        let data = serde_json::to_string(&data)?;
        let msg = StreamMessage::new(event.to_string(), data);
        self.writer.write(&msg.serialize()?).await?;
        Ok(())
    }

    /// Disconnect the stream.
    pub async fn disconnect(&self) -> Result<()> {
        self.writer.disconnect().await?;
        Ok(())
    }

    /// Get id.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Collection of streams.
pub struct Streams {
    streams: Arc<RwLock<HashMap<String, Arc<Stream>>>>,
}

impl Clone for Streams {
    fn clone(&self) -> Self {
        Self {
            streams: self.streams.clone(),
        }
    }
}

impl Extractor for Streams {
    fn extract(sess: &Session) -> Result<Self> {
        match sess.get::<Streams>().cloned() {
            Some(s) => Ok(s),
            None => Err(OtherError::String("Missing streams".to_string()).into()),
        }
    }
}

impl Streams {
    /// Create a new `Streams`.
    pub fn new() -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a stream.
    pub async fn add(&self, stream: Arc<Stream>) {
        self.streams
            .write()
            .await
            .insert(stream.id().to_string(), stream);
    }

    /// Remove a stream.
    pub async fn remove(&self, id: &str) {
        self.streams.write().await.remove(id);
    }

    /// Send message to all streams.
    pub async fn send_all<T: Serialize + Clone>(&self, event: &str, msg: T) -> Result<()> {
        for stream in self.streams.read().await.values() {
            stream.write(event, msg.clone()).await?;
        }
        Ok(())
    }

    /// Send message to a specific stream.
    pub async fn send<T: Serialize>(&self, id: &str, event: &str, msg: T) -> Result<()> {
        if let Some(stream) = self.streams.read().await.get(id) {
            stream.write(event, msg).await?;
        }
        Ok(())
    }

    /// Clear streams.
    pub(crate) async fn clear(&self) {
        self.streams.write().await.clear();
    }
}
