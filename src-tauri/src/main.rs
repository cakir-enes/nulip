#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct DoSomethingPayload {
    state: String,
    data: u64,
}

// The commands definitions
// Deserialized from JS
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum Cmd {
    DoSomething {
        count: u64,
        payload: DoSomethingPayload,
        callback: String,
        error: String,
    },
}

#[derive(Serialize)]
struct Response<'a> {
    value: u64,
    message: &'a str,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
    message: &'a str,
}

impl<'a> CommandError<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl<'a> std::fmt::Display for CommandError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

struct Stream {
    blocks: Vec<String>,
}
impl Stream {
    pub fn append(&mut self, new_block: &str) {
        self.blocks.push(new_block.to_string());
    }
}

use std::sync::{Arc, RwLock};
fn main() {
    let stream = Arc::from(RwLock::new(Stream { blocks: vec![] }));
    tauri::AppBuilder::new()
        .invoke_handler(move |_webview, arg| {
            use Cmd::*;

            let c1 = stream.clone();

            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),

                Ok(command) => {
                    match command {
                        DoSomething {
                            count,
                            payload,
                            callback,
                            error,
                        } => tauri::execute_promise(
                            _webview,
                            move || {
                                let c3 = c1.clone();
                                // let c3 = stream.clone();
                                if count > 5 {
                                    let response = Response {
                                        value: 5,
                                        message: "async response!",
                                    };

                                    match c3.try_write() {
                                        Ok(mut t) => t.append("asdas"),
                                        Err(e) => println!("kek"),
                                    }
                                    // stream.append("kekeke");
                                    Ok(response)
                                } else {
                                    Err(CommandError::new("count should be > 5").into())
                                }
                            },
                            callback,
                            error,
                        ),
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}