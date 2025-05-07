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

use quote::{quote, ToTokens};

use super::contacts::Contact;

/// Info.
/// The info about the API.
pub struct Info {
    title: String,
    version: String,
    description: Option<String>,
    license: String,
    contact: Option<Contact>,
}

impl Info {
    /// Load info.
    pub fn load(provided_description: Option<String>) -> Self {
        let title = std::env::var("CARGO_PKG_NAME").unwrap();
        let version = std::env::var("CARGO_PKG_VERSION").unwrap();
        let description = match provided_description {
            Some(desc) => Some(desc),
            None => std::env::var("CARGO_PKG_DESCRIPTION").ok(),
        };
        let license = std::env::var("CARGO_PKG_LICENSE").unwrap_or_default();
        let contact = Contact::load();
        Self {
            title,
            version,
            description,
            license,
            contact,
        }
    }
}

impl ToTokens for Info {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let title = &self.title;
        let version = &self.version;
        let description = if let Some(desc) = &self.description {
            quote!(Some(#desc))
        } else {
            quote!(None)
        };
        let license = &self.license;
        let contact = if let Some(contact) = &self.contact {
            quote!(Some(#contact))
        } else {
            quote!(None)
        };
        tokens.extend(quote!(::utoipa::openapi::InfoBuilder::new().title(#title).version(#version).description(#description).contact(#contact).license(Some(::utoipa::openapi::LicenseBuilder::new().name(#license).build()))));
    }
}
