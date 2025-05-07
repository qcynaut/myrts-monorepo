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
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::RwLock;
use types::proto::{Answer, Ices, Offer};
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors,
        media_engine::{MediaEngine, MIME_TYPE_OPUS},
        APIBuilder,
    },
    ice_transport::{
        ice_candidate::RTCIceCandidateInit, ice_credential_type::RTCIceCredentialType,
        ice_gatherer_state::RTCIceGathererState, ice_server::RTCIceServer,
    },
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, RTCPeerConnection,
    },
    rtp_transceiver::rtp_codec::{RTCRtpCodecCapability, RTCRtpCodecParameters, RTPCodecType},
    track::{
        track_local::{track_local_static_rtp::TrackLocalStaticRTP, TrackLocalWriter},
        track_remote::TrackRemote,
    },
};

/// RTCError.
#[derive(Debug, thiserror::Error)]
pub enum RTCError {
    #[error("error: {0}")]
    Rtc(#[from] webrtc::Error),
    #[error("error: {0}")]
    Serde(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, RTCError>;

fn capability() -> RTCRtpCodecCapability {
    RTCRtpCodecCapability {
        mime_type: MIME_TYPE_OPUS.to_owned(),
        clock_rate: 48000,
        channels: 2,
        sdp_fmtp_line: "".to_owned(),
        rtcp_feedback: vec![],
    }
}

async fn new_rtc(receive_audio: bool) -> Result<RTCPeerConnection> {
    let mut m = MediaEngine::default();
    m.register_codec(
        RTCRtpCodecParameters {
            capability: capability(),
            payload_type: 111,
            ..Default::default()
        },
        RTPCodecType::Audio,
    )?;
    let mut r = Registry::new();
    r = register_default_interceptors(r, &mut m)?;
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(r)
        .build();
    let peer = api
        .new_peer_connection(RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                urls: vec!["turn:api.brandio.id:3478".to_owned()],
                username: "brandio".to_owned(),
                credential: "brandio".to_owned(),
                credential_type: RTCIceCredentialType::Password,
            }],
            ..Default::default()
        })
        .await?;
    if receive_audio {
        peer.add_transceiver_from_kind(RTPCodecType::Audio, None)
            .await?;
    }
    Ok(peer)
}

/// RTCProvider.
/// This kind of rtc used on the server for handling user connection that will be forwaded to the consumer.
#[derive(Clone)]
pub struct RTCProvider {
    stream: Stream,
    local_track: Arc<TrackLocalStaticRTP>,
    peer: Arc<RTCPeerConnection>,
}

impl RTCProvider {
    /// Create new RTCProvider.
    pub async fn new(stream: Stream) -> Result<Self> {
        let peer = Arc::new(new_rtc(true).await?);
        let ices: Arc<RwLock<Vec<RTCIceCandidateInit>>> = Arc::new(RwLock::new(vec![]));
        let ices_clone = ices.clone();
        peer.on_ice_candidate(Box::new(move |c| {
            let ices = ices_clone.clone();
            Box::pin(async move {
                if let Some(c) = c {
                    if let Ok(c) = c.to_json() {
                        let mut ices = ices.write().await;
                        ices.push(c);
                    }
                }
            })
        }));
        let stream_clone = stream.clone();
        let ices_clone = ices.clone();
        peer.on_ice_gathering_state_change(Box::new(move |state| {
            let stream = stream_clone.clone();
            let ices = ices_clone.clone();
            Box::pin(async move {
                if state == RTCIceGathererState::Complete {
                    let lock = ices.read().await;
                    let ices = lock.clone();
                    let json = serde_json::to_string(&ices).unwrap();
                    let _ = stream.write("ices", Ices { ices: json }).await;
                }
            })
        }));
        let local_track = Arc::new(TrackLocalStaticRTP::new(
            capability(),
            "audio".to_owned(),
            "webrtc-rs".to_owned(),
        ));
        let peer_clone = peer.clone();
        let local_track_clone = local_track.clone();
        peer.on_track(Box::new(move |track, _, _| {
            let local_track = local_track_clone.clone();
            let peer = peer_clone.clone();

            Box::pin(async move {
                loop {
                    if let Ok((p, _)) = track.read_rtp().await {
                        let _ = local_track.write_rtp(&p).await;
                    } else {
                        if peer.connection_state() != RTCPeerConnectionState::Connected {
                            log::debug!("peer connection state: {:?}", peer.connection_state());
                            break;
                        }
                    }
                }
            })
        }));

        Ok(Self {
            stream,
            local_track,
            peer,
        })
    }

