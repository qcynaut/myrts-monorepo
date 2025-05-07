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

use crate::error::{OtherError, Result};

use super::extractor::Extractor;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
    sync::Arc,
};

/// The state of the application.
#[derive(Default)]
pub struct State {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl State {
    /// Create a new `State`.
    #[inline]
    pub fn new() -> State {
        State {
            map: HashMap::default(),
        }
    }

    /// Insert an item to the state.
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(val));
    }

    /// Get a reference to an item of a given type.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>())
    }
}

/// State wrapper for sharable state.
pub struct Data<T: ?Sized + Send + Sync>(Arc<T>);

impl<T: Send + Sync> Data<T> {
    /// Create a new `Data`.
    pub fn new(val: T) -> Self {
        Self(Arc::new(val))
    }

    /// Get inner `Arc<T>`.
    pub fn get_ref(&self) -> Arc<T> {
        self.0.clone()
    }
}

impl<T: ?Sized + Send + Sync> Clone for Data<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized + Send + Sync> Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized + Send + Sync + 'static> Extractor for Data<T> {
    fn extract(sess: &super::session::Session) -> Result<Self> {
        match sess.get::<Data<T>>() {
            Some(v) => Ok(v.clone()),
            None => {
                Err(OtherError::String(format!("{} not found", std::any::type_name::<T>())).into())
            }
        }
    }
}
