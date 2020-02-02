use serde::Serialize;

use super::BotWrapper;
use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to kick a user from a group, a supergroup or a channel.
///
/// In the case of supergroups and channels, the user will not be able to return
/// to the group on their own using invite links, etc., unless [unbanned] first.
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#kickchatmember).
///
/// [unbanned]: crate::Bot::unban_chat_member
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct KickChatMember<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
    user_id: i32,
    until_date: Option<i32>,
}

#[async_trait::async_trait]
impl Request for KickChatMember<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "kickChatMember",
            &self,
        )
        .await
    }
}

impl<'a> KickChatMember<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
            user_id,
            until_date: None,
        }
    }

    /// Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// Date when the user will be unbanned, unix time.
    ///
    /// If user is banned for more than 366 days or less than 30 seconds from
    /// the current time they are considered to be banned forever.
    pub fn until_date(mut self, val: i32) -> Self {
        self.until_date = Some(val);
        self
    }
}