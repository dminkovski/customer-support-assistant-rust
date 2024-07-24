use async_trait::async_trait;

use crate::helpers::command_line::CLIPrint;
use crate::helpers::request::{ai_request, prepare_message};
use crate::models::agents_common::common_agent::{AgentState, CommonAgent};
use crate::models::agents_common::common_traits::{AgentFunctionTrait, CommonTrait};
use crate::models::ai::chatgpt::Message;
use crate::models::general::support_case::{self, SupportCase};
use std::default;
use std::error::Error;

#[derive(Debug)]
pub struct AgentIntent {
    common: CommonAgent,
}

impl AgentIntent {
    pub fn new() -> Self {
        let common = CommonAgent::new(
            String::from("Intent Classifier"),
            String::from(
                r#"
  You are a personal customer support assistant who helps customers manage their inquiries based on the context you are working in. 
  The user may have various concerns or questions that fall into specific categories. 
  Based on the conversation, you need to identify the user intent. 
  The available intents are: 'Technical Support', 'Account Management', 'Billing and Payments', 'Product Information', 'Order Management', 'Customer Feedback and Complaints', 'Training and Onboarding', 'Warranty and Repairs', 'Customer Retention', 'Security Concerns'. 
  If none of the intents are identified, provide the user with the list of the available intents. If an intent is identified, return the output in text format as shown below:
  'Technical Support'


  If you don't understand or if an intent is not identified, be polite and friendly to the user, ask clarifying questions also using the list of the available intents. 
  IMPORTANT: Don't add any comments in the output or other characters, just use a valid text / string format.
  
  Here are some example scenarios and classifications:
    'Technical Support': 'Assist customers with software installation, troubleshooting errors, resolving bugs, guiding customers through troubleshooting steps for malfunctioning hardware, and helping with internet connectivity issues.',
    'Account Management': 'Help customers with password resets, updating personal information, and managing subscription upgrades, downgrades, cancellations, and renewals.',
    'Billing and Payments': 'Clarify charges on bills or invoices, assist with payment failures, process refunds, resolve billing disputes, explain promotions and discounts, and handle related queries.',
    'Product Information': 'Provide detailed information about products or services to potential customers, explain specific features or functionalities of a product, and inform about product availability and shipping details.',
    'Order Management': 'Provide updates on the status of an order or shipment, guide customers through the return or exchange process, and assist in modifying or canceling orders before they are shipped.',
    'Customer Feedback and Complaints': 'Address customer complaints, provide solutions to ensure satisfaction, gather customer feedback, and handle situations requiring escalation to higher-level support or management.',
    'Training and Onboarding': 'Provide training sessions or resources to help customers understand how to use a product or service, and assist new customers with setup and initial configuration.',
    'Warranty and Repairs': 'Assist customers with filing warranty claims, explain warranty coverage, coordinate repair services for damaged or faulty products, and provide status updates on repair progress.',
    'Customer Retention': 'Address concerns or issues that might lead to customer churn, offer solutions to retain customers, and explain loyalty program benefits.',
    'Security Concerns': 'Assist customers with potential fraud cases or unauthorized account access, and provide advice on securing accounts and protecting personal information.'
  "#,
            ),
        );
        Self { common }
    }

    pub async fn classify_intent(&mut self, support_case: &mut SupportCase) {
        self.common.update_state(AgentState::Working);
        let query: &str = &support_case.customer_query;
        let msg: Message =
            prepare_message(&self.common.objective, &support_case.support_context, query);
        support_case.trace.push(msg.clone());
        let result: Result<String, Box<dyn Error + Send>> = ai_request(msg).await;
        support_case.updated();
        if let Ok(response) = result {
            support_case.intent_category = Some(response.clone());
            self.common.update_state(AgentState::Finished);
        } else {
            support_case.support_response = Some(result.unwrap_err().to_string());
            self.common.update_state(AgentState::Error);
        };
    }
}

impl Default for AgentIntent {
    fn default() -> Self {
        AgentIntent::new()
    }
}

#[async_trait]
impl AgentFunctionTrait for AgentIntent {
    async fn execute(&mut self, support_case: &mut SupportCase) -> Result<(), Box<dyn Error>> {
        while self.common.state != AgentState::Finished {
            match self.common.state {
                AgentState::Waiting => {
                    CLIPrint::Info.out(
                        &self.common.role,
                        format!("Analyzing intent from: {}", &support_case.customer_query).as_str(),
                    );
                    self.classify_intent(support_case).await;
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

        CLIPrint::Info.out(
            &self.common.role,
            format!(
                "Intent classified as: {}",
                support_case.intent_category.as_ref().unwrap()
            )
            .as_str(),
        );
        self.common.state = AgentState::Waiting;
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
    async fn tests_intent_agent() {
        let context = String::from("E-Commerce Website");
        let query: String = String::from(
            "I cannot believe you are selling this product. Its really bad. I want my money back.",
        );

        let mut agent_intent = AgentIntent::new();
        let mut support_case = SupportCase::new(context, query);
        agent_intent.execute(&mut support_case).await;
        assert_eq!(
            support_case.intent_category.as_ref().unwrap(),
            "Customer Feedback and Complaints"
        )
    }
}
