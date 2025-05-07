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

use serde::{Deserialize, Serialize};

/// Authenticate.
/// This is the `authenticate` data sent from the client to the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct Authenticate {
    pub client_id: String,
    pub client_type: u8,
    pub client_description: String,
    pub client_address: String,
}

/// Scedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub sid: i32,
    pub name: String,
    pub days: Vec<Option<i32>>,
    pub record_url: String,
    pub kind: i32,
    pub weeks: Vec<Option<i32>>,
    pub dates: Vec<Option<i32>>,
    pub times: Vec<Option<String>>,
    pub month: Option<i32>,
    pub year: Option<i32>,
    pub volume: Option<f32>,
}

/// Sync.
/// This is the `sync` data sent from the server to the client.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sync {
    pub add: Vec<Schedule>,
    pub remove: Vec<i32>,
}

/// SyncReq.
/// This is the `sync` data sent from the client to the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReq {
    pub local: Vec<i32>,
}

/// Turn.
/// This is the `turn` data sent from the server to the client.
#[derive(Debug, Serialize, Deserialize)]
pub struct Turn {
    pub url: String,
    pub username: String,
    pub password: String,
}

/// CmdRequest.
/// This is the `cmd_request` data sent from the client to the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct CmdRequest {
    pub command: String,
    pub sender: i32,
    pub target: String,
}

/// CmdResponse.
/// This is the `cmd_response` data sent from the server to the client.
#[derive(Debug, Serialize, Deserialize)]
pub struct CmdResponse {
    pub response: String,
    pub sender: i32,
    pub target: String,
}

/// Offer.
#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    pub offer: String,
    pub target: Vec<String>,
}

/// WsErr.
#[derive(Debug, Serialize, Deserialize)]
pub struct WsErr {
    pub msg: String,
}

/// Ices.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ices {
    pub ices: String,
}

/// Answer.
#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub answer: String,
}

/// Volume.
#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    pub volume: String,
}

/// AvsInfo.
#[derive(Debug, Serialize, Deserialize)]
pub struct AvsInfo {
    pub networks: Option<String>,
    pub mem_total: Option<String>,
    pub mem_free: Option<String>,
    pub disk_total: Option<String>,
    pub disk_free: Option<String>,
    pub cpu_temp: Option<String>,
}
