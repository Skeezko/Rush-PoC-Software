use crate::http_client::{send_request, ApiResponse, HttpMethod, Request};
use crate::message::{Message, ResponseTab};

use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

#[derive(Debug, Clone)]
pub struct HistoryItem {
    pub method: HttpMethod,
    pub url: String,
    pub request_headers: Vec<(String, String)>,
    pub request_body: String,
    pub response: Option<Result<ApiResponse, String>>, 
}

#[derive(Debug, Clone)]
pub struct Rustman {
    pub url: String,
    pub method: HttpMethod,
    pub header: Vec<(String, String)>,
    pub body: String,
    pub response: Option<Result<ApiResponse, String>>,
    pub is_loading: bool,
    pub active_tab: ResponseTab,

    pub history: Vec<HistoryItem>,
    pub selected_history: Option<usize>,
}

impl Default for Rustman {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: HttpMethod::GET,
            header: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: String::new(),
            response: None,
            is_loading: false,
            active_tab: ResponseTab::Body,

            history: Vec::new(),
            selected_history: None,
        }
    }
}

impl Rustman {
    pub fn run_app() -> iced::Result {
        <Self as Application>::run(Settings::default())
    }
}

impl Application for Rustman {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Rustman".to_string()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UrlChanged(url) => self.url = url,
            Message::MethodSelected(method) => self.method = method,

            Message::HeaderKeyChanged(i, key) => {
                if let Some(p) = self.header.get_mut(i) {
                    p.0 = key;
                }
            }
            Message::HeaderValueChanged(i, value) => {
                if let Some(p) = self.header.get_mut(i) {
                    p.1 = value;
                }
            }

            Message::AddHeader => self.header.push((String::new(), String::new())),
            Message::RemoveHeader(i) => {
                if i < self.header.len() {
                    self.header.remove(i);
                }
            }

            Message::BodyChanged(body) => self.body = body,

            Message::SendRequest => {
                self.is_loading = true;

                self.history.insert(
                    0,
                    HistoryItem {
                        method: self.method,
                        url: self.url.clone(),
                        request_headers: self.header.clone(),
                        request_body: self.body.clone(),
                        response: None,
                    },
                );
                self.selected_history = Some(0);

                let req = Request {
                    url: self.url.clone(),
                    method: self.method,
                    headers: self.header.clone(),
                    body: (!self.body.trim().is_empty()).then_some(self.body.clone()),
                };

                return Command::perform(send_request(req), Message::RequestCompleted);
            }

            Message::RequestCompleted(res) => {
                self.is_loading = false;

                self.response = Some(res.clone());

                if let Some(item) = self.history.get_mut(0) {
                    item.response = Some(res);
                }
            }

            Message::TabChanged(tab) => self.active_tab = tab,

            Message::SelectHistory(i) => {
                if let Some(item) = self.history.get(i).cloned() {
                    self.selected_history = Some(i);

                    self.method = item.method;
                    self.url = item.url;
                    self.header = item.request_headers;
                    self.body = item.request_body;

                    self.response = item.response;
                }
            }

            Message::ClearHistory => {
                self.history.clear();
                self.selected_history = None;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        crate::view::view(self)
    }
}
