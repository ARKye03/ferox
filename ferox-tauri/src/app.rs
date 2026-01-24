use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct EvalRequest {
    code: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum EvalResponse {
    Success { value: Option<String> },
    Incomplete,
    Error {
        message: String,
        error_type: String,
        line: usize,
        column: usize,
    },
}

#[derive(Clone)]
struct HistoryEntry {
    input: String,
    output: Option<String>,
    is_error: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (input_buffer, set_input_buffer) = signal(String::new());
    let (history, set_history) = signal(Vec::<HistoryEntry>::new());
    let (is_evaluating, set_is_evaluating) = signal(false);

    let eval_handler = move |ev: SubmitEvent| {
        ev.prevent_default();

        let current_input = input.get_untracked();
        if current_input.trim().is_empty() { return; }

        let code = if input_buffer.get_untracked().is_empty() {
            current_input.clone()
        } else {
            format!("{}\n{}", input_buffer.get_untracked(), current_input)
        };

        set_is_evaluating.set(true);

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&EvalRequest { code: code.clone() })
                .unwrap();
            let result = invoke("eval_code", args).await;
            let response: EvalResponse = serde_wasm_bindgen::from_value(result).unwrap();

            match response {
                EvalResponse::Success { value } => {
                    set_history.update(|h| h.push(HistoryEntry {
                        input: code,
                        output: value,
                        is_error: false,
                    }));
                    set_input_buffer.set(String::new());
                    set_input.set(String::new());
                }
                EvalResponse::Incomplete => {
                    set_input_buffer.update(|buf| {
                        if !buf.is_empty() { buf.push('\n'); }
                        buf.push_str(&current_input);
                    });
                    set_input.set(String::new());
                }
                EvalResponse::Error { message, error_type, line, column } => {
                    let err_msg = format!("{} error at {}:{}: {}",
                        error_type, line, column, message);
                    set_history.update(|h| h.push(HistoryEntry {
                        input: code,
                        output: Some(err_msg),
                        is_error: true,
                    }));
                    set_input_buffer.set(String::new());
                    set_input.set(String::new());
                }
            }

            set_is_evaluating.set(false);
        });
    };

    let reset_handler = move |_| {
        spawn_local(async move {
            invoke("reset_interpreter", JsValue::NULL).await;
            set_history.set(Vec::new());
            set_input_buffer.set(String::new());
            set_input.set(String::new());
        });
    };

    let prompt = move || {
        if input_buffer.get().is_empty() { "> " } else { "... " }
    };

    view! {
        <div class="repl-container">
            <h1>"FEROX REPL"</h1>

            <div class="history-panel">
                <For
                    each=move || history.get()
                    key=|entry| entry.input.clone()
                    children=move |entry: HistoryEntry| {
                        view! {
                            <div class="history-entry">
                                <div class="history-input">
                                    {entry.input}
                                </div>
                                {entry.output.map(|out| view! {
                                    <div class=move || {
                                        if entry.is_error { "history-output history-error" }
                                        else { "history-output" }
                                    }>
                                        {out}
                                    </div>
                                })}
                            </div>
                        }
                    }
                />
            </div>

            <form class="input-panel" on:submit=eval_handler>
                <span class="prompt">{prompt}</span>
                <input
                    type="text"
                    class="code-input"
                    prop:value=move || input.get()
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                    prop:disabled=move || is_evaluating.get()
                    placeholder="Enter FEROX code..."
                />
                <button type="submit" disabled=move || is_evaluating.get()>
                    "Eval"
                </button>
            </form>

            <div class="control-panel">
                <button on:click=reset_handler>"Reset"</button>
            </div>
        </div>
    }
}
