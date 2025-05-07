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

use proto::app::Stream;
use rtc::{RTCForwader, RTCProvider};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use types::proto::{Volume, WsErr};

/// Streaming.
#[derive(Clone)]
pub struct Streaming {
    provider: Arc<RTCProvider>,
    forwarders: Arc<RwLock<HashMap<String, Arc<RTCForwader>>>>,
}

impl Streaming {
    /// Create new Streaming.
    pub fn new(provider: Arc<RTCProvider>, forwarders: HashMap<String, Arc<RTCForwader>>) -> Self {
        Self {
            provider,
            forwarders: Arc::new(RwLock::new(forwarders)),
        }
    }

    /// Begin streaming.
    pub async fn begin(&self) {
        let lock = self.forwarders.read().await;
        for (id, forwarder) in lock.iter() {
            let fw = self.forwarders.clone();
            let id = id.to_owned();
            let track = self.provider.local_track();
            forwarder.on_failed(Box::new(move || {
                let fw = fw.clone();
                let id = id.clone();
                let track = track.clone();
                Box::pin(async move {
                    let mut lock = fw.write().await;
                    if let Some(forwarder) = lock.remove(&id) {
                        forwarder.disconnect().await;
                        let new_forwarder = RTCForwader::new(forwarder.stream(), track).await;
                        if let Ok(forwarder) = new_forwarder {
                            let forwarder = Arc::new(forwarder);
                            lock.insert(id, forwarder.clone());
                            let _ = forwarder.create_offer().await;
                        }
                    }
                })
            }));
            if let Err(e) = forwarder.create_offer().await {
                log::error!("failed to create offer: {:?}", e);
            }
        }
        if let Err(e) = self.provider.answer().await {
            log::error!("failed to create answer: {:?}", e);
        }
    }

    /// Add ices to provider.
    pub async fn add_ices_provider(&self, ices: String) {
        if let Err(e) = self.provider.add_ices(ices).await {
            log::error!("failed to add ices: {:?}", e);
        }
    }

    /// Add ices to forwarder.
    pub async fn add_ices_forwarder(&self, id: &str, ices: String) {
        let lock = self.forwarders.read().await;
        if let Some(forwarder) = lock.get(id) {
            if let Err(e) = forwarder.add_ices(ices).await {
                log::error!("failed to add ices: {:?}", e);
            }
        }
    }

    /// Add forwarder answer.
    pub async fn add_answer_forwarder(&self, id: &str, answer: String) {
        let lock = self.forwarders.read().await;
        if let Some(forwarder) = lock.get(id) {
            if let Err(e) = forwarder.add_answer(answer).await {
                log::error!("failed to add answer: {:?}", e);
            }
        }
    }

    /// Close forwarder.
    pub async fn close_forwarder(&self, id: &str) {
        let mut lock = self.forwarders.write().await;
        if let Some(forwarder) = lock.remove(id) {
            forwarder.disconnect().await;
        }
    }
}

/// StreamingState.
#[derive(Clone)]
pub struct StreamingState {
    streaming: Arc<RwLock<HashMap<i32, Arc<Streaming>>>>,
    avs_map: Arc<RwLock<HashMap<String, i32>>>,
    on_going: Arc<RwLock<HashMap<i32, Vec<String>>>>,
}

impl StreamingState {
    /// Create new StreamingState.
    pub fn new(on_going: Arc<RwLock<HashMap<i32, Vec<String>>>>) -> Self {
        Self {
            streaming: Arc::new(RwLock::new(HashMap::new())),
            avs_map: Arc::new(RwLock::new(HashMap::new())),
            on_going,
        }
    }

