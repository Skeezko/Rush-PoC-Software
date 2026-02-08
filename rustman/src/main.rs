mod message;
mod http_client;
mod app;
mod view;

use app::Rustman;
use iced::{Application, Settings};
fn main() -> iced::Result {
    iced::run(Rustman::update, Rustman::view)
}