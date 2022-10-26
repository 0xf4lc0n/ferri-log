use gloo_net::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state};

use crate::models::LogEntry;

#[function_component(LogsList)]
pub fn logs_list() -> Html {
    let logs = use_state(Vec::new);

    {
        let logs = logs.clone();
        use_effect_with_deps(
            move |_| {
                let logs = logs;
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_logs: Vec<LogEntry> = Request::get("http://localhost:8080/logs")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    logs.set(fetched_logs);
                });
                || ()
            },
            (),
        );
    }

    logs.iter()
        .map(|log| {
            html! {
                <details>
                    <summary>{ format!("{} {} {} {}", log.timestamp, log.host, log.severity, log.source) }</summary>
                    <p>{ format!("Facility: {}", log.facility) }</p>
                    <p>{ format!("Tag: {}", log.syslog_tag) }</p>
                    <p>{ format!("Message: {}", log.message) }</p>
                </details>
            }
        })
        .collect()
}
