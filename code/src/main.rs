// This is definitely not most idiomatic Rust code but it should work just fine >.<

use leptos::*;
use regex_lite::Regex;

fn main() {
    mount_to_body(|| view! { <App/> });
}

const NO_CAPTURES_FOUND: &str = "Required captures not found, invalid TailwindUI comment";
fn get_output(input: String) -> String {
    let enter_regex = Regex::new(r#"Entering:\s*"(.*)"\s*From:\s*"(.*)"\s*To:\s*"(.*)""#).unwrap();
    let leave_regex = Regex::new(r#"Leaving:\s*"(.*)"\s*From:\s*"(.*)"\s*To:\s*"(.*)""#).unwrap();

    let enter_captures = enter_regex.captures(&input);
    let leave_captures = leave_regex.captures(&input);

    if enter_captures.is_none() || leave_captures.is_none() {
        return NO_CAPTURES_FOUND.to_string();
    }

    let enter_captures = enter_captures.unwrap();
    let leave_captures = leave_captures.unwrap();

    let output = format!(
        r#"
<Transition
  enter="{}"
  enterFrom="{}"
  enterTo="{}"
  leave="{}"
  leaveFrom="{}"
  leaveTo="{}"
>
</Transition>"#,
        enter_captures.get(1).unwrap().as_str(),
        enter_captures.get(2).unwrap().as_str(),
        enter_captures.get(3).unwrap().as_str(),
        leave_captures.get(1).unwrap().as_str(),
        leave_captures.get(2).unwrap().as_str(),
        leave_captures.get(3).unwrap().as_str(),
    );

    output
}

const DEFAULT_INPUT: &str = r#"<!--
  Show/hide based on menu state.

  Entering: "transition ease-out duration-100"
    From: "transform opacity-0 scale-95"
    To: "transform opacity-100 scale-100"
  Leaving: "transition ease-in duration-75"
    From: "transform opacity-100 scale-100"
    To: "transform opacity-0 scale-95"
-->"#;

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = create_signal(DEFAULT_INPUT.to_string());
    let output = move || get_output(input.get());

    view! {
        <h2>Input</h2>
        <textarea
            on:input=move |ev| {
                set_input(event_target_value(&ev));
            }

            prop:value=input
        />

        <h2>Output</h2>
        <textarea readonly id="output" class:error=move || output() == NO_CAPTURES_FOUND>{output}</textarea>
        <button disabled=move || output() == NO_CAPTURES_FOUND onclick="copyOutput()">Copy</button>
    }
}
