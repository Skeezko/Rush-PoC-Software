// src/main.rs

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
    // iced 0.14: runner custom dans a14.rs
    app::Rustman::run_app()
}

// Si tu oublies d'activer une feature, tu auras un message clair:
#[cfg(not(any(feature = "iced12", feature = "iced14")))]
fn main() {
    
}
