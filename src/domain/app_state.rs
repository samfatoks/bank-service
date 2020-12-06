use crate::{Config, AppError, core::QldbProcessor};

#[derive(Clone)]
pub struct AppState {
    pub processor: QldbProcessor,
}

impl AppState {
    pub async fn new(config: Config) -> Result<AppState, AppError>  {
        let processor = QldbProcessor::new(config.ledger_name.clone()).await?;
        Ok(AppState { processor })
    }
}