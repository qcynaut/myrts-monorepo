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

use self::{session::Session, state::State};
pub use service::{AppService, EventServiceFactory};
pub use session::{Msg, MsgData};
pub use state::Data;
use std::sync::Arc;
pub use stream::{Stream, Streams};
use tokio::task::JoinHandle;

mod extractor;
mod service;
mod session;
mod state;
mod stream;

/// The top-level builder for myrts proto.
pub struct App {
    state: State,
    services: AppService,
    streams: Streams,
}

impl App {
    /// Create a new `App`.
    pub fn new() -> Self {
        Self {
            state: State::new(),
            services: AppService::new(),
            streams: Streams::new(),
        }
    }

    /// Add application state data.
    pub fn add_state<T: Send + Sync + 'static>(mut self, state: T) -> Self {
        self.state.insert(state);
        self
    }

    /// Get the streams.
    pub fn get(&self) -> Streams {
        self.streams.clone()
    }

    /// Add service to the application.
    pub fn service<T: EventServiceFactory>(mut self, service: T) -> Self {
        service.register(&mut self.services);
        self
    }

    /// Build the application.
    pub fn build(self) -> Service {
        Service {
            state: Arc::new(self.state),
            services: Arc::new(self.services),
            streams: self.streams,
        }
    }
}

/// Service is an application service.
pub struct Service {
    state: Arc<State>,
    services: Arc<AppService>,
    streams: Streams,
}

impl Service {
    /// Handle incoming events.
    pub(crate) async fn handle(&self, stream: Arc<Stream>) -> JoinHandle<()> {
        self.streams.add(stream.clone()).await;
        let state = self.state.clone();
        let services = self.services.clone();
        let streams = self.streams.clone();
        tokio::spawn(async move {
            let state = state.clone();
            let services = services.clone();
            let stream = stream.clone();
            let streams = streams.clone();
            let mut h: Vec<JoinHandle<()>> = vec![];
            match services.handle(
                "start".to_owned(),
                Session::new(
                    state.clone(),
                    String::new(),
                    stream.clone(),
                    streams.clone(),
                ),
            ) {
                Some(f) => h.push(tokio::spawn(async move {
                    match f.await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("{}", e);
                        }
                    }
                })),
                None => {
                    log::warn!("start service not found");
                }
            }
            loop {
                log::debug!("Waiting for message");
                let msg = stream.read().await;
                match msg {
                    Ok(msg) => {
                        let evt = msg.event();
                        log::debug!("Received event: {}", evt);
                        let data = msg.data();
                        let sess = Session::new(
                            state.clone(),
                            data.to_owned(),
                            stream.clone(),
                            streams.clone(),
                        );
                        match services.handle(evt.to_owned(), sess) {
                            Some(f) => h.push(tokio::spawn(async move {
                                match f.await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        log::error!("{}", e);
                                    }
                                }
                            })),
                            None => {
                                log::error!("{} service not found", evt);
                            }
                        }
                    }
                    Err(e) => match e {
                        crate::error::Error::Connection(e) => {
                            log::error!("{}", e);
                            match services.handle(
                                "end".to_owned(),
                                Session::new(
                                    state.clone(),
                                    String::new(),
                                    stream.clone(),
                                    streams.clone(),
                                ),
                            ) {
                                Some(f) => h.push(tokio::spawn(async move {
                                    match f.await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            log::error!("{}", e);
                                        }
                                    }
                                })),
                                None => {
                                    log::error!("end service not found");
                                }
                            }
                            break;
                        }
                        _ => {
                            log::error!("{}", e);
                        }
                    },
                }
            }

            for handle in h {
                if handle.is_finished() {
                    continue;
                }
                // await for 5 seconds, if not finished, kill the task
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                if !handle.is_finished() {
                    handle.abort();
                }
            }
        })
    }

    /// Clear stream.
    pub(crate) async fn clear(&self) {
        self.streams.clear().await;
    }
}
