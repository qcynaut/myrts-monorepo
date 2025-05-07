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

//! # api
//!
//! The REST API for myrts.

use actix_web::{middleware::Logger, web};
use api_db::ApiDatabase;
use proto::WsState;
use utils::{crypto::Jwt, files::ApiAssets, mail::Mail};

mod middlewares;
mod routes;

/// ApiError.
/// Error response.
#[api_rt::response(error: 500)]
pub struct ApiError;

api_rt::schemas! {
    ApiError
}

impl From<api_db::DatabaseError> for ApiError {
    fn from(value: api_db::DatabaseError) -> Self {
        match value {
            api_db::DatabaseError::Connection => ApiError::new("Database connection error"),
            api_db::DatabaseError::NotFound => ApiError::new("Resource not found").status(404),
            api_db::DatabaseError::Conflict => ApiError::new("Resource already exists").status(409),
            api_db::DatabaseError::Expired => ApiError::new("Expired").status(401),
            api_db::DatabaseError::Other(e) => ApiError::new(e),
        }
    }
}

/// Token.
/// The jwt token data.
#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
}

#[derive(Clone)]
/// This is just for saving the base url.
pub(crate) struct Config {
    base_url: String,
    web_url: String,
}

impl Config {
    /// Create a new config.
    pub fn new(base_url: &str, web_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            web_url: web_url.to_string(),
        }
    }

    /// Format url.
    pub fn format<T>(&self, path: T) -> String
    where
        T: std::fmt::Display,
    {
        format!("{}{}", &self.base_url, path)
    }

    /// Format web url.
    pub fn format_web<T>(&self, path: T) -> String
    where
        T: std::fmt::Display,
    {
        format!("{}{}", &self.web_url, path)
    }
}

#[cfg(feature = "doc")]
/// Register documentation routes.
fn doc(cfg: &mut actix_web::web::ServiceConfig) {
    use utoipa::OpenApi;
    use utoipa_rapidoc::RapiDoc;
    use utoipa_redoc::{Redoc, Servable};
    use utoipa_swagger_ui::SwaggerUi;

    /// ApiKey.
    /// Used to add api_key to the headers from documentation.
    struct ApiKey;

    impl utoipa::Modify for ApiKey {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    utoipa::openapi::security::SecurityScheme::ApiKey(
                        utoipa::openapi::security::ApiKey::Header(
                            utoipa::openapi::security::ApiKeyValue::new("api_key"),
                        ),
                    ),
                );
            }
        }
    }

    /// # MyRTS api documentation.
    ///
    /// This api contains the following endpoints:
    /// - `/assets` - The assets endpoint `(stable)`.
    /// - `/auth` - The authentication endpoint `(stable)`.
    /// - `/avs` - The avs endpoint `(stable)`.
    /// - `/cities` - The cities endpoint `(stable)`.
    /// - `/groups` - The groups endpoint `(stable)`.
    /// - `/packages` - The packages endpoint `(stable)`.
    /// - `/provinces` - The provinces endpoint `(stable)`.
    /// - `/records` - The records endpoint `(stable)`.
    /// - `/roles` - The roles endpoint `(stable)`.
    /// - `/schedules` - The schedules endpoint `(stable)`.
    /// - `/users` - The users endpoint `(stable)`.
    ///
    /// ## Notes
    /// - Every generated timestamp is in UTC so you need to convert it to your local timezone.
    #[api_rt::doc(
        routes(
            routes::assets,
            routes::avs,
            routes::auth,
            routes::users,
            routes::groups,
            routes::provinces,
            routes::cities,
            routes::roles,
            routes::packages,
            routes::schedules,
            routes::records,
            routes::statistics,
            routes::streaming,
        ),
        types(
            self,
            types::api,
            types::api::avs,
            types::api::city,
            types::api::province,
            types::api::role,
            types::api::user,
            types::api::user_group,
            types::api::package,
            types::api::subscription,
            types::api::schedules,
            types::api::records,
        ),
        mods(&ApiKey)
    )]
    struct Doc;

    let openapi = Doc::openapi();
    cfg.service(Redoc::with_url("/redoc", openapi.clone()))
        .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", openapi.clone()))
        .service(RapiDoc::new("/docs/openapi.json").path("/rapidoc"));
}

/// Configure app.
fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    #[cfg(feature = "doc")]
    doc(cfg);

    api_rt::add_routes! {
        cfg
        routes::avs
        routes::auth
        routes::users
        routes::groups
        routes::provinces
        routes::cities
        routes::roles
        routes::packages
        routes::schedules
        routes::records
        routes::assets
        routes::statistics
        routes::install
        routes::streaming
    }
}

/// Run the api.
pub async fn run(
    port: u16,
    db: ApiDatabase,
    jwt: Jwt,
    mail: Mail,
    assets: ApiAssets,
    base_url: &str,
    web_url: &str,
    ws_state: WsState,
) -> std::io::Result<()> {
    let config = Config::new(base_url, web_url);
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .configure(configure)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(jwt.clone()))
            .app_data(web::Data::new(mail.clone()))
            .app_data(web::Data::new(assets.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(ws_state.clone()))
            .wrap(middlewares::docs::DocsAuth)
            .wrap(middlewares::error::Error)
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
