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

use self::schedule_state::ScheduleState;
use audio::{audio::AudioPlayer, decoder::Decoder};
use proto::app::Stream;
use proto_db::ProtoDatabase;
use rtc::RTCConsumer;
use std::sync::Arc;
use tokio::sync::RwLock;
use utils::files::ApiAssets;

mod schedule_state;

/// ClientState holds the state of the client.
#[derive(Clone)]
pub struct ClientState {
    player: Arc<AudioPlayer>,
    schedule_state: ScheduleState,
    streaming: Arc<RwLock<Option<Arc<RTCConsumer>>>>,
    streaming_volume: Arc<RwLock<f32>>,
}

impl ClientState {
    /// New client state.
    pub async fn new(assets: ApiAssets, db: ProtoDatabase) -> Self {
        let player = Arc::new(AudioPlayer::new());
        let schedule_state = ScheduleState::new(db, assets, player.clone());
        schedule_state.run().await;
        Self {
            player,
            schedule_state,
            streaming: Arc::new(RwLock::new(None)),
            streaming_volume: Arc::new(RwLock::new(1.0)),
        }
    }

    /// Update schedule.
    pub fn update_schedule(&self) {
        log::debug!("update schedule");
        self.schedule_state.update();
    }

    /// Block schedule.
    pub async fn block_schedule(&self) {
        log::debug!("block schedule");
        self.schedule_state.block().await;
    }

    /// Unblock schedule.
    pub async fn unblock_schedule(&self) {
        log::debug!("unblock schedule");
        self.schedule_state.unblock().await;
    }

    /// Set streaming volume.
    pub async fn set_streaming_volume(&self, volume: f32) {
        log::debug!("set streaming volume: {}", volume);
        *self.streaming_volume.write().await = volume;
    }

    /// Start streaming.
    pub async fn start_streaming(&self, stream: Stream, offer: String) {
        log::debug!("start streaming");
        {
            let may_stream = self.streaming.write().await.take();
            if let Some(consumer) = may_stream {
                consumer.disconnect().await;
            }
        }
        self.set_streaming_volume(1.0).await;
        let consumer = if let Ok(consumer) = RTCConsumer::new(stream).await {
            Arc::new(consumer)
        } else {
            return;
        };
        self.block_schedule().await;
        let consumer_clone = consumer.clone();
        let volume = self.streaming_volume.clone();
        let player = self.player.clone();
        consumer.on_track(Box::new(move |track| {
            let player = player.clone();
            let consumer_clone = consumer_clone.clone();
            let volume = volume.clone();
            Box::pin(async move {
                let mut decoder = Decoder::new().unwrap();
                let mut current_colume = 1.0;
                loop {
                    {
                        let lock = volume.read().await;
                        if *lock != current_colume {
                            current_colume = *lock;
                            player.set_volume(current_colume);
                        }
                    }
                    if let Ok((p, _)) = track.read_rtp().await {
                        log::debug!(
                            "Playing {} bytes with {} volume",
                            p.payload.len(),
                            current_colume
                        );
                        for frame in decoder.decode(&p.payload) {
                            player.append(frame);
                        }
                        player.play();
                    } else {
                        if !consumer_clone.connected() {
                            log::debug!("consumer disconnected");
                            break;
                        }
                    }
                }
            })
        }));
        if let Err(e) = consumer.add_offer(offer).await {
            log::error!("failed to add offer: {:?}", e);
            self.unblock_schedule().await;
            return;
        }
        if let Err(e) = consumer.answer().await {
            log::error!("failed to answer: {:?}", e);
            self.unblock_schedule().await;
            return;
        }
        *self.streaming.write().await = Some(consumer);
    }

    /// Add ices.
    pub async fn add_ices(&self, ices: String) {
        log::debug!("add ices");
        let lock = self.streaming.read().await;
        if let Some(consumer) = &*lock {
            if let Err(e) = consumer.add_ices(ices).await {
                log::error!("failed to add ices: {:?}", e);
            }
        }
    }

    /// Close streaming.
    pub async fn close_streaming(&self) {
        log::debug!("close streaming");
        self.unblock_schedule().await;
        let streaming = self.streaming.write().await.take();
        if let Some(consumer) = streaming {
            consumer.disconnect().await;
        }
    }
}
