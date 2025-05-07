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
    dev::{ServiceRequest, ServiceResponse},
    web, HttpResponse,
};
use api_db::{repos::DocCredentialRepo, ApiDatabase};
use utils::encoding::Base64;

/// DocsAuth.
/// Used to authenticate user when accessing documentation.
#[api_rt::middleware]
pub struct DocsAuth;

#[api_rt::middleware(DocsAuth)]
fn call(&self, req: ServiceRequest) -> Self::Future {
    let path = req.path();
    if path.starts_with("/swagger")
        || path.starts_with("/rapidoc")
        || path.starts_with("/redoc")
        || path.starts_with("/docs")
    {
        let basic_auth = req.headers().get("Authorization");
        if basic_auth.is_none() {
            return Box::pin(async move {
                let res = HttpResponse::Unauthorized()
                    .append_header(("WWW-Authenticate", "Basic realm=\"myrts\""))
                    .finish();
                Ok(ServiceResponse::new(
                    req.request().clone(),
                    res.map_into_right_body(),
                ))
            });
        }
        let (username, password) = {
            let auth = basic_auth
                .unwrap()
                .to_str()
                .unwrap_or("")
                .replace("Basic ", "");
            let decoded = Base64::dec(&auth).unwrap_or_default();
            let split = decoded
                .split_once(':')
                .map(|(x, y)| (x.to_string(), y.to_string()));
            split.unwrap_or_default()
        };
        let db = req.app_data::<web::Data<ApiDatabase>>().unwrap();
        let repo = db.repository::<DocCredentialRepo>();
        match repo.get(&username) {
            Ok(user) => {
                if user.password != password {
                    return Box::pin(async move {
                        let res = HttpResponse::Unauthorized()
                            .append_header(("WWW-Authenticate", "Basic realm=\"myrts\""))
                            .finish();
                        Ok(ServiceResponse::new(
                            req.request().clone(),
                            res.map_into_right_body(),
                        ))
                    });
                } else {
                    let res = self.service.call(req);
                    Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
                }
            }
            Err(_) => {
                return Box::pin(async move {
                    let res = HttpResponse::Unauthorized()
                        .append_header(("WWW-Authenticate", "Basic realm=\"myrts\""))
                        .finish();
                    Ok(ServiceResponse::new(
                        req.request().clone(),
                        res.map_into_right_body(),
                    ))
                })
            }
        }
    } else {
        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
