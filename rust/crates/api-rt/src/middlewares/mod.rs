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
use syn::parse::Parse;

use self::{logic::Logic, middleware::Middleware};

mod logic;
mod middleware;

/// MiddlewareParser.
/// The `middleware` macro parser.
pub enum MiddlewareParser {
    Middleware,
    Logic(Ident),
}

impl MiddlewareParser {
    /// Generate code from token stream.
    pub fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
        let m = syn::parse_macro_input!(attr as MiddlewareParser);
        match m {
            MiddlewareParser::Middleware => Middleware::gen(item),
            MiddlewareParser::Logic(ident) => Logic::gen(ident, item),
        }
    }
}

impl Parse for MiddlewareParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self::Middleware)
        } else {
            let ident = input.parse::<Ident>()?;
            Ok(Self::Logic(ident))
        }
    }
}
