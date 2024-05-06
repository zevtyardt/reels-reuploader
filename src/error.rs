use std::{fmt::Debug, sync::Arc};

use futures::future::BoxFuture;
use teloxide::error_handlers::ErrorHandler;

pub struct CustomLoggingErrorHandler {
    text: String,
}

impl CustomLoggingErrorHandler {
    #[must_use]
    pub fn with_custom_text<T>(text: T) -> Arc<Self>
    where
        T: Into<String>,
    {
        Arc::new(Self { text: text.into() })
    }

    #[must_use]
    pub fn new() -> Arc<Self> {
        Self::with_custom_text("Error".to_owned())
    }
}

impl<E> ErrorHandler<E> for CustomLoggingErrorHandler
where
    E: Debug,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        paris::error!("{text}: {:?}", error, text = self.text);
        Box::pin(async {})
    }
}
