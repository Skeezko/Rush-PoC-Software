use std::fmt::Debug;

use crate::http_client::{ApiResponse, HttpMethod};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTab {
    Body,
    Headers,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    MethodSelected(HttpMethod),
    HeaderKeyChanged(usize, String),
    HeaderValueChanged(usize, String),
    AddHeader,
    RemoveHeader(usize),
    BodyChanged(String),
    SendRequest,
    RequestCompleted(Result<ApiResponse, String>),
    TabChanged(ResponseTab),
    SelectHistory(usize),
    ClearHistory,
}
