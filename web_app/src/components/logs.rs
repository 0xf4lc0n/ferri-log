mod logs_list;

use logs_list::LogsList;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event};

#[function_component(Logs)]
pub fn logs() -> Html {
    html! {
        <>
        <form>
            <div class="grid">
                <label for="date">{ "Date"}
                <input type="date" id="date" name="date" onchange={Callback::from(|e: Event| {
                    let target = e.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    log::info!("New value is: {:?}", input.map(|i| i.value()));
                })}/>
                </label>

                <label for="time">{"Time"}
                <input type="time" id="time" name="time"/>
                </label>

                <label for="hostname">{"Hostname"}
                <input type="text" id="hostname" name="hostname" placeholder="Host"/>
                </label>

                <label for="severity">{"Severity"}
                <select id="severity">
                    <option value="" selected=true>{"Select a severity"}</option>
                    <option value="info">{"Info"}</option>
                    <option value="warning">{"Warning"}</option>
                    <option value="error">{"Error"}</option>
                </select>
                </label>

                <label for="lastname">{"Source"}
                <input type="text" id="lastname" name="lastname" placeholder="Source"/>
                </label>
            </div>
        </form>
        <LogsList/>
        </>
    }
}
