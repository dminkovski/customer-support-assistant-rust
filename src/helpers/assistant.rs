use std::error::Error;

use crate::{api::assistant_request::create_assistant, models::{agents_common::common_traits::AgentFunctionTrait, ai::assistant::AssistantRequest}};

pub async fn create_assistant_from_agent(role: &str, objective: &str) -> Result<String, Box<dyn Error + Send>>{
  let model: String = dotenv::var("AZURE_OPEN_AI_MODEL_DEPLOYMENT_NAME")
        .expect("AZURE_OPEN_AI_MODEL_DEPLOYMENT_NAME missing from .env");
  let assistant_request = AssistantRequest::new(model.clone(), role.to_string(), objective.to_string());
  let assistant_response = create_assistant(assistant_request).await?;
  Ok(assistant_response.id)
}