use async_trait::async_trait;

use crate::api::assistant_request::{create_and_run_thread, create_assistant};
use crate::helpers::assistant::create_assistant_from_agent;
use crate::helpers::command_line::CLIPrint;
use crate::helpers::request::{ai_request, prepare_message};
use crate::models::agents_common::common_agent::{AgentState, CommonAgent};
use crate::models::agents_common::common_traits::{AgentFunctionTrait, CommonTrait};
use crate::models::ai::assistant::AssistantRequest;
use crate::models::ai::chatgpt::Message;
use crate::models::general::support_case::{self, SupportCase};
use std::default;
use std::error::Error;

#[derive(Debug)]
pub struct AgentFeedback {
    common: CommonAgent,
}

impl AgentFeedback {
    pub fn new() -> Self {
        let common = CommonAgent::new(
            String::from("Feedback and Complaints Agent"),
            String::from(
                r#"
  You are a personal customer support assistant who is focused on the specific category: 'Customer Feedback and Complaints'.
  Your job is to give the user a feeling of being heard and understood and to listen to its complaints with empathy.
  If the user wants to know which concrete actions you are taking you shall always let the user know, that you will create a ticket and that you will keep the user updated on the status.
  In case the user agrees with creating a ticket you shall ask for the users email to inform him about the ticket status.
  In the rare occasion that you assess, that the user still is very unhappy and it seems like an important case - you may redirect the user to a human.
  You will be provided with functions that you may call to: Create a ticket and contact a human.
  If you don't understand, be polite and friendly to the user, ask clarifying questions. You never ever make the user feel like he is not understood or taken care of.
  "#,
            ),
        );
        Self { common }
    }

    async fn handle_feedback(&mut self, support_case: &mut SupportCase) {
        self.common.update_state(AgentState::Working);
        let query: &str = &support_case.customer_query;
        let history: &str = &support_case.get_history();
        let send_request: &str = &format!("{} CHAT HISTORY: {}", query, history);
        let msg: Message = prepare_message(
            &self.common.objective,
            &support_case.support_context,
            send_request,
        );
        support_case.trace.push(msg.clone());
        let result: Result<String, Box<dyn Error + Send>> = assistant_message(thread_id, assistant_id, msg).await;
        support_case.updated();
        if let Ok(response) = result {
            support_case.support_response = Some(response.clone());
            self.common.update_state(AgentState::Finished);
        } else {
            support_case.support_response = Some(result.unwrap_err().to_string());
            self.common.update_state(AgentState::Error);
        };
    }
}

impl Default for AgentFeedback {
    fn default() -> Self {
        AgentFeedback::new()
    }
}

#[async_trait]
impl AgentFunctionTrait for AgentFeedback {
    async fn execute(&mut self, support_case: &mut SupportCase) -> Result<(), Box<dyn Error>> {
        if self.common.assistant_id == None {
            CLIPrint::Info.out(&self.common.role, "Initializing assistant...");
            self.common.assistant_id = Some(create_assistant_from_agent(&self.common.role, &self.common.objective).await.expect("should have given assistant id"));
            let thread_run_response = create_and_run_thread(Message {
                role: String::from("user"),
                content: support_case.customer_query.clone()
            }, &self.common.assistant_id.as_ref().unwrap()).await.expect("should have been threadrun response");
            self.common.thread_id = Some(thread_run_response.thread_id.clone());
        }

        while self.common.state != AgentState::Finished {
            match self.common.state {
                AgentState::Waiting => {
                    CLIPrint::Info.out(&self.common.role, "Handling Feedback and Complaints");
                    self.handle_feedback(support_case).await;
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

        self.common.update_state(AgentState::Waiting);
        CLIPrint::Info.out(
            &self.common.role,
            support_case.support_response.as_ref().unwrap(),
        );

        Ok(())
    }

    fn get_common_from_agent(&self) -> &CommonAgent {
        &self.common
    }
    fn set_assistant_id(&mut self, id: String){
        self.common.assistant_id = Some(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_feedback_agent() {}
}
