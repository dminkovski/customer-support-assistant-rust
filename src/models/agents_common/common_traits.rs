use crate::models::{ai::chatgpt::Message, general::support_case::SupportCase};

use super::common_agent::{AgentState, CommonAgent};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;

pub trait CommonTrait {
    fn new(role: String, objective: String) -> Self;
    fn update_state(&mut self, new_state: AgentState);
    fn get_state(&self) -> &AgentState;
    fn get_role(&self) -> &String;
    fn get_objective(&self) -> &String;
    fn get_memory(&self) -> &Vec<Message>;
}

#[async_trait]
pub trait AgentFunctionTrait: Debug {
    // Execute agent logic
    async fn execute(&mut self, support_case: &mut SupportCase) -> Result<(), Box<dyn Error>>;

    // Coordinator can get common information from agent
    fn get_common_from_agent(&self) -> &CommonAgent;

    // Sets the assistant ID that was created from Azure OpenAI
    fn set_assistant_id(&mut self, id: String);
}
