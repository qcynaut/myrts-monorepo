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
use syn::{parse::Parse, Field, ItemStruct};

/// Response.
/// The `response` macro parser.
pub struct Response {
    error: bool,
    status: u16,
}

impl Response {
    /// Create derived macro.
    fn derive(&self) -> proc_macro2::TokenStream {
        let mut item = quote!(serde::Serialize);
        #[cfg(feature = "doc")]
        item.extend(quote!(,::utoipa::ToSchema));
        if self.error {
            item.extend(quote!(,Debug));
        }
        item
    }

    #[cfg(not(feature = "doc"))]
    /// Clear doc on fields.
    fn clear(&self, item: &mut ItemStruct) {
        item.fields.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident("schema"));
        });
    }

    /// Rebuild response struct.
    fn rebuild(&self, item: &mut ItemStruct) -> syn::Result<()> {
        let ident = item.ident.clone();
        let vis = item.vis.clone();
        let fields_vec = item.fields.clone().into_iter().collect::<Vec<Field>>();
        let mut fields = quote!();
        let generics;
        if self.error {
            generics = quote!();
            fields.extend(quote!(error: String,));
        } else {
            let g = item.generics.clone();
            generics = quote!(#g);
            for field in fields_vec {
                fields.extend(quote!(#field,));
            }
        };
        let derive = self.derive();
        let mut attrs = quote!();
        for attr in item.attrs.iter() {
            attrs.extend(quote!(#attr));
        }
        let new_item = quote! {
            #attrs
            #[derive(#derive)]
            #vis struct #ident #generics {
                #[serde(skip_serializing)]
                __status: u16,
                #fields
            }
        };
        *item = syn::parse2(new_item.clone())
            .map_err(|e| syn::Error::new(e.span(), format!("{}\n\n{}", e, new_item.to_string())))?;
        Ok(())
    }

    /// Generate implementation.
    fn gen_impl(&self, item: &ItemStruct) -> proc_macro2::TokenStream {
        let ident = &item.ident;
        let vis = &item.vis;
        let status = self.status;
        let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
        if self.error {
            quote! {
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.error)
                    }
                }
                impl #ident {
                    /// Create a new #ident.
                    #vis fn new<T: ToString>(e: T) -> Self {
                        Self {
                            __status: #status,
                            error: e.to_string(),
                        }
                    }
                    /// Create http response.
                    #vis fn response(self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
                        ::actix_web::HttpResponse::build(::actix_web::http::StatusCode::from_u16(self.__status).unwrap())
                            .json(self)
                    }
                    /// Wrap #ident in a `Result`.
                    #vis fn wrap<T>(self) -> Result<T, Self> {
                        Err(self)
                    }
                    /// Change the status code.
                    #vis fn status(mut self, status: u16) -> Self {
                        self.__status = status;
                        self
                    }
                }
                impl ::actix_web::ResponseError for #ident {
                    fn status_code(&self) -> ::actix_web::http::StatusCode {
                        ::actix_web::http::StatusCode::from_u16(self.__status).unwrap()
                    }
                    fn error_response(&self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
                        ::actix_web::HttpResponse::build(self.status_code())
                            .json(self)
                    }
                }
            }
        } else {
            let mut new = quote!();
            let mut new_args = quote!();
            let mut comma = false;
            for field in item.fields.iter() {
                let ident = field.ident.clone();
                let ty = field.ty.clone();
                if let Some(ident) = ident {
                    if comma {
                        new.extend(quote!(,));
                        new_args.extend(quote!(,));
                    }
                    new.extend(quote!(#ident: #ident));
                    new_args.extend(quote!(#ident: #ty));
                    if !comma {
                        comma = true;
                    }
                }
            }
            quote! {
                impl #impl_generics #ident #ty_generics #where_clause {
                    /// Create a new #ident.
                    #vis fn new(#new_args) -> Self {
                        Self {
                            __status: #status,
                            #new,
                        }
                    }
                    /// Create http response.
                    #vis fn response(self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
                        ::actix_web::HttpResponse::build(::actix_web::http::StatusCode::from_u16(self.__status).unwrap())
                            .json(self)
                    }
                    /// Wrap #ident in a `Result`.
                    #vis fn wrap<T>(self) -> Result<Self, T> {
                        Ok(self)
                    }
                    /// Wrap vector of #ident in a `Result`.
                    #vis fn wrap_vec<T>(v: Vec<#ident #ty_generics>, status: Option<u16>) -> Result<::actix_web::HttpResponse<::actix_web::body::BoxBody>, T> {
                        let status = status.unwrap_or(#status);
                        Ok(::actix_web::HttpResponse::build(::actix_web::http::StatusCode::from_u16(status).unwrap()).json(v))
                    }
                    /// Change the status code.
                    #vis fn status(mut self, status: u16) -> Self {
                        self.__status = status;
                        self
                    }
                }
                impl #impl_generics ::actix_web::Responder for #ident #ty_generics #where_clause {
                    type Body = ::actix_web::body::BoxBody;
                    fn respond_to(self, _req: &::actix_web::HttpRequest) -> ::actix_web::HttpResponse<Self::Body> {
                        self.response()
                    }
                }
            }
        }
    }

    /// Generate code from token stream.
    pub fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
        let attr = syn::parse_macro_input!(attr as Response);
        let mut item = syn::parse_macro_input!(item as ItemStruct);
        #[cfg(not(feature = "doc"))]
        attr.clear(&mut item);
        let imp = attr.gen_impl(&item);
        match attr.rebuild(&mut item) {
            Ok(_) => {}
            Err(e) => return e.to_compile_error().into(),
        }
        quote! {
            #[derive(Clone)]
            #item
            #imp
        }
        .into()
    }
}

impl Parse for Response {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut error = false;
        let mut status = 200;
        if !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            let ident_str = &*ident.to_string();
            match ident_str {
                "error" => {
                    error = true;
                    input.parse::<syn::token::Colon>()?;
                    let s = input.parse::<syn::LitInt>()?;
                    status = s.base10_parse()?;
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown response type {}", ident_str),
                    ));
                }
            }
        }
        Ok(Self { error, status })
    }
}
