use async_trait::async_trait;

use crate::helpers::command_line::CLIPrint;
use crate::helpers::request::{ai_request, prepare_message};
use crate::models::agents_common::common_agent::{AgentState, CommonAgent};
use crate::models::agents_common::common_traits::{AgentFunctionTrait, CommonTrait};
use crate::models::ai::chatgpt::Message;
use crate::models::general::support_case::{self, SupportCase};
use std::error::Error;

#[derive(Debug)]
pub struct AgentSentiment {
    common: CommonAgent,
}

impl AgentSentiment {
    pub fn new() -> Self {
        let common = CommonAgent::new(
            "Psychologist".to_string(),
            "You are a Psychologist helping out Customer Support. 
            You handle incoming customer queries analyze them and their sentiment into one of the following categories: 'Negative', 'Positive'.
            IMPORTANT: You do not ask any follow up questions. No questions at all.
            VERY IMPORTANT: Your answer is always either 'Positive' or 'Negative'. You provide absolutely NO additional info.".to_string(),
        );
        Self { common }
    }

    async fn analyze_sentiment(&mut self, support_case: &mut SupportCase) {
        self.common.update_state(AgentState::Working);
        let query: &str = &support_case.customer_query;
        let msg: Message =
            prepare_message(&self.common.objective, &support_case.support_context, query);
        support_case.trace.push(msg.clone());
        let result: Result<String, Box<dyn Error + Send>> = ai_request(msg).await;
        support_case.updated();
        if let Ok(response) = result {
            support_case.sentiment = Some(response.clone());
            match response.as_str() {
                "Positive" => {
                    // Thanks
                    CLIPrint::Info.out(
                        &self.common.role,
                        "Sentiment is 'Positive'",
                    );
                }
                "Negative" => {
                    // Escalate
                    CLIPrint::Warning.out(&self.common.role, "Sentiment is 'Negative'");
                    support_case.should_escalate = true;
                }
                _ => {
                    self.common.update_state(AgentState::Error);
                }
            }

            self.common.update_state(AgentState::Finished);
        } else {
            support_case.support_response = Some(result.unwrap_err().to_string());
            self.common.update_state(AgentState::Error);
        };
    }
}

#[async_trait]
impl AgentFunctionTrait for AgentSentiment {
    async fn execute(&mut self, support_case: &mut SupportCase) -> Result<(), Box<dyn Error>> {
        while self.common.state != AgentState::Finished {
            match self.common.state {
                AgentState::Waiting => {
                    CLIPrint::Info.out(&self.common.role, "Analyzing sentiment...");
                    self.analyze_sentiment(support_case).await;
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
    async fn tests_sentiment_agent() {}
}
