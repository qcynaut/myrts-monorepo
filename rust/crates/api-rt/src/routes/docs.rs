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

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use serde::Deserialize;
use syn::ItemFn;

/// D.
/// The Doc parser.
#[derive(Deserialize)]
struct D {
    tags: Option<Vec<String>>,
    responses: Option<Vec<Response>>,
    request: Option<Request>,
    params: Option<Vec<Param>>,
    #[allow(unused)]
    description: String,
    auth: Option<String>,
}

/// Content.
/// The content documentation for the Route.
#[derive(Deserialize, serde::Serialize)]
pub enum Content {
    Vec(String),
    T(String),
}

/// Transform string into content.
fn string_to_content(content: &str) -> syn::Result<proc_macro2::TokenStream> {
    match content {
        "String" => Ok(quote!(::utoipa::openapi::ObjectBuilder::new()
            .schema_type(::utoipa::openapi::SchemaType::String))),
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => {
            Ok(quote!(::utoipa::openapi::ObjectBuilder::new()
                .schema_type(::utoipa::openapi::SchemaType::Integer)))
        }
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Content must be one of String, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize",
        ))
    }
}

impl Content {
    fn content(&self, content: &str) -> proc_macro2::TokenStream {
        string_to_content(content)
            .unwrap_or(quote!(::utoipa::openapi::Ref::from_schema_name(#content)))
    }
}

impl ToTokens for Content {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Vec(content) => {
                let content = self.content(content);
                tokens
                    .extend(quote!(::utoipa::openapi::schema::ArrayBuilder::new().items(#content)));
            }
            Self::T(content) => {
                tokens.extend(self.content(content));
            }
        }
    }
}

/// Response.
/// The response documentation for the Route.
#[derive(Deserialize)]
pub struct Response {
    description: String,
    status: u16,
    content: Option<Content>,
    content_type: Option<String>,
}

impl ToTokens for Response {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let status = format!("{}", &self.status);
        let description = &self.description;
        let content_type = self
            .content_type
            .clone()
            .unwrap_or("application/json".to_string());
        let mut builder =
            quote!(::utoipa::openapi::ResponseBuilder::new().description(#description));

        if let Some(content) = &self.content {
            builder.extend(quote!(.content(#content_type,::utoipa::openapi::ContentBuilder::new().schema(#content).build())));
        }

        tokens.extend(quote!(.response(#status,#builder .build())));
    }
}

/// Request.
/// The request documentation for the Route.
#[derive(Deserialize)]
pub struct Request {
    description: Option<String>,
    content: Content,
    content_type: Option<String>,
}

impl ToTokens for Request {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let description = &self.description;
        let content_type = self
            .content_type
            .clone()
            .unwrap_or("application/json".to_string());
        let content = &self.content;
        let mut inner = quote!(::utoipa::openapi::request_body::RequestBodyBuilder::new().content(#content_type, ::utoipa::openapi::ContentBuilder::new().schema(#content).build()));
        if let Some(description) = description {
            inner.extend(quote!(.description(Some(#description))));
        }
        tokens.extend(quote!(.request_body(Some(#inner.build()))));
    }
}

/// ParamKind.
/// The param kind documentation for the Route.
#[derive(Deserialize)]
pub enum ParamKind {
    Path(String),
    Query(String),
}

impl ToTokens for ParamKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ParamKind::Path(path) => match string_to_content(&path) {
                Ok(content) => {
                    tokens.extend(quote!(.parameter_in(::utoipa::openapi::path::ParameterIn::Path).schema(Some(#content))));
                }
                Err(e) => {
                    tokens.extend(e.to_compile_error());
                }
            },
            ParamKind::Query(query) => match string_to_content(&query) {
                Ok(content) => {
                    tokens.extend(quote!(.parameter_in(::utoipa::openapi::path::ParameterIn::Query).schema(Some(#content))))
                }
                Err(e) => {
                    tokens.extend(e.to_compile_error());
                }
            },
        }
    }
}

/// Param.
/// The param documentation for the Route.
#[derive(Deserialize)]
pub struct Param {
    name: String,
    kind: ParamKind,
    description: Option<String>,
    required: bool,
}

impl ToTokens for Param {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let kind = &self.kind;
        let description = self
            .description
            .clone()
            .map(|d| quote!(.description(Some(#d))))
            .unwrap_or(quote!());
        let required = if self.required {
            quote!(.required(::utoipa::openapi::Required::True))
        } else {
            quote!(.required(::utoipa::openapi::Required::False))
        };
        tokens.extend(quote!(.parameter(::utoipa::openapi::path::ParameterBuilder::from(::utoipa::openapi::path::Parameter::new(#name))#kind #description #required)));
    }
}

/// Doc.
/// The documentation for te Route.
pub struct Doc {
    ident: Ident,
    path: String,
    method: Ident,
    tags: Vec<String>,
    responses: Vec<Response>,
    request: Option<Request>,
    summary: Option<String>,
    description: String,
    params: Option<Vec<Param>>,
    op: String,
    auth: Option<String>,
}

impl Doc {
    fn new(
        ident: Ident,
        d: D,
        path: String,
        method: Ident,
        summary: Option<String>,
        description: String,
    ) -> Self {
        let new_ident = format!("__doc_{}", &ident.to_string().to_lowercase());
        Self {
            ident: Ident::new(&new_ident, ident.span()),
            tags: d.tags.unwrap_or(vec![]),
            responses: d.responses.unwrap_or(vec![]),
            request: d.request,
            description,
            path,
            method,
            summary,
            params: d.params,
            op: ident.to_string(),
            auth: d.auth,
        }
    }
}

impl ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let path = &self.path;
        let method = &self.method;
        let method_str = method.to_string();
        let method_str = method_str[0..1].to_uppercase() + &method_str[1..];
        let method_ident = Ident::new(&method_str, method.span());
        let op = &self.op;

        let mut responses = quote!(::utoipa::openapi::ResponsesBuilder::new());
        for r in &self.responses {
            responses.extend(quote!(#r));
        }
        if !self.responses.is_empty() {
            responses = quote!(.responses(#responses.build()));
        } else {
            responses = quote!();
        }

        let request = &self.request;

        let mut tags = quote!();
        for tag in &self.tags {
            tags.extend(quote!(Some(#tag),))
        }

        let summary = if let Some(summary) = &self.summary {
            quote!(.summary(Some(#summary)))
        } else {
            quote!()
        };
        let description = {
            let description = &self.description;
            quote!(.description(Some(#description)))
        };

        let mut params = quote!();
        if let Some(pars) = &self.params {
            for p in pars {
                params.extend(quote!(#p));
            }
        }

        let auth = if let Some(auth) = &self.auth {
            quote!(.securities(Some([::utoipa::openapi::security::SecurityRequirement::new::<&str,[&str;0usize],&str>(#auth,[]),])))
        } else {
            quote!()
        };

        tokens.extend(quote! {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            pub struct #ident;

            impl ::utoipa::Path for #ident {
                fn path() -> &'static str {
                    #path
                }
                fn path_item(default_tag: Option<&str>) -> ::utoipa::openapi::path::PathItem {
                    use std::iter::FromIterator;
                    use ::utoipa::openapi::ToArray;
                    ::utoipa::openapi::PathItem::new(::utoipa::openapi::PathItemType::#method_ident,::utoipa::openapi::path::OperationBuilder::new()#request #responses #auth .operation_id(Some(#op))#summary #description #params .tag(*[#tags default_tag, Some("crate")].iter().flatten().find(|t|!t.is_empty()).unwrap()))
                }
            }
        });
    }
}

/// DocParser.
/// The parser for the Doc.
pub struct DocParser<'a> {
    route: &'a mut ItemFn,
    path: &'a str,
    method: &'a Ident,
}

impl<'a> DocParser<'a> {
    pub fn new(route: &'a mut ItemFn, path: &'a str, method: &'a Ident) -> Self {
        Self {
            route,
            path,
            method,
        }
    }

    pub fn parse(self) -> syn::Result<Doc> {
        let mut description = Vec::new();
        let mut doc = Vec::new();
        let attrs = self.route.attrs.clone();
        let mut state = 0;
        let mut start = None;
        let mut summary = None;
        for (i, attr) in attrs.into_iter().enumerate() {
            if attr.path().is_ident("doc") {
                let mut doc_content = attr
                    .meta
                    .require_name_value()?
                    .value
                    .to_token_stream()
                    .to_string();
                doc_content = doc_content.replace("\"", "").to_string();
                if doc_content.starts_with(" ") {
                    doc_content.remove(0);
                }
                if doc_content.starts_with("---") {
                    state = 1;
                    start = Some(i);
                    continue;
                }
                if i == 0 {
                    summary = Some(doc_content.clone());
                }
                if state == 0 {
                    description.push(doc_content);
                    description.push("\n".to_owned());
                } else {
                    doc.push(doc_content);
                }
            }
        }
        if let Some(i) = start {
            self.route.attrs = self.route.attrs[..i].to_vec();
        }
        let doc_str = format!("{}\ndescription: desc", doc.join("\n"));
        let parsed: D = serde_yaml::from_str(&doc_str).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to parse doc: {}\n\n{}", e, &doc_str),
            )
        })?;
        Ok(Doc::new(
            self.route.sig.ident.clone(),
            parsed,
            self.path.to_owned(),
            self.method.to_owned(),
            summary,
            description.join("\n"),
        ))
    }
}
