use anyhow::Result;
use async_trait::async_trait;

use crate::shared::ports::SecurityAnalysisRequest;

#[async_trait]
pub trait AnalysisGateway: Send + Sync {
    async fn analyze(&self, request: SecurityAnalysisRequest) -> Result<String>;
}
