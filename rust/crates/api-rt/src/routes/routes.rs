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
use syn::{parse::Parse, Expr};

/// Routes.
/// The `routes` macro parser.
pub struct Routes {
    routes: Vec<(Ident, bool)>,
}

impl Routes {
    #[cfg(feature = "doc")]
    /// Generate the doc code for the routes.
    fn doc(&self) -> proc_macro2::TokenStream {
        let mut inner = quote!();
        for (route, doc) in &self.routes {
            if *doc {
                let doc_ident = Ident::new(&format!("__doc_{}", route), route.span());
                inner.extend(quote!(.path(#doc_ident ::path(), #doc_ident ::path_item(None))));
            }
        }
        quote! {
            pub fn __rt_docs(builder: ::utoipa::openapi::PathsBuilder) -> ::utoipa::openapi::PathsBuilder {
                use ::utoipa::Path;
                builder #inner
            }
        }
    }

    /// Generate the code for the routes.
    fn code(&self) -> proc_macro2::TokenStream {
        let mut inner = quote!();
        #[cfg(feature = "doc")]
        let doc = self.doc();
        #[cfg(not(feature = "doc"))]
        let doc = quote!();
        for (route, _) in &self.routes {
            inner.extend(quote!(.service(#route)));
        }
        quote!(
            pub fn __rt_routes(cfg: &mut ::actix_web::web::ServiceConfig) -> &mut ::actix_web::web::ServiceConfig {
                cfg #inner
            }
            #doc
        )
    }

    /// Generate code from token stream.
    pub fn gen(item: TokenStream) -> TokenStream {
        let item = syn::parse_macro_input!(item as Routes);
        item.code().into()
    }
}

impl Parse for Routes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut routes = Vec::new();

        while !input.is_empty() {
            let mut doc = true;
            // Check if it's a #[no_doc].
            if input.peek(syn::token::Pound) {
                input.parse::<syn::token::Pound>()?;
                let bracketed;
                syn::bracketed!(bracketed in input);
                let ident = bracketed.parse::<Ident>()?;
                if &*ident.to_string() != "no_doc" {
                    return Err(syn::Error::new_spanned(ident, "Unknown attribute"));
                } else {
                    doc = false;
                }
            }
            let ident = input.parse::<Ident>()?;
            routes.push((ident, doc));
        }

        Ok(Self { routes })
    }
}

/// AddRoutes.
/// The `add_routes` macro parser.
pub struct AddRoutes {
    config: Ident,
    routes: Vec<Expr>,
}

impl AddRoutes {
    /// Generate the code for the routes.
    fn code(&self) -> proc_macro2::TokenStream {
        let config = &self.config;
        let mut inner = quote!(#config);
        for route in &self.routes {
            inner = quote!(#route ::__rt_routes(#inner));
        }
        if !self.routes.is_empty() {
            quote!(#inner;)
        } else {
            quote!()
        }
    }

    /// Generate code from token stream.
    pub fn gen(item: TokenStream) -> TokenStream {
        let item = syn::parse_macro_input!(item as AddRoutes);
        item.code().into()
    }
}

impl Parse for AddRoutes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let config = input.parse()?;
        let mut routes = Vec::new();
        while !input.is_empty() {
            routes.push(input.parse()?);
        }
        Ok(Self { config, routes })
    }
}
