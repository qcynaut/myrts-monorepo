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

use app::Stream;
pub use proto_macro::service;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub mod app;
pub mod client;
pub mod error;
pub mod server;

/// WsState.
/// The state of the websocket.
#[derive(Clone, Debug)]
pub struct WsState {
    avs: Arc<RwLock<HashMap<String, Stream>>>,
    user: Arc<RwLock<HashMap<i32, Stream>>>,
    avs_map: Arc<RwLock<HashMap<String, String>>>,
    user_map: Arc<RwLock<HashMap<String, i32>>>,
    streaming: Arc<RwLock<HashMap<i32, Vec<String>>>>,
}

impl Default for WsState {
    fn default() -> Self {
        Self {
            avs: Arc::new(RwLock::new(HashMap::new())),
            user: Arc::new(RwLock::new(HashMap::new())),
            avs_map: Arc::new(RwLock::new(HashMap::new())),
            user_map: Arc::new(RwLock::new(HashMap::new())),
            streaming: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl WsState {
    /// Get avs.
    pub async fn avs(&self, stream_id: String) -> Option<Stream> {
        if let Some(avs_id) = self.avs_map.read().await.get(&stream_id) {
            self.avs.read().await.get(avs_id).cloned()
        } else {
            None
        }
    }

    /// Get avs id.
    pub async fn avs_id(&self, stream_id: String) -> Option<String> {
        self.avs_map.read().await.get(&stream_id).cloned()
    }

    /// Get avs by id.
    pub async fn avs_by_id(&self, avs_id: String) -> Option<Stream> {
        self.avs.read().await.get(&avs_id).cloned()
    }

    /// Get user.
    pub async fn user(&self, stream_id: String) -> Option<Stream> {
        if let Some(user_id) = self.user_map.read().await.get(&stream_id) {
            self.user.read().await.get(user_id).cloned()
        } else {
            None
        }
    }

    /// Get user id.
    pub async fn user_id(&self, stream_id: String) -> Option<i32> {
        self.user_map.read().await.get(&stream_id).cloned()
    }

    /// Get user by id.
    pub async fn user_by_id(&self, user_id: i32) -> Option<Stream> {
        self.user.read().await.get(&user_id).cloned()
    }

    /// Set avs.
    pub async fn set_avs(&self, avs_id: String, avs: Stream) {
        self.avs_map
            .write()
            .await
            .insert(avs.id().to_owned(), avs_id.clone());
        self.avs.write().await.insert(avs_id, avs);
    }

    /// Set user.
    pub async fn set_user(&self, user_id: i32, user: Stream) {
        self.user_map
            .write()
            .await
            .insert(user.id().to_owned(), user_id);
        self.user.write().await.insert(user_id, user);
    }

    /// Remove avs.
    pub async fn remove_avs(&self, stream_id: String) {
        if let Some(avs_id) = self.avs_map.write().await.remove(&stream_id) {
            self.avs.write().await.remove(&avs_id);
        }
    }

    /// Remove user.
    pub async fn remove_user(&self, stream_id: String) {
        if let Some(user_id) = self.user_map.write().await.remove(&stream_id) {
            self.user.write().await.remove(&user_id);
        }
    }

    /// Get streaming.
    pub fn streaming(&self) -> Arc<RwLock<HashMap<i32, Vec<String>>>> {
        self.streaming.clone()
    }

    /// Get on going streaming by the given unique ids.
    pub async fn get_ongoing_from_unique(
        &self,
        unique_ids: Vec<String>,
    ) -> HashMap<i32, Vec<String>> {
        let mut res = HashMap::new();
        let lock = self.streaming.read().await;
        for unique_id in unique_ids {
            for (user_id, avs_ids) in lock.iter() {
                if avs_ids.contains(&unique_id) {
                    res.insert(*user_id, avs_ids.clone());
                }
            }
        }

        res
    }

    /// Get all on going streaming.
    pub async fn get_ongoing(&self) -> HashMap<i32, Vec<String>> {
        let lock = self.streaming.read().await;
        let mut res = HashMap::new();
        for (user_id, avs_ids) in lock.iter() {
            res.insert(*user_id, avs_ids.clone());
        }

        res
    }
}
