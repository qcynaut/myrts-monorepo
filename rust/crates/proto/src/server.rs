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

use crate::{
    app::{App, Stream},
    error::{Error, Result},
};
use std::sync::Arc;
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_tungstenite::accept_async;

/// MyRTS server builder.
pub struct Server {
    app: App,
    port: u16,
    handle: Vec<JoinHandle<()>>,
}

impl Server {
    /// Create a new `Server`.
    pub fn new(app: App, port: u16) -> Self {
        Self {
            app,
            port,
            handle: vec![],
        }
    }

    /// Run the application.
    pub async fn run(mut self) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        let service = self.app.build();
        while let Ok((stream, _)) = listener.accept().await {
            match accept_async(stream).await {
                Ok(stream) => {
                    let stream = Arc::new(Stream::new(stream));
                    self.handle.push(service.handle(stream).await);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }
        service.clear().await;
        Ok(())
    }
}
