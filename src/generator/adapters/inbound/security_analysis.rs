use anyhow::Result;
use async_trait::async_trait;

use crate::generator::ports::outbound::AnalysisGateway;
use crate::shared::ports::{SecurityAnalysisPort, SecurityAnalysisRequest};

pub struct GeneratorSecurityAnalysis {
    gateway: Box<dyn AnalysisGateway>,
}

impl GeneratorSecurityAnalysis {
    pub fn new(gateway: Box<dyn AnalysisGateway>) -> Self {
        Self { gateway }
    }
}

#[async_trait]
impl SecurityAnalysisPort for GeneratorSecurityAnalysis {
    async fn analyze(&self, request: SecurityAnalysisRequest) -> Result<String> {
        self.gateway.analyze(request).await
    }
}
