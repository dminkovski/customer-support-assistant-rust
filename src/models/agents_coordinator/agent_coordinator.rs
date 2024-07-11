use crate::models::agents::agent_escalation::AgentEscalation;
use crate::models::agents::{agent_query::AgentCustomerQuery, agent_sentiment::AgentSentiment};
use crate::models::agents_common::{
    common_agent::CommonAgent,
    common_traits::{AgentFunctionTrait, CommonTrait},
};
use crate::models::general::support_case::SupportCase;

pub struct CoordinatorAgent {
    common: CommonAgent,
    support_case: SupportCase,
    agents: Vec<Box<dyn AgentFunctionTrait>>,
}

impl CoordinatorAgent {
    pub fn new(context: String, query: String) -> Self {
        let support_case = SupportCase::new(context, query);
        let common: CommonAgent = CommonAgent::new(
            "Support Coordinator".to_string(),
            "You are an agent coordinator.".to_string(),
        );

        Self {
            support_case,
            common,
            agents: vec![],
        }
    }
    fn add_agent(&mut self, agent: Box<dyn AgentFunctionTrait>) {
        self.agents.push(agent);
    }
    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentCustomerQuery::new()));
        self.add_agent(Box::new(AgentSentiment::new()));
        self.add_agent(Box::new(AgentEscalation::new()));
    }
    pub async fn handle_support_request(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            agent
                .execute(&mut self.support_case)
                .await
                .expect("Should have executed agent");
        }
    }
}