    /// Add offer.
    pub async fn add_offer(&self, offer: String) -> Result<()> {
        let offer: RTCSessionDescription = serde_json::from_str(&offer)?;
        self.peer.set_remote_description(offer).await?;
        Ok(())
    }

    /// Create answer.
    pub async fn answer(&self) -> Result<()> {
        let answer = self.peer.create_answer(None).await?;
        self.peer.set_local_description(answer.clone()).await?;
        let answer = serde_json::to_string(&answer)?;
        let _ = self.stream.write("answer", Answer { answer }).await;
        Ok(())
    }

    /// Add ices.
    pub async fn add_ices(&self, ices: String) -> Result<()> {
        let mut tick = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            tick.tick().await;
            if self.peer.remote_description().await.is_some() {
                break;
            }
        }
        let ices: Vec<RTCIceCandidateInit> = serde_json::from_str(&ices)?;
        for ice in ices {
            let _ = self.peer.add_ice_candidate(ice).await;
        }
        Ok(())
    }

    /// Get local track.
    pub fn local_track(&self) -> Arc<TrackLocalStaticRTP> {
        self.local_track.clone()
    }

    /// Disconnect.
    pub async fn disconnect(&self) {
        if self.peer.connection_state() == RTCPeerConnectionState::Connected {
            let _ = self.peer.close().await;
        }
    }
}

/// RTCForwader.
/// This kind of rtc used on the server for handling user provided stream to the consumer.
#[derive(Clone)]
pub struct RTCForwader {
    stream: Stream,
    peer: Arc<RTCPeerConnection>,
}

impl RTCForwader {
    /// Create new RTCForwader.
    pub async fn new(stream: Stream, track: Arc<TrackLocalStaticRTP>) -> Result<Self> {
        let peer = Arc::new(new_rtc(false).await?);
        let ices: Arc<RwLock<Vec<RTCIceCandidateInit>>> = Arc::new(RwLock::new(vec![]));
        let ices_clone = ices.clone();
        peer.on_ice_candidate(Box::new(move |c| {
            let ices = ices_clone.clone();
            Box::pin(async move {
                if let Some(c) = c {
                    if let Ok(c) = c.to_json() {
                        let mut ices = ices.write().await;
                        ices.push(c);
                    }
                }
            })
        }));
        let stream_clone = stream.clone();
        let ices_clone = ices.clone();
        peer.on_ice_gathering_state_change(Box::new(move |state| {
            let stream = stream_clone.clone();
            let ices = ices_clone.clone();
            Box::pin(async move {
                if state == RTCIceGathererState::Complete {
                    let lock = ices.read().await;
                    let ices = lock.clone();
                    let json = serde_json::to_string(&ices).unwrap();
                    let _ = stream.write("ices", Ices { ices: json }).await;
                }
            })
        }));
        peer.add_track(track).await?;
        Ok(Self { stream, peer })
    }

    /// On failed.
    pub fn on_failed(
        &self,
        f: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
    ) {
        let f = Arc::new(f);
        self.peer
            .on_peer_connection_state_change(Box::new(move |state| {
                let f = f.clone();
                if state == RTCPeerConnectionState::Failed {
                    Box::pin(async move {
                        f().await;
                    })
                } else {
                    Box::pin(async move {})
                }
            }));
    }

    /// Create offer.
    pub async fn create_offer(&self) -> Result<()> {
        let offer = self.peer.create_offer(None).await?;
        self.peer.set_local_description(offer.clone()).await?;
        let offer = serde_json::to_string(&offer)?;
        let _ = self
            .stream
            .write(
                "offer",
                Offer {
                    offer,
                    target: vec![],
                },
            )
            .await;
        Ok(())
    }

