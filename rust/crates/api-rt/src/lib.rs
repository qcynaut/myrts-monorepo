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

//! # api-rt
//!
//! The proc-macro to generate documentation, routes, middleware, requests and responses for
//! actix web project. This will reduce the code to be written by the developers
//! which means faster development.

#[cfg(feature = "doc")]
use docs::Doc;

use middlewares::MiddlewareParser;
use proc_macro::TokenStream;
use requests::Request;
use responses::Response;
use routes::{AddRoutes, Route, Routes};
use schemas::Schemas;

#[cfg(feature = "doc")]
mod docs;
mod middlewares;
mod requests;
mod responses;
mod routes;
mod schemas;

/// Generate services for the actix web route.
/// # Example
/// ```ignore
/// api_api_rt::routes! {
///     get
///     post
///     put
///     patch
///     delete
/// }
/// ```
#[proc_macro]
pub fn routes(item: TokenStream) -> TokenStream {
    Routes::gen(item)
}

/// Generate actix web route.
/// # Example
/// ```ignore
/// #[api_rt::route(get, "/", Middleware1, Middleware2, Middleware3)]
/// async fn index() -> impl Responder {
///     HttpResponse::Ok()
/// }
/// ```
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    Route::gen(attr, item)
}

#[cfg(feature = "doc")]
/// Generate documentation for the actix web route.
#[proc_macro_attribute]
pub fn doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    Doc::gen(attr, item)
}

/// Create a request type.
/// # Example
/// ```ignore
/// #[api_rt::request]
/// struct MyRequest;
/// ```
#[proc_macro_attribute]
pub fn request(attr: TokenStream, item: TokenStream) -> TokenStream {
    Request::gen(attr, item)
}

/// Generate schema appender.
/// # Example
/// ```ignore
/// #[api_rt::request]
/// struct MyRequest {
///     ...
/// }
///
/// api_rt::schemas! {
///     MyRequest
/// }
/// ```
#[proc_macro]
pub fn schemas(item: TokenStream) -> TokenStream {
    Schemas::gen(item)
}

/// Generate actix web response.
/// # Example
/// ```ignore
/// #[api_rt::response]
/// struct MyResponse;
/// ```
#[proc_macro_attribute]
pub fn response(attr: TokenStream, item: TokenStream) -> TokenStream {
    Response::gen(attr, item)
}

/// Append routes to the actix web config.
/// # Example
/// ```ignore
/// api_rt::add_routes! {
///     config
///     routes::index
///     routes::users
///     routes::posts
///     ...
/// }
/// ```
#[proc_macro]
pub fn add_routes(item: TokenStream) -> TokenStream {
    AddRoutes::gen(item)
}

/// Generate actix middleware.
/// # Example
/// ```ignore
/// #[api_rt::middleware]
/// struct MyMiddleware;
/// ```
/// You can generate the logic of the middleware like this:
/// ```ignore
/// #[api_rt::middleware(logic)]
/// fn call(&self, req: ServiceRequest) -> Self::Future {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    MiddlewareParser::gen(attr, item)
}
