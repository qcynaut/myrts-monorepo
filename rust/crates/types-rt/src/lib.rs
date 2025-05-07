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

//! # types-rt
//!
//! The proc-macro to generate types for serde, utoipa, and diesel.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse::Parse, ItemStruct, LitBool, LitInt};

/// DbKind.
enum DbKind {
    Query,
    Insert,
    Update,
}

impl Parse for DbKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match &*ident.to_string() {
            "Query" => Ok(DbKind::Query),
            "Insert" => Ok(DbKind::Insert),
            "Update" => Ok(DbKind::Update),
            _ => Err(syn::Error::new(
                ident.span(),
                format!("Unknown attribute: {}", ident),
            )),
        }
    }
}

impl ToTokens for DbKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DbKind::Query => tokens.extend(quote!(Queryable, Selectable, Identifiable)),
            DbKind::Insert => tokens.extend(quote!(Insertable)),
            DbKind::Update => tokens.extend(quote!(AsChangeset)),
        }
    }
}

/// Db.
/// The proc-macro parser for `ty`.
struct Db {
    kind: DbKind,
    table: Ident,
    relations: Vec<Ident>,
}

impl Parse for Db {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut kind = None;
        let mut table = None;
        let mut relations = Vec::new();
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match &*ident.to_string() {
                "kind" => {
                    input.parse::<syn::token::Colon>()?;
                    kind = Some(input.parse()?);
                }
                "table" => {
                    input.parse::<syn::token::Colon>()?;
                    table = Some(input.parse()?);
                }
                "relations" => {
                    input.parse::<syn::token::Colon>()?;
                    let item;
                    syn::bracketed!(item in input);
                    while !item.is_empty() {
                        let ident: Ident = item.parse()?;
                        relations.push(ident);
                        if !item.is_empty() {
                            item.parse::<syn::token::Comma>()?;
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown attribute: {}", ident),
                    ))
                }
            }
            if !input.is_empty() {
                input.parse::<syn::token::Comma>()?;
            }
        }
        Ok(Db {
            kind: kind.unwrap(),
            table: table.unwrap(),
            relations,
        })
    }
}

/// WebKind.
enum WebKind {
    Request {
        form: bool,
    },
    Response {
        error: Option<u16>,
        pagination: bool,
    },
}

impl Parse for WebKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match &*ident.to_string() {
            "Request" => {
                if !input.is_empty() {
                    let item;
                    syn::parenthesized!(item in input);
                    let expect_form: Ident = item.parse()?;
                    if &*expect_form.to_string() != "form" {
                        return Err(syn::Error::new(
                            expect_form.span(),
                            format!("Unknown attribute: {}", expect_form),
                        ));
                    }
                    Ok(WebKind::Request { form: true })
                } else {
                    Ok(WebKind::Request { form: false })
                }
            }
            "Response" => {
                if !input.is_empty() {
                    let item;
                    syn::parenthesized!(item in input);
                    let ident: Ident = item.parse()?;
                    let mut error = None;
                    let mut paginated = false;
                    while !item.is_empty() {
                        match &*ident.to_string() {
                            "error" => {
                                item.parse::<syn::token::Colon>()?;
                                let code: LitInt = item.parse()?;
                                error = Some(code.base10_parse()?);
                            }
                            "pagination" => {
                                item.parse::<syn::token::Colon>()?;
                                let code: LitBool = item.parse()?;
                                paginated = code.value;
                            }
                            _ => {
                                return Err(syn::Error::new(
                                    ident.span(),
                                    format!("Unknown attribute: {}", ident),
                                ))
                            }
                        }
                        if !item.is_empty() {
                            item.parse::<syn::token::Comma>()?;
                        }
                    }
                    Ok(WebKind::Response {
                        error,
                        pagination: paginated,
                    })
                } else {
                    Ok(WebKind::Response {
                        error: None,
                        pagination: false,
                    })
                }
            }
            _ => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("Unknown attribute: {}", ident),
                ))
            }
        }
    }
}

