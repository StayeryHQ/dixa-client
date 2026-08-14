// use uuid::{Uuid, uuid};
//
// use crate::generated::{
//     Content, CreateConversationInput, CreateEndUserInput, CreateMessageInput, Email, EndUser,
//     HttpClient, Outbound, Text,
// };
//
// mod generated;
//
// const SEND_TO_MAIL: &str = "lucaherzke@gmail.com";
// const MY_UUID: Uuid = uuid!("444b6223-9629-49e0-b0c5-1dcddd50fd86");
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenvy::dotenv()?;
//
//     let api_key =
//         std::env::var("DIXA_API_KEY").map_err(|e| format!("DIXA_API_KEY not privided {e}"))?;
//
//     let client = HttpClient::new().with_api_key(api_key);
//
//     send_mail_to(&client, SEND_TO_MAIL).await?;
//
//     Ok(())
// }
//
// async fn send_mail_to(
//     client: &HttpClient,
//     to_mail: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let send_to_user = match get_first_user_from_email(client, to_mail).await {
//         Ok(user) => {
//             println!("Found user to send the mail to");
//             user
//         }
//         Err(_) => {
//             println!("Needing to create a end user");
//             create_new_user(client, to_mail).await?
//         }
//     };
//
//     client
//         .post_conversations(CreateConversationInput::Email(Email {
//             email_integration_id: "stayery-sandbox-support@email.dixa.io".to_string(),
//             language: None,
//             message: CreateMessageInput::Outbound(Outbound {
//                 agent_id: MY_UUID,
//                 attachments: None,
//                 bcc: None,
//                 cc: None,
//                 content: Content::Text(Text {
//                     value: "Content written in rust".to_string(),
//                 }),
//                 external_id: None,
//                 integration_email: None,
//             }),
//             requester_id: send_to_user.id,
//             subject: "Test from Rust SDK".to_string(),
//         }))
//         .await?;
//
//     Ok(())
// }
//
// async fn create_new_user(
//     client: &HttpClient,
//     email: impl Into<String>,
// ) -> Result<EndUser, Box<dyn std::error::Error>> {
//     let new_user = client
//         .post_endusers(CreateEndUserInput {
//             email: Some(email.into()),
//             ..Default::default()
//         })
//         .await?;
//
//     Ok(new_user.data)
// }
//
// async fn get_first_user_from_email(
//     client: &HttpClient,
//     email: &str,
// ) -> Result<EndUser, Box<dyn std::error::Error>> {
//     let endusers = client
//         .get_endusers(None, None::<&str>, Some(email), None::<&str>, None::<&str>)
//         .await?;
//
//     if let Some(users) = endusers.data {
//         if !users.is_empty() {
//             return Ok(users
//                 .into_iter()
//                 .next()
//                 .expect("Checked if vec is empty before"));
//         }
//     }
//
//     Err(format!("No user found for email: {}", email).into())
// }
