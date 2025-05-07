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

use std::convert::Infallible;

/// The error type used in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Cannot process message: {0}")]
    Processing(String),
    #[error("Something went wrong: {0}")]
    Other(#[from] OtherError),
}

/// Other error.
#[derive(Debug, thiserror::Error)]
pub enum OtherError {
    #[error("{0}")]
    Infallible(#[from] Infallible),
    #[error("{0}")]
    String(String),
}

pub type Result<T> = core::result::Result<T, Error>;