    /// Add answer.
    pub async fn add_answer(&self, answer: String) -> Result<()> {
        let answer: RTCSessionDescription = serde_json::from_str(&answer)?;
        self.peer.set_remote_description(answer).await?;
        Ok(())
    }

    /// Get stream.
    pub fn stream(&self) -> Stream {
        self.stream.clone()
    }

    /// Add ices.
    pub async fn add_ices(&self, ices: String) -> Result<()> {
        let mut tick = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            tick.tick().await;
            if self.peer.remote_description().await.is_some() {
                break;
            }
        }
        let ices: Vec<RTCIceCandidateInit> = serde_json::from_str(&ices)?;
        for ice in ices {
            let _ = self.peer.add_ice_candidate(ice).await;
        }
        Ok(())
    }

    /// Disconnect.
    pub async fn disconnect(&self) {
        if self.peer.connection_state() == RTCPeerConnectionState::Connected {
            let _ = self.stream.write("stream:close", "").await;
            let _ = self.peer.close().await;
        }
    }
}

/// RTCConsumer.
/// This kind of rtc used on the consumer for handling stream from the server.
#[derive(Clone)]
pub struct RTCConsumer {
    stream: Stream,
    peer: Arc<RTCPeerConnection>,
}

impl RTCConsumer {
    /// Create new RTCConsumer.
    pub async fn new(stream: Stream) -> Result<Self> {
        let peer = Arc::new(new_rtc(true).await?);
        let ices: Arc<RwLock<Vec<RTCIceCandidateInit>>> = Arc::new(RwLock::new(vec![]));
        let ices_clone = ices.clone();
        peer.on_ice_candidate(Box::new(move |c| {
            let ices = ices_clone.clone();
            Box::pin(async move {
                if let Some(c) = c {
                    if let Ok(c) = c.to_json() {
                        let mut ices = ices.write().await;
                        ices.push(c);
                    }
                }
            })
        }));
        let stream_clone = stream.clone();
        let ices_clone = ices.clone();
        peer.on_ice_gathering_state_change(Box::new(move |state| {
            let stream = stream_clone.clone();
            let ices = ices_clone.clone();
            Box::pin(async move {
                if state == RTCIceGathererState::Complete {
                    let lock = ices.read().await;
                    let ices = lock.clone();
                    let json = serde_json::to_string(&ices).unwrap();
                    let _ = stream.write("ices", Ices { ices: json }).await;
                }
            })
        }));
        Ok(Self { stream, peer })
    }

    /// On track.
    pub fn on_track(
        &self,
        f: Box<dyn Fn(Arc<TrackRemote>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
    ) {
        self.peer
            .on_track(Box::new(move |track, _, _| Box::pin(f(track))));
    }

    /// Add offer.
    pub async fn add_offer(&self, offer: String) -> Result<()> {
        let offer: RTCSessionDescription = serde_json::from_str(&offer)?;
        self.peer.set_remote_description(offer).await?;
        Ok(())
    }

    /// Create answer.
    pub async fn answer(&self) -> Result<()> {
        let answer = self.peer.create_answer(None).await?;
        self.peer.set_local_description(answer.clone()).await?;
        let answer = serde_json::to_string(&answer)?;
        let _ = self.stream.write("answer", Answer { answer }).await;
        Ok(())
    }

    /// Add ices.
    pub async fn add_ices(&self, ices: String) -> Result<()> {
        let mut tick = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            tick.tick().await;
            if self.peer.remote_description().await.is_some() {
                break;
            }
        }
        let ices: Vec<RTCIceCandidateInit> = serde_json::from_str(&ices)?;
        for ice in ices {
            let _ = self.peer.add_ice_candidate(ice).await;
        }
        Ok(())
    }

    /// Disconnect.
    pub async fn disconnect(&self) {
        if self.peer.connection_state() == RTCPeerConnectionState::Connected {
            let _ = self.peer.close().await;
        }
    }

    /// Check if connected.
    pub fn connected(&self) -> bool {
        self.peer.connection_state() == RTCPeerConnectionState::Connected
    }
}
