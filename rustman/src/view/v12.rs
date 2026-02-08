use crate::app::Rustman;
use crate::http_client::HttpMethod;
use crate::message::{Message, ResponseTab};
use crate::app::HistoryItem;

use iced::alignment::Horizontal;
use iced::widget::{
    button, column, container, pick_list, row, scrollable, text, text_input, Space,
};
use iced::{Alignment, Element, Length, Theme};

const METHODS: [HttpMethod; 5] = [
    HttpMethod::GET,
    HttpMethod::POST,
    HttpMethod::PUT,
    HttpMethod::PATCH,
    HttpMethod::DELETE,
];

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

pub fn view(app: &Rustman) -> Element<Message> {
    let brand = text("RUSTMAN").size(34);
    let subtitle = text("The Rust API Testing Tool").size(14);

    // --- Request bar ---
    let method = pick_list(METHODS, Some(app.method), Message::MethodSelected)
        .padding(10)
        .text_size(14)
        .width(Length::Fixed(120.0));

    let url = text_input("https://...", &app.url)
        .on_input(Message::UrlChanged)
        .padding(12)
        .size(14)
        .width(Length::Fill);

    let disabled_send = app.is_loading || app.url.trim().is_empty();
    let send_label = if app.is_loading { "Sending…" } else { "▶ Send" };

    let send_btn = button(text(send_label).size(14))
        .padding(12)
        .on_press_maybe((!disabled_send).then_some(Message::SendRequest));

    let request_line = row![method, url, send_btn]
        .spacing(12)
        .align_items(Alignment::Center);

    let headers_title = row![
        text("Headers").size(16),
        Space::with_width(Length::Fill),
        button(text("+ Add").size(13)).padding(8).on_press(Message::AddHeader),
    ]
    .align_items(Alignment::Center);

    let headers_list = column(
        app.header
            .iter()
            .enumerate()
            .map(|(i, (k, v))| header_row(i, k, v))
            .collect::<Vec<_>>(),
    )
    .spacing(10);

    let headers_panel = container(
        column![
            headers_title,
            Space::with_height(Length::Fixed(10.0)),
            headers_list
        ],
    )
    .padding(14)
    .width(Length::Fill);

    let body_title = text("Request Body").size(16);

    let body_input = text_input("Enter JSON body…", &app.body)
        .on_input(Message::BodyChanged)
        .padding(12)
        .size(14)
        .width(Length::Fill);

    let body_panel = container(
        column![
            body_title,
            Space::with_height(Length::Fixed(10.0)),
            body_input
        ],
    )
    .padding(14)
    .width(Length::Fill);

    let (status_line, _status_is_ok) = response_status_line(app);

    let response_header = row![
        text("Response").size(16),
        Space::with_width(Length::Fixed(16.0)),
        text(status_line).size(14),
    ]
    .align_items(Alignment::Center);

    let tabs = row![
        tab_button("Body", ResponseTab::Body, app.active_tab),
        tab_button("Headers", ResponseTab::Headers, app.active_tab),
        Space::with_width(Length::Fill),
    ]
    .spacing(10);

    let response_content = match (&app.response, app.active_tab) {
        (None, _) => code_box("Send a request to see the response…".to_string()),
        (Some(Ok(res)), ResponseTab::Body) => code_box(res.body.clone()),
        (Some(Ok(res)), ResponseTab::Headers) => headers_box(&res.headers),
        (Some(Err(e)), _) => code_box(e.clone()),
    };

    let response_panel = container(
        column![
            response_header,
            Space::with_height(Length::Fixed(10.0)),
            tabs,
            Space::with_height(Length::Fixed(10.0)),
            response_content
        ]
        .spacing(0),
    )
    .padding(14)
    .width(Length::Fill);

    let page: Element<'static, Message> = container(
        column![
            brand,
            subtitle,
            Space::with_height(Length::Fixed(16.0)),
            request_line,
            Space::with_height(Length::Fixed(14.0)),
            headers_panel,
            Space::with_height(Length::Fixed(14.0)),
            body_panel,
            Space::with_height(Length::Fixed(14.0)),
            response_panel,
        ]
        .padding(18)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    let history_panel: Element<'static, Message> = history_view(app);

    let layout: Element<'static, Message> = row![history_panel, page]
        .spacing(14)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    layout
}
    