    /// Create new Streaming.
    pub async fn new_streaming(
        &self,
        stream: Stream,
        id: i32,
        offer: String,
        mut target: HashMap<String, Stream>,
    ) {
        {
            let avs_map = self.avs_map.read().await;
            // check if the target is not in the avs_map and filter them.
            for (target_id, _) in target.clone().iter() {
                if avs_map.contains_key(target_id) {
                    target.remove(target_id);
                }
            }
        }

        let provider = if let Ok(provider) = RTCProvider::new(stream.clone()).await {
            Arc::new(provider)
        } else {
            let _ = stream
                .write(
                    "offer:fail",
                    WsErr {
                        msg: "failed to create provider".to_owned(),
                    },
                )
                .await;
            return;
        };

        if let Err(e) = provider.add_offer(offer).await {
            log::error!("failed to add offer: {:?}", e);
            let _ = stream
                .write(
                    "offer:fail",
                    WsErr {
                        msg: "failed to add offer".to_owned(),
                    },
                )
                .await;
            return;
        }

        let track = provider.local_track();

        let mut forwarders = HashMap::new();
        let mut on_going = vec![];
        log::debug!("create forwarders for target: {:?}", target);
        for (target_id, avs) in target.iter() {
            let forwarder =
                if let Ok(forwarder) = RTCForwader::new(avs.clone(), track.clone()).await {
                    self.avs_map.write().await.insert(target_id.to_owned(), id);
                    on_going.push(target_id.to_owned());
                    Arc::new(forwarder)
                } else {
                    log::error!("failed to create forwarder for {}", target_id);
                    continue;
                };
            forwarders.insert(target_id.to_owned(), forwarder);
        }

        let streaming = Arc::new(Streaming::new(provider, forwarders));
        self.streaming.write().await.insert(id, streaming.clone());
        self.on_going.write().await.insert(id, on_going);
        streaming.begin().await;
    }

    /// Add ices to provider.
    pub async fn add_ices_provider(&self, id: i32, ices: String) {
        if let Some(streaming) = self.streaming.read().await.get(&id) {
            streaming.add_ices_provider(ices).await;
        }
    }

    /// Add ices to forwarder.
    pub async fn add_ices_forwarder(&self, id: &str, ices: String) {
        if let Some(stream_id) = self.avs_map.read().await.get(id) {
            if let Some(streaming) = self.streaming.read().await.get(stream_id) {
                streaming.add_ices_forwarder(id, ices).await;
            }
        }
    }

    /// Add forwarder answer.
    pub async fn add_answer_forwarder(&self, id: &str, answer: String) {
        if let Some(stream_id) = self.avs_map.read().await.get(id) {
            if let Some(streaming) = self.streaming.read().await.get(stream_id) {
                streaming.add_answer_forwarder(id, answer).await;
            }
        }
    }

    /// Set volume.
    pub async fn set_volume(&self, id: i32, volume: Volume) {
        if let Some(streaming) = self.streaming.read().await.get(&id) {
            for (_, forwarder) in streaming.forwarders.read().await.iter() {
                let stream = forwarder.stream();
                if let Err(e) = stream
                    .write(
                        "volume",
                        Volume {
                            volume: volume.volume.clone(),
                        },
                    )
                    .await
                {
                    log::error!("failed to set volume: {:?}", e);
                }
            }
        }
    }

    /// Close streaming.
    pub async fn close_streaming(&self, id: i32) {
        if let Some(streaming) = self.streaming.write().await.remove(&id) {
            self.on_going.write().await.remove(&id);
            streaming.provider.disconnect().await;
            for (id, forwarder) in streaming.forwarders.read().await.iter() {
                forwarder.disconnect().await;
                self.avs_map.write().await.remove(id);
            }
        }
    }

    /// Close forwarder.
    pub async fn close_forwarder(&self, id: &str) {
        if let Some(stream_id) = self.avs_map.write().await.remove(id) {
            let mut lock = self.streaming.write().await;
            if let Some(streaming) = lock.get_mut(&stream_id) {
                let mut lock2 = self.on_going.write().await;
                if let Some(on_going) = lock2.get_mut(&stream_id) {
                    on_going.retain(|x| x != id);
                }
                streaming.close_forwarder(id).await;
            }
        }
    }
}
