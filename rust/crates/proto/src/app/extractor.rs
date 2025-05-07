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

use super::session::Session;
use crate::error::Result;

/// Extractor for data.
pub trait Extractor: Sized {
    fn extract(sess: &Session) -> Result<Self>;
}

#[doc(hidden)]
#[allow(non_snake_case)]
mod tuple_from_req {
    use super::*;

    macro_rules! tuple_from_req {
        ($fut: ident; $($T: ident),*) => {
            /// Extractor implementation for tuple
            #[allow(unused_parens)]
            impl<$($T: Extractor + Send + Sync + 'static),+> Extractor for ($($T,)+)
            {
                fn extract(sess: &Session) -> Result<Self> {
                    let res = ($($T::extract(sess)?,)+);
                    Ok(res)
                }
            }
        };
    }

    impl Extractor for () {
        fn extract(_: &Session) -> Result<Self> {
            Ok(())
        }
    }

    tuple_from_req! { TupleExtractor1; A }
    tuple_from_req! { TupleExtractor2; A, B }
    tuple_from_req! { TupleExtractor3; A, B, C }
    tuple_from_req! { TupleExtractor4; A, B, C, D }
    tuple_from_req! { TupleExtractor5; A, B, C, D, E }
    tuple_from_req! { TupleExtractor6; A, B, C, D, E, F }
    tuple_from_req! { TupleExtractor7; A, B, C, D, E, F, G }
    tuple_from_req! { TupleExtractor8; A, B, C, D, E, F, G, H }
    tuple_from_req! { TupleExtractor9; A, B, C, D, E, F, G, H, I }
    tuple_from_req! { TupleExtractor10; A, B, C, D, E, F, G, H, I, J }
}
