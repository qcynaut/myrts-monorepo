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

use crate::services::auth::auth_avs;

use super::auth_user;
use api_db::ApiDatabase;
use proto::{
    app::{Data, MsgData, Stream},
    error::Result,
    WsState,
};
use types::proto::Authenticate;
use utils::crypto::Jwt;

#[proto::service("auth")]
async fn auth(
    stream: Stream,
    data: MsgData<Authenticate>,
    state: Data<WsState>,
    jwt: Data<Jwt>,
    db: Data<ApiDatabase>,
) -> Result<()> {
    let data = data.into_inner();
    match data.client_type {
        1 => {
            auth_user::auth(stream, data, state, db, jwt).await;
            Ok(())
        }
        2 => {
            auth_avs::auth(stream, data, state, db).await;
            Ok(())
        }
        _ => Ok(()),
    }
}
