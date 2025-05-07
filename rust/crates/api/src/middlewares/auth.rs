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

use crate::{ApiError, Token};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, HttpMessage,
};
use api_db::{
    repos::{BlacklistTokenRepo, SessionsRepo, UserRepo},
    ApiDatabase,
};
use utils::crypto::Jwt;

/// Auth.
/// Used to protect endpoints.
#[api_rt::middleware]
pub struct Auth;

macro_rules! err {
    ($req:expr, $status:expr, $msg:expr) => {
        Box::pin(async move {
            let res = ApiError::new($msg).status($status).response();
            Ok(ServiceResponse::new(
                $req.request().to_owned(),
                res.map_into_right_body(),
            ))
        })
    };
    ($req:expr, $res:expr) => {
        Box::pin(async move {
            Ok(ServiceResponse::new(
                $req.request().to_owned(),
                $res.map_into_right_body(),
            ))
        })
    };
}

#[api_rt::middleware(Auth)]
fn call(&self, req: ServiceRequest) -> Self::Future {
    let headers = req.headers();
    match headers.get("api_key") {
        None => err!(req, 401, "api_key not found"),
        Some(v) => {
            let jwt = req.app_data::<web::Data<Jwt>>().unwrap();
            let db = req.app_data::<web::Data<ApiDatabase>>().unwrap();
            let repo = db.repository::<UserRepo>();
            let token = v.to_str().unwrap();
            let session = db.repository::<SessionsRepo>();
            let blacklist = db.repository::<BlacklistTokenRepo>();
            match session.is_valid(token) {
                Err(err) => {
                    return err!(req, ApiError::from(err).status(401).response());
                }
                Ok(valid) => {
                    if !valid {
                        return err!(req, 401, "invalid token");
                    }
                }
            }
            match blacklist.is_blacklisted(token) {
                Err(err) => {
                    return err!(req, ApiError::from(err).status(401).response());
                }
                Ok(blacklisted) => {
                    if blacklisted {
                        return err!(req, 401, "blacklisted token");
                    }
                }
            }
            match jwt.verify(token) {
                Err(err) => err!(req, ApiError::new(err).status(401).response()),
                Ok((id, expired)) => {
                    if expired {
                        return err!(req, 401, "expired token");
                    }
                    match repo.get(id) {
                        Err(err) => err!(req, ApiError::from(err).status(401).response()),
                        Ok(user) => {
                            req.extensions_mut().insert(user);
                            req.extensions_mut().insert(Token {
                                token: token.to_owned(),
                            });
                            let res = self.service.call(req);
                            Box::pin(
                                async move { res.await.map(ServiceResponse::map_into_left_body) },
                            )
                        }
                    }
                }
            }
        }
    }
}
