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
use syn::ItemStruct;

/// Middleware.
/// The `middleware` macro parser.
pub struct Middleware;

impl Middleware {
    /// Generate code from token stream.
    pub fn gen(item: TokenStream) -> TokenStream {
        let item = syn::parse_macro_input!(item as ItemStruct);
        let vis = &item.vis;
        let ident = &item.ident;
        let logic_ident = Ident::new(&format!("{}Logic", &ident), proc_macro2::Span::call_site());
        quote! {
            #item
            impl<S,B> ::actix_web::dev::Transform<S,::actix_web::dev::ServiceRequest> for #ident
            where
                S: ::actix_web::dev::Service<::actix_web::dev::ServiceRequest, Response = ::actix_web::dev::ServiceResponse<B>, Error = ::actix_web::Error>,
                S::Future: 'static,
                B: 'static
            {
                type Response = ::actix_web::dev::ServiceResponse<::actix_web::body::EitherBody<B>>;
                type Error = ::actix_web::Error;
                type InitError = ();
                type Transform = #logic_ident<S>;
                type Future = ::std::future::Ready<Result<Self::Transform, Self::InitError>>;
                fn new_transform(&self, service: S) -> Self::Future {
                    ::std::future::ready(Ok(#logic_ident { service }))
                }
            }
            #vis struct #logic_ident<S> {
                service: S,
            }
        }.into()
    }
}
