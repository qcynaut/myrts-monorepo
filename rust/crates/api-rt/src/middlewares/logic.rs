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

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::ItemFn;

/// Logic.
/// The `middleware` macro parser.
pub struct Logic;

impl Logic {
    /// Generate code from token stream.
    pub fn gen(ident: Ident, item: TokenStream) -> TokenStream {
        let item = syn::parse_macro_input!(item as ItemFn);
        let ident = Ident::new(&format!("{}Logic", &ident), proc_macro2::Span::call_site());
        quote! {
            impl<S,B> ::actix_web::dev::Service<::actix_web::dev::ServiceRequest> for #ident<S>
            where
                S: ::actix_web::dev::Service<::actix_web::dev::ServiceRequest, Response = ::actix_web::dev::ServiceResponse<B>, Error = ::actix_web::Error>,
                S::Future: 'static,
                B: 'static
            {
                type Response = ::actix_web::dev::ServiceResponse<::actix_web::body::EitherBody<B>>;
                type Error = ::actix_web::Error;
                type Future = ::futures_util::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
                ::actix_web::dev::forward_ready!(service);
                #item
            }
        }.into()
    }
}
