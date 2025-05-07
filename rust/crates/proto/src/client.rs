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
//!
//! MyRTS protcol definitions and implementation.
//!
//! This crate provides the definitions and implementations of the
//! MyRTS protocol.

use crate::{
    app::{App, Stream},
    error::{Error, Result},
};
use std::sync::Arc;
use tokio_tungstenite::connect_async;

/// MyRTS client builder.
pub struct Client {
    app: App,
    url: String,
}

impl Client {
    /// Create a new `Client`.
    pub fn new(app: App, url: &str) -> Self {
        Self {
            app,
            url: url.to_string(),
        }
    }

    /// Run the application.
    pub async fn run(self) -> Result<()> {
        let (stream, _) = connect_async(self.url)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        let service = self.app.build();
        let handle = service.handle(Arc::new(Stream::new(stream))).await;
        let _ = handle.await;
        Ok(())
    }
}
