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

#[cfg(feature = "db")]
use super::schema::*;
#[cfg(feature = "db")]
use diesel::prelude::*;
use types_rt::ty;

/// Avs.
/// The avs data.
#[ty(db(kind: Query, table: avs))]
pub struct Avs {
    pub id: i32,
    pub unique_id: String,
    /// The status of the avs: 0 = inactive, 1 = active.
    pub status: i32,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub address: Option<String>,
    pub description: Option<String>,
    /// The type of the avs.
    pub kind: i32,
    pub pending: i32,
    pub slots: String,
    pub networks: Option<String>,
    pub mem_total: Option<String>,
    pub mem_free: Option<String>,
    pub disk_total: Option<String>,
    pub disk_free: Option<String>,
    pub cpu_temp: Option<String>,
}

/// Avs.
/// The avs data.
#[ty(web(Response))]
pub struct AvsResponse {
    pub id: i32,
    pub unique_id: String,
    /// The status of the avs: 0 = inactive, 1 = active.
    pub status: i32,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub address: Option<String>,
    pub description: Option<String>,
    /// The type of the avs.
    pub kind: i32,
    pub pending: i32,
    pub networks: Option<String>,
    pub mem_total: Option<String>,
    pub mem_free: Option<String>,
    pub disk_total: Option<String>,
    pub disk_free: Option<String>,
    pub cpu_temp: Option<String>,
}

#[cfg(feature = "web")]
impl From<Avs> for AvsResponse {
    fn from(avs: Avs) -> Self {
        Self {
            __status: 200,
            id: avs.id,
            unique_id: avs.unique_id,
            status: avs.status,
            lat: avs.lat,
            lng: avs.lng,
            address: avs.address,
            description: avs.description,
            kind: avs.kind,
            pending: avs.pending,
            networks: avs.networks,
            mem_total: avs.mem_total,
            mem_free: avs.mem_free,
            disk_total: avs.disk_total,
            disk_free: avs.disk_free,
            cpu_temp: avs.cpu_temp,
        }
    }
}

/// NewAvs.
/// The data to create an avs.
#[ty(db(kind: Insert, table: avs),web(Request))]
pub struct NewAvs {
    pub unique_id: String,
    pub status: i32,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub kind: i32,
    pub pending: i32,
    pub slots: String,
    pub networks: Option<String>,
    pub mem_total: Option<String>,
    pub mem_free: Option<String>,
    pub disk_total: Option<String>,
    pub disk_free: Option<String>,
    pub cpu_temp: Option<String>,
}

/// PartialUpdateAvs.
/// The data to partially update an avs.
#[ty(db(kind: Update, table: avs),web(Request))]
pub struct PartialUpdateAvs {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub kind: Option<i32>,
}

/// UdateAvsInfo
/// The data to update an avs.
#[ty(db(kind: Update, table: avs),web(Request))]
pub struct UpdateAvsInfo {
    pub networks: Option<String>,
    pub mem_total: Option<String>,
    pub mem_free: Option<String>,
    pub disk_total: Option<String>,
    pub disk_free: Option<String>,
    pub cpu_temp: Option<String>,
}

api_rt::schemas! {
    AvsResponse
    NewAvs
    PartialUpdateAvs
}
