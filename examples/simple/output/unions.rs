// DO NOT EDIT THIS FILE
// This file was generated by https://github.com/tacogips/async-graphql-reverse
use super::objects::Friend;
use super::objects::Notification;
use async_graphql::*;
#[derive(Union, Debug, Clone)]
pub enum SearchResult {
    Friend(Friend),
    Notification(Notification),
}
