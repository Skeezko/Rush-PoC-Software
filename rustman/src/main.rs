
mod app;
mod view;
mod message;
mod http_client;

#[cfg(feature = "iced12")]
fn main() -> iced::Result {

    app::Rustman::run_app()
}

#[cfg(feature = "iced14")]
fn main() -> iced::Result {
    app::Rustman::run_app()
}

#[cfg(not(any(feature = "iced12", feature = "iced14")))]
fn main() {
    
}
