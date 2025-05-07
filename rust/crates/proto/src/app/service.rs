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

use super::{extractor::Extractor, session::Session};
use crate::error::Result;
use futures_util::Future;
use std::{pin::Pin, sync::Arc};

/// Handler trait abstraction.
pub trait Handler<Args> {
    fn call(&self, args: Args)
        -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>>;
}

/// Protocol service handler.
pub struct AppService {
    services: Vec<(
        String,
        Box<
            dyn Fn(Session) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>>
                + Send
                + Sync
                + 'static,
        >,
    )>,
}

impl AppService {
    /// Create a new `AppService`.
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    /// Handle incoming events.
    pub fn handle(
        &self,
        event: String,
        sess: Session,
    ) -> Option<Pin<Box<(dyn Future<Output = Result<()>> + Send + Sync + 'static)>>> {
        let service = self.services.iter().find(|(name, _)| name == &event);
        if let Some((_, f)) = service {
            log::debug!("Handling event: {}", event);
            Some(f(sess))
        } else {
            log::debug!("No handler for event: {}", event);
            None
        }
    }

    /// Register event handler.
    pub fn register<F, Args>(&mut self, event: &str, f: F)
    where
        F: Handler<Args> + Send + Sync + 'static,
        Args: Extractor + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        self.services.push((
            event.to_string(),
            Box::new(move |sess| {
                let sess = sess.clone();
                let f = f.clone();
                match Args::extract(&sess) {
                    Ok(args) => f.call(args),
                    Err(e) => Box::pin(async move { Err(e) }),
                }
            }),
        ));
    }
}

pub trait EventServiceFactory {
    fn register(self, s: &mut AppService);
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Res, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Res,
        Res: Future<Output = Result<()>> + Send + Sync + 'static,
        $($param: Extractor + Send + Sync + 'static,)*

    {
        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>> {
            Box::pin(self($($param,)*))
        }
    }
});

factory_tuple! {}
factory_tuple! {A}
factory_tuple! {A B}
factory_tuple! {A B C}
factory_tuple! {A B C D}
factory_tuple! {A B C D E}
factory_tuple! {A B C D E F}
factory_tuple! {A B C D E F G}
factory_tuple! {A B C D E F G H}
factory_tuple! {A B C D E F G H I}
factory_tuple! {A B C D E F G H I J}
