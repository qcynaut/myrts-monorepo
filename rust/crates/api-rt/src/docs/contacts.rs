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

/// Contacts.
/// The authors contact information.
pub struct Contact {
    name: Option<String>,
    email: Option<String>,
}

impl Contact {
    /// Load contacts information.
    pub fn load() -> Option<Self> {
        std::env::var("CARGO_PKG_AUTHORS")
            .ok()
            .and_then(|authors| authors.split(":").next().map(|s| s.to_owned()))
            .and_then(|author| {
                let split: Vec<&str> = author.split("<").collect();
                let name = split.get(0).map(|s| s.to_owned().to_owned());
                let email = split.get(1).map(|s| s.replace(">", ""));
                if name.is_none() && email.is_none() {
                    None
                } else {
                    Some(Self { name, email })
                }
            })
    }
}

impl ToTokens for Contact {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = if let Some(name) = &self.name {
            quote!(Some(#name))
        } else {
            quote!(None)
        };
        let email = if let Some(email) = &self.email {
            quote!(Some(#email))
        } else {
            quote!(Some("".to_owned()))
        };
        tokens.extend(quote!(::utoipa::openapi::info::ContactBuilder::new().name(#name).email(#email).build()));
    }
}
