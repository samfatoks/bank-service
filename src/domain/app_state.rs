use crate::{core::QldbProcessor, util::Config, AppError};

#[derive(Clone)]
pub struct AppState {
    pub processor: QldbProcessor,
}

impl AppState {
    pub async fn new(config: Config) -> Result<AppState, AppError> {
        let processor = QldbProcessor::new(config.ledger_name, config.session_pool_size).await?;
        Ok(AppState { processor })
    }
}
