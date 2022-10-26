use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let history = use_history().expect("Cannot get hook to AnyHistory");

    let go_to_home_page = {
        let history = history.clone();
        Callback::once(move |_| history.push(Route::Home))
    };

    let go_to_computers = {
        let history = history.clone();
        Callback::once(move |_| history.push(Route::Computers))
    };

    let go_to_logs = Callback::once(move |_| history.push(Route::Logs));

    html! {
        <nav class="container">
            <ul>
                <li onclick={go_to_home_page}><strong>{"Ferri Log"}</strong></li>
            </ul>
            <ul>
                <li><a onclick={go_to_computers}>{"Computers"}</a></li>
                <li><a onclick={go_to_logs}>{"Logs"}</a></li>
            </ul>
        </nav>
    }
}
