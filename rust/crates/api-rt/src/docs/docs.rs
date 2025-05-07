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
use quote::{quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, Expr, ItemStruct, Token};

use super::infos::Info;

/// Doc.
/// The `doc` attribute parser.
pub struct Doc {
    routes: Vec<Expr>,
    types: Vec<Expr>,
    mods: Vec<Expr>,
    info: Info,
}

pub struct DocComment {
    content: String,
}

impl Parse for DocComment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![#]>()?;
        let content;
        syn::bracketed!(content in input);
        content.parse::<Ident>()?;
        content.parse::<Token![=]>()?;
        let lit_str = content.parse::<syn::LitStr>()?;
        Ok(Self {
            content: lit_str.value(),
        })
    }
}

impl Doc {
    /// Generate code from token stream.
    pub fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
        let mut attr = syn::parse_macro_input!(attr as Doc);
        let item = syn::parse_macro_input!(item as ItemStruct);
        let item_attr = &item.attrs;
        let mut doc = String::new();
        for attr in item_attr {
            if attr.path().is_ident("doc") {
                let token = attr.into_token_stream().into();
                let parsed = syn::parse_macro_input!(token as DocComment);
                doc.push_str(&parsed.content);
                doc.push('\n');
            }
        }
        if !doc.is_empty() {
            attr.info = Info::load(Some(doc));
        }
        let ident = &item.ident;
        quote! {
            #item
            impl ::utoipa::OpenApi for #ident {
                fn openapi() -> ::utoipa::openapi::OpenApi {
                    #attr
                }
            }
        }
        .into()
    }
}

impl ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let info = &self.info;
        let mods_expr = &self.mods;
        let mods_size = mods_expr.len();
        let mut routes = quote!(::utoipa::openapi::PathsBuilder::new());
        let mut types = quote!(::utoipa::openapi::ComponentsBuilder::new());
        let mut mods = quote!();

        for route in &self.routes {
            routes = quote!(#route ::__rt_docs(#routes));
        }

        for ty in &self.types {
            types = quote!(#ty ::__rt_schemas(#types))
        }

        let mut mod_comma = false;
        for m in mods_expr {
            if mod_comma {
                mods.extend(quote!(,));
            }
            mods.extend(quote!(#m));
            if !mod_comma {
                mod_comma = true;
            }
        }

        tokens.extend(quote! {
            use ::utoipa::{Path, ToSchema};
            let mut openapi = ::utoipa::openapi::OpenApiBuilder::new()
            .info(#info)
            .paths(#routes .build())
            .components(Some(#types .build()))
        });

        tokens.extend(quote!(.build();));
        tokens.extend(quote! {
            let _mods: [&dyn ::utoipa::Modify; #mods_size] = [#mods];
            _mods
            .iter()
            .for_each(|modifier| modifier.modify(&mut openapi));
        });
        tokens.extend(quote!(openapi));
    }
}

impl Parse for Doc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut routes = Vec::new();
        let mut types = Vec::new();
        let mut mods = Vec::new();

        while !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            let ident_str = &*ident.to_string();
            match ident_str {
                "routes" => {
                    let item;
                    syn::parenthesized!(item in input);
                    routes = Punctuated::<Expr, Token![,]>::parse_terminated(&item)?
                        .into_iter()
                        .collect();
                }
                "types" => {
                    let item;
                    syn::parenthesized!(item in input);
                    types = Punctuated::<Expr, Token![,]>::parse_terminated(&item)?
                        .into_iter()
                        .collect();
                }
                "mods" => {
                    let item;
                    syn::parenthesized!(item in input);
                    mods = Punctuated::<Expr, Token![,]>::parse_terminated(&item)?
                        .into_iter()
                        .collect();
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown attribute: {}", ident_str),
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            routes,
            types,
            info: Info::load(None),
            mods,
        })
    }
}
