use std::collections::HashMap;

use crate::api::assistant_request::{self, create_assistant};
use crate::helpers::command_line::{get_user_response, CLIPrint};
use crate::models::agents::agent_feedback::AgentFeedback;
use crate::models::agents::agent_intent::AgentIntent;
use crate::models::agents_common::{
    common_agent::CommonAgent,
    common_traits::{AgentFunctionTrait, CommonTrait},
};
use crate::models::ai::assistant::AssistantRequest;
use crate::models::general::support_case::{self, SupportCase};

pub struct CoordinatorAgent {
    support_case: SupportCase,
    agents: HashMap<String, Box<dyn AgentFunctionTrait>>,
    intent_agent: Box<dyn AgentFunctionTrait>,
}

impl CoordinatorAgent {
    pub fn new(context: String, query: String) -> Self {
        let support_case = SupportCase::new(context, query);

        Self {
            support_case,
            agents: HashMap::new(),
            intent_agent: Box::new(AgentIntent::new()),
        }
    }
    fn add_agents(&mut self) {
        self.agents.insert(
            String::from("Customer Feedback and Complaints"),
            Box::new(AgentFeedback::new()),
        );
    }

    pub async fn get_intent(&mut self) {
        self.intent_agent
            .as_mut()
            .execute(&mut self.support_case)
            .await
            .expect("should have classified intent");
    }

    pub async fn handle_support_request(&mut self) {
        self.add_agents();
        self.get_intent().await;
        let intent = self
            .support_case
            .intent_category
            .as_ref()
            .unwrap()
            .to_string();

        loop {
            let agent = self
                .agents
                .get_mut(&intent)
                .expect("should have been agent");            
            agent
                .execute(&mut self.support_case)
                .await
                .expect("should have executed agent");
            self.support_case.customer_query = get_user_response("");
            if self.support_case.customer_query.contains("quit") {
                break;
            }
        }
    }
}
