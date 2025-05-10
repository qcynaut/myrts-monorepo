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

use crate::{middlewares::auth::Auth, ApiError, Config, Token};
use actix_web::web;
use api_db::{
    repos::{ForgotPasswordRepo, SessionsRepo, UserRepo, VerifyRepo},
    ApiDatabase,
};
use types::api::{user::User, AuthRes, EmailReq, LoginReq, Message, PasswordReq, VerifyRes};
use utils::{
    crypto::{Bcrypt, Jwt},
    mail::Mail,
};

/// # Check auth token.
///
/// This endpoint checks whether the provided token is valid or not. If the token is pending, it is considered invalid.
/// ---
/// tags:
///     - auth
/// responses:
///     - status: 200
///       content: !T Message
///       description: The token is valid.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/auth", Auth)]
async fn get() -> actix_web::Result<Message> {
    Message::new("ok".to_owned()).wrap()
}

/// Verify user.
/// ---
/// tags:
///     - auth
/// responses:
///     - status: 200
///       content: !T VerifyRes
///       description: User verified.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 404
///       content: !T ApiError
///       description: Key not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: uuid
///       kind: !Path String
///       required: true
///       description: user uuid.
#[api_rt::route(get, "/auth/verify/{uuid}")]
async fn get_verify(
    uuid: web::Path<String>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<VerifyRes> {
    let repo = db.repository::<VerifyRepo>();
    let (token, mobile) = repo.verify(&uuid).map_err(ApiError::from)?;
    VerifyRes::new(token, mobile).wrap()
}

/// Check reset link.
/// ---
/// tags:
///     - auth
/// responses:
///     - status: 200
///       content: !T Message
///       description: The link is valid.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 404
///       content: !T ApiError
///       description: The link is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: uuid
///       kind: !Path String
///       required: true
///       description: user uuid.
#[api_rt::route(get, "/auth/reset/{uuid}")]
async fn get_reset(
    uuid: web::Path<String>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<Message> {
    let repo = db.repository::<ForgotPasswordRepo>();
    let res = repo.exists(&uuid).map_err(ApiError::from)?;
    if !res {
        return Err(ApiError::new("Link not found or expired")
            .status(401)
            .into());
    }
    Message::new("ok".to_owned()).wrap()
}

/// # Authenticate the user.
///
/// This endpoint is responsible for authenticating the user.
///
/// Before the user can access any of the routes, they must be authenticated first. The `mobile` field in the request body
/// indicates whether the user is accessing the API from a mobile device `(true)` or not `(false)`.
///
/// After the authentication process is completed, the endpoint will return the generated API key along with the `pending`
/// field set to `false` if no other session is detected.
///
/// For mobile developers, it is necessary to save the token locally and wait for the user to complete the `confirmation`
/// process sent by email.
/// ---
/// tags:
///   - auth
/// request:
///     content: !T LoginReq
///     content_type: application/json
///     description: Login data.
/// responses:
///     - status: 200
///       content: !T AuthRes
///       description: The user is authenticated.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 401
///       content: !T ApiError
///       description: The email or password is incorrect.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
#[api_rt::route(post, "/auth")]
async fn post(
    data: web::Json<LoginReq>,
    db: web::Data<ApiDatabase>,
    jwt: web::Data<Jwt>,
    mail: web::Data<Mail>,
    cfg: web::Data<Config>,
) -> actix_web::Result<AuthRes> {
    let user_repo = db.repository::<UserRepo>();
    let session_repo = db.repository::<SessionsRepo>();
    let verify_repo = db.repository::<VerifyRepo>();

    let user = user_repo
        .get_by_email(&data.email)
        .map_err(ApiError::from)?;
    if !Bcrypt::verify(&user.password, &data.password).map_err(ApiError::new)? {
        return Err(ApiError::new("Incorrect email or password").into());
    }
    let need_pending = session_repo
        .is_need_pending(user.id, data.mobile)
        .map_err(ApiError::new)?;
    let token = jwt.create(user.id).map_err(ApiError::new)?;
    if need_pending {
        let pending_id = session_repo
            .create_pending(user.id, data.mobile, &token)
            .map_err(ApiError::from)?;
        let uuid = verify_repo
            .create(data.mobile, pending_id)
            .map_err(ApiError::from)?;
        mail.send_confirmation(
            &user.name,
            &user.email,
            &cfg.format_web(format!("/verify/{uuid}")),
        )
        .await
        .map_err(ApiError::new)?;
        AuthRes::new(token, true).wrap()
    } else {
        let _ = session_repo
            .create_session(user.id, data.mobile, &token)
            .map_err(ApiError::from)?;
        AuthRes::new(token, false).wrap()
    }
}

/// # Forgot password.
///
/// This endpoint sends the reset link to the user. The password reset process occurs on the MyRTS website, and the mobile app should only send the reset password request to this endpoint.
/// ---
/// tags:
///   - auth
/// request:
///     content: !T EmailReq
///     content_type: application/json
///     description: Email of the user.
/// responses:
///     - status: 200
///       content: !T Message
///       description: The reset link is sent.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 404
///       content: !T ApiError
///       description: The user is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
#[api_rt::route(post, "/auth/forgot")]
async fn post_forgot(
    data: web::Json<EmailReq>,
    db: web::Data<ApiDatabase>,
    mail: web::Data<Mail>,
    cfg: web::Data<Config>,
) -> actix_web::Result<Message> {
    let user_repo = db.repository::<UserRepo>();
    let forgot_repo = db.repository::<ForgotPasswordRepo>();
    let user = user_repo
        .get_by_email(&data.email)
        .map_err(ApiError::from)?;
    let forgot = forgot_repo.create(user.id).map_err(ApiError::from)?;
    mail.send_reset(
        &user.name,
        &user.email,
        &cfg.format_web(format!("/reset/{}", forgot.uuid)),
    )
    .await
    .map_err(ApiError::new)?;
    Message::new("ok".to_owned()).wrap()
}

/// Reset password.
/// ---
/// tags:
///   - auth
/// request:
///     content: !T PasswordReq
///     content_type: application/json
///     description: Password of the user.
/// responses:
///     - status: 200
///       content: !T Message
///       description: The password is changed.
///     - status: 400
///       content: !T ApiError
///       description: The request is invalid.
///     - status: 404
///       content: !T ApiError
///       description: The link is not found.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// params:
///     - name: uuid
///       kind: !Path String
///       required: true
///       description: user uuid.
#[api_rt::route(post, "/auth/reset/{uuid}")]
async fn post_reset(
    data: web::Json<PasswordReq>,
    uuid: web::Path<String>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<Message> {
    let repo = db.repository::<ForgotPasswordRepo>();
    let password = Bcrypt::hash(&data.password).map_err(ApiError::new)?;
    repo.reset(&uuid, &password).map_err(ApiError::from)?;
    Message::new("ok".to_owned()).wrap()
}

/// # Logout.
/// Logout the user and clear all the session.
/// ---
/// tags:
///   - auth
/// responses:
///     - status: 200
///       content: !T Message
///       description: The user is logged out.
///     - status: 401
///       content: !T ApiError
///       description: The user is not authenticated.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(delete, "/auth", Auth)]
async fn delete(
    token: web::ReqData<Token>,
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<Message> {
    let repo = db.repository::<SessionsRepo>();
    repo.logout(user.id, &token.token).map_err(ApiError::from)?;
    Message::new("ok".to_owned()).wrap()
}

api_rt::routes! {
    get
    #[no_doc]
    get_verify
    #[no_doc]
    get_reset
    #[no_doc]
    post_reset
    post
    post_forgot
    delete
}
