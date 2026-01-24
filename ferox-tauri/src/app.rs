use leptos::task::spawn_local;
use leptos::prelude::*;
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

#[derive(Deserialize, Clone)]
struct LineDecoration {
    line: usize,
    result: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum EvalResponse {
    Success {
        value: Option<String>,
        decorations: Vec<LineDecoration>,
    },
    Incomplete,
    Error {
        message: String,
        error_type: String,
        line: usize,
        column: usize,
    },
}

#[component]
pub fn App() -> impl IntoView {
    let (code, set_code) = signal(String::new());
    let (output, set_output) = signal(String::from("// Output will appear here"));
    let (is_error, set_is_error) = signal(false);
    let (is_evaluating, set_is_evaluating) = signal(false);
    let (debounce_timer, set_debounce_timer) = signal(None::<i32>);
    let (decorations, set_decorations) = signal(Vec::<LineDecoration>::new());

    let evaluate_code = move |code_text: String| {
        if code_text.trim().is_empty() {
            set_output.set("// Output will appear here".to_string());
            set_is_error.set(false);
            return;
        }

        set_is_evaluating.set(true);

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&EvalRequest { code: code_text })
                .unwrap();
            let result = invoke("eval_code", args).await;
            let response: EvalResponse = serde_wasm_bindgen::from_value(result).unwrap();

            match response {
                EvalResponse::Success { value, decorations: decors } => {
                    set_decorations.set(decors);
                    if let Some(val) = value {
                        set_output.set(val);
                        set_is_error.set(false);
                    } else {
                        set_output.set("// Function defined successfully".to_string());
                        set_is_error.set(false);
                    }
                }
                EvalResponse::Incomplete => {
                    set_decorations.set(Vec::new());
                    set_output.set("// Code incomplete, keep writing...".to_string());
                    set_is_error.set(false);
                }
                EvalResponse::Error { message, error_type, line, column } => {
                    set_decorations.set(Vec::new());
                    let err_msg = format!("{} error at {}:{}\n{}",
                        error_type, line, column, message);
                    set_output.set(err_msg);
                    set_is_error.set(true);
                }
            }

            set_is_evaluating.set(false);
        });
    };

    let on_input = move |ev| {
        let new_code = event_target_value(&ev);
        set_code.set(new_code.clone());
        set_decorations.set(Vec::new());

        if let Some(timer_id) = debounce_timer.get_untracked() {
            web_sys::window()
                .unwrap()
                .clear_timeout_with_handle(timer_id);
        }

        let callback = Closure::wrap(Box::new(move || {
            evaluate_code(new_code.clone());
        }) as Box<dyn Fn()>);

        let timer_id = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                500,
            )
            .unwrap();

        callback.forget();
        set_debounce_timer.set(Some(timer_id));
    };

    let eval_handler = move |_| {
        let current_code = code.get_untracked();
        evaluate_code(current_code);
    };

    let reset_handler = move |_| {
        spawn_local(async move {
            invoke("reset_interpreter", JsValue::NULL).await;
            set_code.set(String::new());
            set_output.set("// Output will appear here".to_string());
            set_is_error.set(false);
            set_decorations.set(Vec::new());
        });
    };

    let on_scroll = move |ev| {
        let textarea: web_sys::HtmlTextAreaElement = event_target(&ev);
        let scroll_top = textarea.scroll_top();
        let scroll_left = textarea.scroll_left();

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(overlay) = document.get_element_by_id("decorations-overlay") {
                    let html_element: web_sys::HtmlElement = overlay.dyn_into().unwrap();
                    let _ = html_element.style().set_property("transform",
                        &format!("translate({}px, {}px)", -scroll_left, -scroll_top));
                }
            }
        }
    };

    view! {
        <div class="editor-container">
            <h1>"FEROX REPL"</h1>

            <div class="editor-panel">
                <div class="editor-wrapper">
                    <textarea
                        class="code-editor"
                        prop:value=move || code.get()
                        on:input=on_input
                        on:scroll=on_scroll
                        placeholder="Enter FEROX code..."
                        spellcheck="false"
                    />
                    <div class="decorations-overlay" id="decorations-overlay">
                        <For
                            each=move || decorations.get()
                            key=|d| d.line
                            children=move |decoration: LineDecoration| {
                                let line_num = decoration.line;
                                view! {
                                    <div
                                        class="decoration-line"
                                        style=move || format!("top: {}px;", (line_num - 1) * 26)
                                    >
                                        {decoration.result}
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </div>

            <div class="output-panel">
                <div class="output-header">
                    <span class="prompt">"> "</span>
                    <span class="output-label">"Output"</span>
                </div>
                <div class=move || {
                    if is_error.get() { "output-content output-error" }
                    else { "output-content" }
                }>
                    {move || output.get()}
                </div>
            </div>

            <div class="control-panel">
                <button class="eval-button" on:click=eval_handler disabled=move || is_evaluating.get()>
                    "Eval"
                </button>
                <button class="reset-button" on:click=reset_handler>
                    "Reset"
                </button>
            </div>
        </div>
    }
}
