use reqwest::r#async::Client;

use crate::core::requests::{
    get_me::GetMe, send_message::SendMessage, ChatId, RequestContext,
};

pub struct Bot {
    token: String,
    client: Client,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            token: String::from(token),
            client: Client::new(),
        }
    }

    pub fn with_client(token: &str, client: Client) -> Self {
        Bot {
            token: String::from(token),
            client,
        }
    }
}

/// Telegram functions
impl Bot {
    pub fn get_me(&self) -> GetMe {
        GetMe::new(RequestContext {
            token: &self.token,
            client: &self.client,
        })
    }

    pub fn send_message<C, T>(&self, chat_id: C, text: T) -> SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(
            RequestContext {
                token: &self.token,
                client: &self.client,
            },
            chat_id.into(),
            text.into(),
        )
    }
}