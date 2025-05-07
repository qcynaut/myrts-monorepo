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
use quote::quote;

/// Create a new myrts protocol services.
#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let evt = syn::parse_macro_input!(attr as syn::LitStr);
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    let ident = &item.sig.ident;
    quote! {
        #[allow(non_camel_case_types, missing_docs)]
        pub struct #ident;

        impl ::proto::app::EventServiceFactory for #ident {
            fn register(self, s: &mut ::proto::app::AppService) {
                #item
                s.register(#evt, #ident);
            }
        }
    }
    .into()
}
