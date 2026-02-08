<div align="center">

  <h1>RUSTMAN</h1>
  
  <p>
    <strong>The Native, Blazing Fast API Client.</strong>
  </p>

  <p>
    <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/Made_with-Rust-orange?logo=rust" alt="Rust">
    </a>
    <a href="https://github.com/iced-rs/iced">
      <img src="https://img.shields.io/badge/GUI-Iced_0.12-blue?logo=icloud" alt="Iced">
    </a>
    <a href="https://tokio.rs/">
      <img src="https://img.shields.io/badge/Async-Tokio-green" alt="Tokio">
    </a>
    <a href="#">
      <img src="https://img.shields.io/badge/License-MIT-lightgrey" alt="License">
    </a>
  </p>
  
  <br />
</div>

## 📖 About

**Rustman** is a lightweight, native alternative to tools like Postman or Insomnia. Built entirely in **Rust**, it ditches the heavy Electron wrappers to offer instant startup times and minimal memory usage.

Designed with the **ELM Architecture** and the **Iced** framework, Rustman proves that you can build beautiful, responsive, and type-safe GUI applications for the desktop.

## ✨ Features

* 🚀 **Native Performance:** Starts instantly, uses <50MB RAM.
* 📡 **Full HTTP Support:** GET, POST, PUT, DELETE, PATCH.
* 🎨 **Cyberpunk Interface:** Custom Dark Mode with neon orange accents.
* 📝 **JSON Pretty-Printing:** Automatic formatting and coloring of JSON responses.
* 🛠 **Dynamic Headers:** Add, edit, and remove request headers on the fly.
* 📜 **Request History:** Sidebar automatically saves your requests for quick replay.
* ⚡ **Async Core:** Powered by `Tokio` and `Reqwest` for non-blocking I/O.

## 📸 Screenshots

<div align="center">
  <img src="assets/screenshot.png" alt="Rustman Interface" width="800" />
  </div>

## 🛠 Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/) (Edition 2021)
* **GUI Framework:** [Iced (v0.12)](https://github.com/iced-rs/iced)
* **HTTP Client:** [Reqwest](https://github.com/seanmonstar/reqwest)
* **Async Runtime:** [Tokio](https://tokio.rs/)
* **Serialization:** [Serde JSON](https://github.com/serde-rs/json)

## 🏗 Architecture

Rustman follows a strict **Model-View-Update (ELM)** architecture to ensure code scalability and maintainability.

```text
src/
├── main.rs          # 🏁 Entry Point: Window settings & Application launch.
├── app.rs           # 🧠 The Brain: State management & Update logic.
├── view.rs          # 🎨 The View: UI Layout & Styling (Pure functions).
├── message.rs       # 📨 The Events: Enum listing all possible user actions.
└── http_client.rs   # ⚙️ The Engine: Async network logic, decoupled from UI.