impl ToTokens for WebKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            WebKind::Request { form } => {
                if *form {
                    tokens.extend(quote!(#[api_rt::request(form)]))
                } else {
                    tokens.extend(quote!(#[api_rt::request]))
                }
            }
            WebKind::Response {
                error,
                pagination: _,
            } => {
                if let Some(error) = error {
                    tokens.extend(quote!(#[api_rt::response(error: #error)]))
                } else {
                    tokens.extend(quote!(#[api_rt::response]))
                }
            }
        }
    }
}

/// Web.
/// The proc-macro parser for `ty`.
struct Web {
    kind: WebKind,
}

impl Parse for Web {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Web {
            kind: input.parse()?,
        })
    }
}

impl ToTokens for Web {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.kind.to_tokens(tokens)
    }
}

/// Ty.
/// The proc-macro parser for `ty`.
struct Ty {
    db: Option<Db>,
    web: Option<Web>,
    web_only: bool,
}

impl Parse for Ty {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut db = None;
        let mut web = None;
        let mut web_only = true;
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match &*ident.to_string() {
                "db" => {
                    let item;
                    syn::parenthesized!(item in input);
                    db = Some(item.parse()?);
                    web_only = false;
                }
                "web" => {
                    let item;
                    syn::parenthesized!(item in input);
                    web = Some(item.parse()?);
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown attribute: {}", ident),
                    ))
                }
            }
            if !input.is_empty() {
                input.parse::<syn::token::Comma>()?;
            }
        }
        Ok(Ty { db, web, web_only })
    }
}

impl Ty {
    /// Create paginated type.
    fn paginate(item: ItemStruct) -> proc_macro2::TokenStream {
        let item_ident = &item.ident;
        let vis = item.vis;
        let ident = Ident::new(&format!("Paginated{}", &item.ident), item.ident.span());
        quote! {
            #[api_rt::response]
            #vis struct #ident {
                pub items: Vec<#item_ident>,
                pub total: i64,
                pub current_page: i64,
                pub total_page: i64,
                pub next: Option<String>,
                pub prev: Option<String>,
            }
        }
    }

    /// Generate the code.
    fn gen(attr: TokenStream, item: TokenStream) -> TokenStream {
        let attr = syn::parse_macro_input!(attr as Ty);
        let item = syn::parse_macro_input!(item as ItemStruct);
        let mut derive = quote!();
        let mut attrs = quote!();
        #[cfg(feature = "db")]
        if let Some(db) = &attr.db {
            let kind = db.kind.to_token_stream();
            let table_name = db.table.to_token_stream();
            derive.extend(kind);
            attrs.extend(quote!(#[diesel(table_name = #table_name)]));
            let mut assoc = false;
            for relation in &db.relations {
                if !assoc {
                    assoc = true;
                }
                attrs.extend(quote!(#[diesel(belongs_to(#relation))]));
            }
            if assoc {
                derive.extend(quote!(,Associations));
            }
        }
        let mut web = quote!();
        #[cfg(feature = "web")]
        if let Some(w) = &attr.web {
            match w.kind {
                WebKind::Request { form: _ } => attrs.extend(w.kind.to_token_stream()),
                WebKind::Response {
                    error: _,
                    pagination,
                } => {
                    if attr.web_only {
                        attrs.extend(w.kind.to_token_stream());
                        if pagination {
                            web.extend(Self::paginate(item.clone()));
                        }
                    } else {
                        let mut item_clone = item.clone();
                        let current_ident = item_clone.ident.clone();
                        let new_ident =
                            Ident::new(&format!("{}Response", current_ident), item.ident.span());
                        item_clone.ident = new_ident.clone();
                        let attr = w.kind.to_token_stream();
                        let mut form_arg = quote!();
                        let mut need_comma = false;
                        for field in &item.fields {
                            if need_comma {
                                form_arg.extend(quote!(,));
                            }
                            if let Some(field) = &field.ident {
                                form_arg.extend(quote!(item.#field));
                            }
                            need_comma = true;
                        }
                        web = quote!(
                            #attr
                            #item_clone
                            impl From<#current_ident> for #new_ident {
                                fn from(item: #current_ident) -> #new_ident {
                                    #new_ident::new(#form_arg)
                                }
                            }
                        );
                        if pagination {
                            web.extend(Self::paginate(item_clone));
                        }
                    }
                }
            }
        }
        quote!(
            #[derive(#derive)]
            #attrs
            #item
            #web
        )
        .into()
    }
}

/// Types.
/// Create a new type.
/// # Example
/// ```ignore
/// #[types_rt::ty(db(kind: Query, table: users), web(kind: Request))]
/// pub struct MyType;
/// ```
#[proc_macro_attribute]
pub fn ty(attr: TokenStream, item: TokenStream) -> TokenStream {
    Ty::gen(attr, item)
}
