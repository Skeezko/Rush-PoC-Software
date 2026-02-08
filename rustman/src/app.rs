use crate::message::{Message, ResponseTab};
use crate::http_client::{HttpMethod, ApiResponse, Request, send_request};
use iced::Theme;
use iced::executor;
use iced::Application;
use iced::Command;
use iced::Element;

pub struct Rustman {
    url: String,
    method: HttpMethod,
    header: Vec<(String, String)>,
    body: String,
    response: Option<Result<ApiResponse, String>>,
    is_loading: bool,
    active_tab: ResponseTab
}


impl Application for Rustman {

    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self{
                url: String::new(),
                method: HttpMethod::GET,
                header: vec![(String::new(), String::new())],
                body: String::new(),
                response: None,
                is_loading: false,
                active_tab: ResponseTab::Body
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Rustman")
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::UrlChanged(url) => {
                self.url = url;
            }
            Message::MethodSelected(method) => {
                self.method = method
            }
            Message::HeaderKeyChanged(num, key) => {
                if let Some(pair) = self.header.get_mut(num){
                    pair.0 = key;
                }
            }
            Message::HeaderValueChanged(num, value) => {
                if let Some(pair) = self.header.get_mut(num) {
                    pair.1 = value;
                }
            }
            Message::AddHeader => {
                self.header.push((String::new(), String::new()));
            }
            Message::RemoveHeader(num) => {
                if num < self.header.len() {
                    self.header.remove(num);
                }
            }
            Message::BodyChanged(body) => {
                self.body = body
            }
            Message::SendRequest => {
                self.is_loading = true;
                let req = Request {
                    url: self.url.clone(),
                    method: self.method,
                    headers: self.header.clone(),
                    body: if self.body.is_empty() {None} else {Some(self.body.clone())},
                };
                return Command::perform(send_request(req), Message::RequestCompleted);
            }
            Message::RequestCompleted(res) => {
                self.response = Some(res);
                self.is_loading = false;
            }
            Message::TabChanged(tab) => {
                self.active_tab = tab
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        iced::widget::text("Vive le Bénin ! Le Bénin VAINCRA !").size(100).into()
    }
}