fn header_row(i: usize, key: &str, value: &str) -> Element<'static, Message> {
    let key_in = text_input("Content-Type", key)
        .on_input(move |s| Message::HeaderKeyChanged(i, s))
        .padding(10)
        .size(14)
        .width(Length::FillPortion(2));

    let val_in = text_input("application/json", value)
        .on_input(move |s| Message::HeaderValueChanged(i, s))
        .padding(10)
        .size(14)
        .width(Length::FillPortion(3));

    let remove = button(
        text("×")
            .size(18)
            .horizontal_alignment(Horizontal::Center),
    )
    .padding(8)
    .on_press(Message::RemoveHeader(i));

    row![key_in, val_in, remove]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
}

fn tab_button(label: &str, tab: ResponseTab, active: ResponseTab) -> Element<Message> {
    let is_active = tab == active;
    let label = if is_active { format!("[{}]", label) } else { label.to_string() };

    button(text(label).size(13))
        .padding(8)
        .on_press(Message::TabChanged(tab))
        .into()
}

fn response_status_line(app: &Rustman) -> (String, bool) {
    match &app.response {
        None => ("".to_string(), true),
        Some(Ok(r)) => (
            format!("Status: {} {} ({} ms)", r.status, r.status_text, r.duration_ms),
            (200..300).contains(&r.status),
        ),
        Some(Err(_)) => ("Status: ERROR".to_string(), false),
    }
}

fn code_box(body: String) -> Element<'static, Message> {
    container(scrollable(text(body).size(13)).height(Length::Fixed(260.0)))
        .padding(12)
        .width(Length::Fill)
        .into()
}

fn headers_box(headers: &[(String, String)]) -> Element<'static, Message> {
    let mut out = String::new();
    if headers.is_empty() {
        out.push_str("No headers");
    } else {
        for (k, v) in headers {
            out.push_str(k);
            out.push_str(": ");
            out.push_str(v);
            out.push('\n');
        }
    }
    code_box(out) 
}
fn history_view(app: &Rustman) -> Element<'static, Message> {
    let header = row![
        text("History").size(16),
        Space::with_width(Length::Fill),
        button(text("Clear").size(12)).padding(6).on_press(Message::ClearHistory)
    ]
    .align_items(Alignment::Center);

    let list = if app.history.is_empty() {
        column![text("No requests yet").size(13)].spacing(8)
    } else {
        let items = app
            .history
            .iter()
            .enumerate()
            .map(|(i, h)| history_item(i, h, app.selected_history))
            .collect::<Vec<_>>();

        column(items).spacing(8)
    };

    container(column![header, Space::with_height(Length::Fixed(10.0)), scrollable(list)])
        .padding(12)
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .into()
}

fn history_item(
    i: usize,
    h: &HistoryItem,
    selected: Option<usize>,
) -> Element<'static, Message> {
    let is_selected = selected == Some(i);

    
    let (status, extra) = match &h.response {
        None => ("…".to_string(), "".to_string()),
        Some(Ok(r)) => (r.status.to_string(), format!("{}ms", r.duration_ms)),
        Some(Err(_)) => ("ERR".to_string(), "".to_string()),
    };

    let line1 = row![
        text(format!("{}", h.method)).size(12),
        Space::with_width(Length::Fixed(8.0)),
        text(status).size(12),
        Space::with_width(Length::Fill),
        text(extra).size(12),
    ]
    .align_items(Alignment::Center);

    let line2 = text(h.url.clone()).size(12);

    let content = column![line1, line2].spacing(4);

    button(content)
        .padding(if is_selected { 12 } else { 10 })
        .on_press(Message::SelectHistory(i))
        .into()
}
