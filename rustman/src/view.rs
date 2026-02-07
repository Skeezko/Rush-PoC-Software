use iced::widget::{button, column, row, text, text_input, scrollable, container};
use iced::{Alignment, Element, Length};

use crate::message::{Message, ResponseTab};
use crate::http_client::HttpMethod;
use crate::app::Rustman;

pub fn view(app: &Rustman) -> Element<Message> {
    let address_bar = row![
        text(format!("{:?}", app.method)), 
        text_input("https://api.example.com", &app.url)
            .on_input(Message::UrlChanged),
        button(if app.is_loading { "Envoi..." } else { "Send" })
            .on_press(Message::SendRequest),
    ]
    .spacing(10)
    .padding(10)
    .align_y(Alignment::Center);

    let mut headers_column = column![text("Headers")].spacing(5);
    
    for (i, (key, value)) in app.headers.iter().enumerate() {
        let header_row = row![
            text_input("Key", key)
                .on_input(move |v| Message::HeaderKeyChanged(i, v)),
            text_input("Value", value)
                .on_input(move |v| Message::HeaderValueChanged(i, v)),
            button("X").on_press(Message::RemoveHeader(i)),
        ]
        .spacing(10);
        
        headers_column = headers_column.push(header_row);
    }
    
    let add_header_btn = button("Add Header").on_press(Message::AddHeader);

    let response_area = if let Some(res) = &app.response {
        match res {
            Ok(api_res) => {
                column![
                    text(format!("Status: {} {}", api_res.status, api_res.status_text)),
                    text(format!("Time: {}ms", api_res.duration_ms)),
                    scrollable(text(&api_res.body)).height(Length::Fill)
                ].spacing(10)
            }
            Err(e) => column![text(format!("Error: {}", e))],
        }
    } else {
        column![text("En attente de requête...")]
    };

    container(
        column![
            address_bar,
            headers_column,
            add_header_btn,
            response_area
        ]
        .spacing(20)
        .padding(20)
    )
    .into()
}