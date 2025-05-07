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

use actix_web::{
    body::EitherBody,
    dev::ServiceRequest,
    http::header::{self, HeaderValue},
};

use crate::ApiError;

/// Error.
/// Middleware that handle errors.
#[api_rt::middleware]
pub struct Error;

#[api_rt::middleware(Error)]
fn call(&self, req: ServiceRequest) -> Self::Future {
    let res = self.service.call(req);
    Box::pin(async move {
        match res.await {
            Err(e) => Err(e),
            Ok(mut res) => {
                if res.status().is_success() {
                    Ok(res.map_into_left_body())
                } else {
                    let is_json = res
                        .headers()
                        .get("Content-Type")
                        .map(|r| r == "application/json")
                        .unwrap_or(false);
                    if is_json {
                        Ok(res.map_into_left_body())
                    } else {
                        res.headers_mut().append(
                            header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        );
                        let error = format!(
                            "Error: {}",
                            res.status().canonical_reason().unwrap_or_default()
                        );
                        let error = ApiError::new(&error).status(res.status().as_u16());
                        Ok(res.map_body(|_, _| EitherBody::right(error.response().into_body())))
                    }
                }
            }
        }
    })
}
