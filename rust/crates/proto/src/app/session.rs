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

use super::{extractor::Extractor, state::State, stream::Streams, Stream};
use crate::error::{OtherError, Result};
use serde::Deserialize;
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Msg is an inbound message.
#[derive(Clone)]
pub struct Msg {
    data: String,
}

impl Msg {
    /// Get message data.
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl<'a> Msg {
    /// Deserialize message.
    pub fn deserialize<T: Deserialize<'a>>(&'a self) -> Result<T> {
        serde_json::from_str::<T>(&self.data).map_err(Into::into)
    }
}

impl Extractor for Msg {
    fn extract(sess: &Session) -> Result<Self> {
        match sess.get::<Msg>() {
            Some(v) => Ok(v.clone()),
            None => Err(OtherError::String("Missing message".to_string()).into()),
        }
    }
}

/// MsgData is an inbound message.
pub struct MsgData<T> {
    t: T,
}

impl<T> MsgData<T> {
    /// Take message data.
    pub fn into_inner(self) -> T {
        self.t
    }
}

impl<T> Extractor for MsgData<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn extract(sess: &Session) -> Result<Self> {
        let msg = Msg::extract(sess)?;
        Ok(MsgData {
            t: msg.deserialize()?,
        })
    }
}

impl<T> Deref for MsgData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for MsgData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

/// Session store data for each service.
pub struct Session {
    state: Arc<State>,
    msg: Arc<dyn Any + Send + Sync + 'static>,
    stream: Arc<dyn Any + Send + Sync + 'static>,
    streams: Arc<dyn Any + Send + Sync + 'static>,
}

impl Session {
    /// Create a new `Session`.
    pub fn new(state: Arc<State>, data: String, stream: Arc<Stream>, streams: Streams) -> Self {
        Self {
            state,
            msg: Arc::new(Msg { data }),
            stream: stream,
            streams: Arc::new(streams),
        }
    }

    /// Get session state.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == TypeId::of::<Msg>() {
            self.msg.downcast_ref::<T>()
        } else if TypeId::of::<T>() == TypeId::of::<Stream>() {
            self.stream.downcast_ref::<T>()
        } else if TypeId::of::<T>() == TypeId::of::<Streams>() {
            self.streams.downcast_ref::<T>()
        } else {
            self.state.get::<T>()
        }
    }
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            msg: self.msg.clone(),
            stream: self.stream.clone(),
            streams: self.streams.clone(),
        }
    }
}
