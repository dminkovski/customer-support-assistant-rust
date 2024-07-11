use async_trait::async_trait;

use crate::helpers::command_line::CLIPrint;
use crate::helpers::request::{ai_request, prepare_message};
use crate::models::agents_common::common_agent::{AgentState, CommonAgent};
use crate::models::agents_common::common_traits::{AgentFunctionTrait, CommonTrait};
use crate::models::ai::chatgpt::Message;
use crate::models::general::support_case::{self, SupportCase};
use std::error::Error;

#[derive(Debug)]
pub struct AgentCustomerQuery {
    common: CommonAgent,
}

impl AgentCustomerQuery {
    pub fn new() -> Self {
        let common = CommonAgent::new(
            "Customer Support".to_string(),
            "You are a receptionist or assistant. 
            You handle incoming customer queries and provide immediate responses before continuing to work with your customer support team.
            IMPORTANT: You do not ask any follow up questions. No questions at all.".to_string(),
        );
        Self { common }
    }

    async fn handle_initial_query(&mut self, support_case: &mut SupportCase) {
        self.common.update_state(AgentState::Working);
        let query: &str = &support_case.customer_query;
        let msg: Message =
            prepare_message(&self.common.objective, &support_case.support_context, query);
        support_case.trace.push(msg.clone());
        let result: Result<String, Box<dyn Error + Send>> = ai_request(msg).await;
        support_case.updated();
        if let Ok(response) = result {
            support_case.support_response = Some(response.clone());
            support_case.trace.push(Message {
                role: "assistant".to_string(),
                content: response,
            });
            self.common.update_state(AgentState::Finished);
        } else {
            support_case.support_response = Some(result.unwrap_err().to_string());
            self.common.update_state(AgentState::Error);
        };
    }
}
#[async_trait]
impl AgentFunctionTrait for AgentCustomerQuery {
    async fn execute(&mut self, support_case: &mut SupportCase) -> Result<(), Box<dyn Error>> {
        while self.common.state != AgentState::Finished {
            match self.common.state {
                AgentState::Waiting => {
                    CLIPrint::Info.out(&self.common.role, format!("Handling initial query: {}", &support_case.customer_query).as_str());
                    self.handle_initial_query(support_case).await;
                }
                AgentState::Error => {
                    CLIPrint::Error.out(
                        &self.common.role,
                        format!(
                            "There was an error: {:?}",
                            Some(&support_case.support_response)
                        )
                        .as_str(),
                    );
                    self.common.state = AgentState::Finished;
                }
                _ => {
                    self.common.state = AgentState::Finished;
                }
            }
        }

        CLIPrint::Default.out(
            &self.common.role,
            format!("{}", support_case.support_response.as_ref().unwrap()).as_str(),
        );

        Ok(())
    }

    fn get_common_from_agent(&self) -> &CommonAgent {
        &self.common
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_query_agent() {
        let mut agent: AgentCustomerQuery = AgentCustomerQuery::new();

        let mut support_case: SupportCase = SupportCase::new(
            "Working at a very large successful 5 start Hotel group.".to_string(),
            "I don't like this room you gave me.".to_string(),
        );
        let _ = agent.execute(&mut support_case).await;
        dbg!(&support_case);
    }
}
