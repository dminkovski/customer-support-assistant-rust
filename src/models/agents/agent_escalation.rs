
use async_trait::async_trait;

use crate::helpers::command_line::CLIPrint;
use crate::helpers::request::{ai_request, prepare_message};
use crate::models::agents_common::common_agent::{AgentState, CommonAgent};
use crate::models::agents_common::common_traits::{AgentFunctionTrait, CommonTrait};
use crate::models::ai::chatgpt::Message;
use crate::models::general::support_case::{self, SupportCase};
use std::error::Error;

#[derive(Debug)]
pub struct AgentEscalation {
    pub common: CommonAgent,
    pub potential_resolving_actions: Option<String>
}

const ACTIONS_PROMPT: &str = "You are in charge of customer escalations within Customer Support. 
            You handle incoming customer queries and sentiments and provide resolving actions.
            You will respond with a JSON Format of an Array of ACTIONS to call in different customer support scenarios based on context.
            IMPORTANT: You do not ask any follow up questions. No questions at all.
            EXAMPLE 1:
            Input: 5 Stars Hotel
            Output: ['Change room','Provide discount for bar and snacks','Call mechanic','Call room service']

            EXAMPLE 2:
            Input: Small, medium sized Company
            Output: ['Setup meeting with HR','Setup meeting with Sales','Refund item','Offer discount']
            ";


impl AgentEscalation {
    pub fn new() -> Self {
        let common = CommonAgent::new(
            "Escalation Manager".to_string(),
            "You are an Escalation Manager helping with Customer Support. 
            You handle incoming customer queries and sentiments and decide which of the resolving actions you are provided with to call.
            If you are not sure or believe a representative of upper management should get involved, you will answer with 'upper management'.
            IMPORTANT: You do not ask any follow up questions. No questions at all. You decide on ONE of the provided actions OR 'upper management'.
            VERY IMPORTANT: You provide absolutely NO additional info or reasoning.".to_string(),
        );
        Self { common, potential_resolving_actions: None }
    }

    async fn populate_resolving_actions(&mut self, support_case: &mut SupportCase){
      self.common.update_state(AgentState::Working);
        let query: &str = &support_case.customer_query;
        let msg: Message =
            prepare_message(ACTIONS_PROMPT, &support_case.support_context, query);
        support_case.trace.push(msg.clone());
        let result: Result<String, Box<dyn Error + Send>> = ai_request(msg).await;
        if let Ok(action) = result {
          self.potential_resolving_actions = Some(action);
          self.common.state = AgentState::Working;
        } else {
          self.common.state = AgentState::Error;
        }
    }

    async fn handle_escalation(&mut self, support_case: &mut SupportCase) {
        self.common.update_state(AgentState::Working);
        let query: &str = &support_case.customer_query;
        let msg: Message =
            prepare_message(format!("{} ONLY AVAILABLE ACTIONS:{}",&self.common.objective, &self.potential_resolving_actions.as_ref().unwrap()).as_str(), &support_case.support_context, query);
        support_case.trace.push(msg.clone());
        let result: Result<String, Box<dyn Error + Send>> = ai_request(msg).await;
        match result {
          Ok(action) => {
            match action.as_str() {
              "upper management" => {
                support_case.needs_upper_management_attention = true;
                support_case.escalated = true;
              }
              _ => {
                support_case.support_response = Some(action);
              }
            }
          }
          Err(e) => {
            self.common.state = AgentState::Error;
          }

        }
        support_case.updated();
        self.common.state = AgentState::Finished;
        
    }
}

#[async_trait]
impl AgentFunctionTrait for AgentEscalation {
    async fn execute(&mut self, support_case: &mut SupportCase) -> Result<(), Box<dyn Error>> {
        while self.common.state != AgentState::Finished {
            match self.common.state {
                AgentState::Waiting => {
                    CLIPrint::Info.out(&self.common.role, "Preparing action items...");
                    self.populate_resolving_actions(support_case).await;
                }
                AgentState::Working => {
                  CLIPrint::Info.out(&self.common.role, "Handling escalation...");
                    self.handle_escalation(support_case).await;
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
        if support_case.needs_upper_management_attention && support_case.escalated {
          CLIPrint::Info.out(&self.common.role, "Upper management will be with you shortly");
        } else {
          CLIPrint::Default.out(&self.common.role, format!("Time for: {}", support_case.support_response.as_ref().unwrap()).as_str());
        }

        Ok(())
    }

    fn get_common_from_agent(&self) -> &CommonAgent {
        &self.common
    }
}