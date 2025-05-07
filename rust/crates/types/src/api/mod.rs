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

#[cfg(feature = "web")]
use actix_multipart::form;

#[cfg(feature = "web")]
pub use self::web::*;

pub mod avs;
pub mod blacklist_token;
pub mod city;
pub mod docs_credential;
pub mod forgot_password;
pub mod package;
pub mod province;
pub mod records;
pub mod role;
pub mod schedules;
#[cfg(feature = "db")]
pub mod schema;
pub mod session;
pub mod subscription;
pub mod user;
pub mod user_group;
pub mod verify;

#[cfg(feature = "web")]
mod web {
    use super::avs::AvsResponse;
    use super::*;
    use super::{
        city::CityResponse, package::PackageResponse, province::ProvinceResponse,
        role::RoleResponse, user_group::UserGroupResponse,
    };

    /// LoginReq.
    /// The login request data.
    #[types_rt::ty(web(Request))]
    pub struct LoginReq {
        pub email: String,
        pub password: String,
        pub mobile: bool,
    }

    /// EmailReq.
    /// The email request data.
    #[types_rt::ty(web(Request))]
    pub struct EmailReq {
        pub email: String,
    }

    /// PasswordReq.
    /// The password request data.
    #[types_rt::ty(web(Request))]
    pub struct PasswordReq {
        pub password: String,
    }

    /// AuthRes.
    /// The authentication response data.
    #[types_rt::ty(web(Response))]
    pub struct AuthRes {
        pub token: String,
        pub pending: bool,
    }

    /// VerifyRes.
    /// The verification response data.
    #[types_rt::ty(web(Response))]
    pub struct VerifyRes {
        pub token: String,
        pub mobile: bool,
    }

    /// CityData.
    /// The data of the city.
    #[types_rt::ty(web(Response))]
    pub struct CityData {
        pub id: i32,
        pub name: String,
        pub province: ProvinceResponse,
    }

    /// SubscriptionData.
    /// The data of the subscription.
    #[types_rt::ty(web(Response))]
    pub struct SubscriptionData {
        pub id: i32,
        pub user_id: i32,
        pub package: PackageResponse,
        pub order_date: chrono::NaiveDateTime,
        pub expire_date: chrono::NaiveDateTime,
    }

    /// UserData.
    /// The data of the user.
    #[types_rt::ty(web(Response))]
    pub struct UserData {
        pub id: i32,
        pub name: String,
        pub email: String,
        pub image_url: Option<String>,
        pub role: RoleResponse,
        pub city: Option<CityData>,
        pub group: Vec<UserGroupResponse>,
        pub subscription: Option<SubscriptionData>,
        pub device_ids: Vec<Option<i32>>,
        pub user_group_ids: Vec<Option<i32>>,
    }

    /// PageQuery.
    /// The query parameters for pagination.
    #[types_rt::ty(web(Request))]
    pub struct PageQuery {
        pub page: Option<i64>,
    }

    /// ServiceQuery.
    /// The query parameters for service.
    #[types_rt::ty(web(Request))]
    pub struct ServiceQuery {
        pub address: String,
        pub description: String,
    }

    /// UserReq.
    /// The data of the user.
    #[types_rt::ty(web(Request))]
    pub struct UserReq {
        pub name: String,
        pub email: String,
        pub role: i32,
        pub group_ids: Vec<i32>,
        pub city_id: Option<i32>,
        pub package_id: Option<i32>,
        pub devices: Vec<i32>,
    }

    /// ProvinceData.
    /// The data of the province.
    #[types_rt::ty(web(Response(pagination: true)))]
    pub struct ProvinceData {
        pub id: i32,
        pub name: String,
        pub cities: Vec<CityResponse>,
    }

    /// GroupData.
    /// The data of the group.
    #[types_rt::ty(web(Response(pagination: true)))]
    pub struct GroupData {
        pub id: i32,
        pub name: String,
        pub description: Option<String>,
        pub children: Vec<GroupData>,
        pub users: Vec<UserData>,
        pub parent_id: Option<i32>,
    }

    /// CmdReq.
    /// Request to execute a command.
    #[types_rt::ty(web(Request))]
    pub struct CmdReq {
        pub id: String,
        pub cmd: String,
    }

    /// MsgReq.
    /// Request to send a message.
    #[types_rt::ty(web(Request))]
    pub struct MsgReq {
        pub id: String,
        pub evt: String,
        pub msg: String,
    }

    /// RecordsReq.
    /// The data of the records.
    #[types_rt::ty(web(Request(form)))]
    pub struct RecordsReq {
        #[schema(value_type = String)]
        pub name: form::text::Text<String>,
        #[schema(value_type = Option<String>)]
        pub description: Option<form::text::Text<String>>,
        #[schema(value_type = String, format = Binary)]
        pub file: form::tempfile::TempFile,
        #[schema(value_type = Vec<i32>)]
        #[multipart(rename = "user_ids[]")]
        pub user_ids: Vec<form::text::Text<i32>>,
    }

    /// AvsStaticParams.
    /// The static parameters for avs.
    #[types_rt::ty(web(Request))]
    pub struct AvsStaticParams {
        pub id: i32,
        pub ipv4: String,
        pub netmask: String,
        pub gateway: String,
    }

    /// Message.
    /// The response message.
    #[types_rt::ty(web(Response))]
    pub struct Message {
        pub message: String,
    }

    /// File.
    /// The response message.
    #[types_rt::ty(web(Response))]
    pub struct File {
        #[serde(skip_serializing)]
        _file: Vec<u8>,
    }

    /// FileReq.
    /// The request data.
    #[types_rt::ty(web(Request(form)))]
    pub struct FileReq {
        #[schema(value_type = String, format = Binary)]
        pub file: form::tempfile::TempFile,
    }

    /// ValIntReq.
    /// The request data.
    #[types_rt::ty(web(Request))]
    pub struct ValIntReq {
        pub val: i32,
    }

    /// StatisticsRes.
    #[types_rt::ty(web(Response))]
    pub struct StatisticsRes {
        pub duration: usize,
        pub schedule: usize,
        pub records: usize,
        pub unit: usize,
        pub avs: usize,
    }

    /// OnGoingStreaming.
    #[types_rt::ty(web(Response))]
    pub struct OnGoingStreaming {
        pub user: UserData,
        pub avs: Vec<AvsResponse>,
    }

    #[cfg(feature = "web")]
    api_rt::schemas! {
        LoginReq
        EmailReq
        AuthRes
        VerifyRes
        CityData
        SubscriptionData
        UserData
        PageQuery
        UserReq
        ProvinceData
        PaginatedProvinceData
        GroupData
        PaginatedGroupData
        CmdReq
        MsgReq
        RecordsReq
        AvsStaticParams
        Message
        File
        FileReq
        ValIntReq
        StatisticsRes
        ServiceQuery
        OnGoingStreaming
    }
}
