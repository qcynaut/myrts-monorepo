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

use api_db::{repos::SessionsRepo, ApiDatabase};
use proto::{
    app::{Data, Stream},
    WsState,
};
use types::proto::Authenticate;
use utils::crypto::Jwt;

pub(super) async fn auth(
    stream: Stream,
    data: Authenticate,
    state: Data<WsState>,
    db: Data<ApiDatabase>,
    jwt: Data<Jwt>,
) {
    let repo = db.repository::<SessionsRepo>();
    let token = data.client_id;
    match repo.is_valid(&token) {
        Err(_) => {
            let _ = stream.disconnect().await;
            return;
        }
        Ok(val) => {
            if !val {
                let _ = stream.disconnect().await;
                return;
            }
        }
    }
    let id = match jwt.verify(&token) {
        Ok(val) => val.0,
        Err(_) => {
            let _ = stream.disconnect().await;
            return;
        }
    };
    if let Some(_) = state.user_by_id(id).await {
        let _ = stream.disconnect().await;
        return;
    }
    state.set_user(id, stream.clone()).await;
    let _ = stream.write("authenticated", "").await;
}
