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

#![allow(unused_mut)]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parse, ItemFn, LitStr};

#[cfg(feature = "doc")]
use super::docs::{Doc, DocParser};

/// Route.
/// The `route` macro parser.
pub struct Route {
    method: Ident,
    path: String,
    #[cfg(feature = "doc")]
    doc: Option<Doc>,
    middleware: Option<Ident>,
}

impl Route {
    #[cfg(feature = "doc")]
    /// Generate documentation for the route.
    fn doc(&self, tokens: &mut proc_macro2::TokenStream) {
        let doc = &self.doc;
        tokens.extend(quote!(#doc));
    }

    /// Generate route attribute.
    fn attr(&self, tokens: &mut proc_macro2::TokenStream) {
        let method = &self.method;
        let path = &*self.path;
        let middleware = if let Some(m) = &self.middleware {
            let m = m.to_string();
            quote!(,wrap=#m)
        } else {
            quote!()
        };
        tokens.extend(quote!(#[actix_web:: #method(#path #middleware)]));
    }

    /// Generate the code for the route.
    fn code(&self) -> proc_macro2::TokenStream {
        let mut code = quote!();
        #[cfg(feature = "doc")]
        self.doc(&mut code);
        self.attr(&mut code);
        code
    }

    /// Generate code from token stream.
    pub fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
        let mut attr = syn::parse_macro_input!(attr as Route);
        let mut item = syn::parse_macro_input!(item as ItemFn);
        #[cfg(feature = "doc")]
        {
            attr.doc = match DocParser::new(&mut item, &attr.path, &attr.method).parse() {
                Ok(doc) => Some(doc),
                Err(e) => {
                    return e.to_compile_error().into();
                }
            };
        }
        let code = attr.code();
        quote!(#code #item).into()
    }
}

impl Parse for Route {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let method = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let path = input.parse::<LitStr>()?.value();
        let mut middleware = None;
        if !input.is_empty() {
            input.parse::<syn::token::Comma>()?;
            middleware = Some(input.parse()?);
        }
        Ok(Route {
            method,
            path,
            #[cfg(feature = "doc")]
            doc: None,
            middleware,
        })
    }
}
