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
use syn::{parse::Parse, ItemStruct};

/// Request.
/// The `Request` macro parser.
pub struct Request {
    form: bool,
}

impl Request {
    /// Create derived macro.
    fn derive(&self) -> proc_macro2::TokenStream {
        let mut item = if self.form {
            quote!(::actix_multipart::form::MultipartForm)
        } else {
            quote!(::serde::Deserialize)
        };
        #[cfg(feature = "doc")]
        item.extend(quote!(,::utoipa::ToSchema));
        item
    }

    #[cfg(not(feature = "doc"))]
    /// Clear doc on fields.
    fn clear(&self, item: &mut ItemStruct) {
        item.fields.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident("schema"));
        });
    }

    /// Generate code from token stream.
    pub fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
        let attr = syn::parse_macro_input!(attr as Request);
        let mut item = syn::parse_macro_input!(item as ItemStruct);
        #[cfg(not(feature = "doc"))]
        attr.clear(&mut item);
        let derive = attr.derive();
        quote! {
            #[derive(#derive)]
            #item
        }
        .into()
    }
}

impl Parse for Request {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Request { form: false });
        }
        let form = input.parse::<Ident>()?;
        let form_str = &*form.to_string();
        match form_str {
            "form" => Ok(Request { form: true }),
            _ => Err(syn::Error::new(input.span(), "unknown request type")),
        }
    }
}
