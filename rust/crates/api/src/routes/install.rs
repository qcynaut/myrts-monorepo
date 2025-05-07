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

use crate::{middlewares::auth::Auth, ApiError};
use actix_multipart::form::MultipartForm;
use actix_web::{web, Result};
use std::{collections::HashMap, io::Read};
use types::api::{user::User, FileReq, Message};
use utils::{crypto::uuid, files::ApiAssets};

#[api_rt::route(post, "/install", Auth)]
async fn install_post(
    user: web::ReqData<User>,
    data: MultipartForm<FileReq>,
    assets: web::Data<ApiAssets>,
) -> Result<Message> {
    if user.role_id > 2 {
        return Err(ApiError::new("Only root and super admin can install")
            .status(401)
            .into());
    }
    let mut data = data.into_inner();
    let name = data.file.file_name.unwrap_or(uuid());
    let f = data.file.file.as_file_mut();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|e| ApiError::new(e))?;
    assets.write_other(&name, &buf).map_err(ApiError::new)?;
    Message::new(name).wrap()
}

#[api_rt::route(get, "/install/{name}")]
async fn install_get(
    name: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    assets: web::Data<ApiAssets>,
) -> Result<impl actix_web::Responder> {
    let buf = assets.read_other(&name).map_err(ApiError::new)?;
    if query.is_empty() {
        return Ok(actix_web::HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(buf));
    }
    let query = query.into_inner();
    let mut buf = String::from_utf8_lossy(&buf).to_string();
    for (k, v) in query {
        let fmt = format!("{{{}}}", k.to_uppercase());
        let replaced = buf.replace(&fmt, &v);
        buf = replaced;
    }
    Ok(actix_web::HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(buf))
}

api_rt::routes! {
    install_post
    install_get
}
