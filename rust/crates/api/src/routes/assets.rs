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

use crate::ApiError;
use actix_web::{http::StatusCode, web};
use utils::files::ApiAssets;

/// # Get audio.
/// Get the audio file.
/// ---
/// tags:
///   - assets
/// responses:
///     - status: 200
///       content: !T File
///       content_type: audio/mp3
///       description: The audio file.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: name
///       kind: !Path String
///       required: true
///       description: The name of the audio file.
#[api_rt::route(get, "/assets/audio/{name}")]
async fn get_audio(
    name: web::Path<String>,
    assets: web::Data<ApiAssets>,
) -> actix_web::Result<impl actix_web::Responder> {
    let f = assets.read_audio(&name).map_err(ApiError::new)?;
    Ok(actix_web::HttpResponse::build(StatusCode::OK)
        .content_type("audio/mp3")
        .body(f))
}

/// # Get image.
/// Get the image file.
/// ---
/// tags:
///   - assets
/// responses:
///     - status: 200
///       content: !T File
///       content_type: image/png
///       description: The image file.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: name
///       kind: !Path String
///       required: true
///       description: The name of the image file.
#[api_rt::route(get, "/assets/image/{name}")]
async fn get_image(
    name: web::Path<String>,
    assets: web::Data<ApiAssets>,
) -> actix_web::Result<impl actix_web::Responder> {
    let f = assets.read_image(&name).map_err(ApiError::new)?;
    Ok(actix_web::HttpResponse::build(StatusCode::OK)
        .content_type("image/png")
        .body(f))
}

api_rt::routes! {
    get_audio
    get_image
}
