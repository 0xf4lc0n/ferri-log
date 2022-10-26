use yew::{function_component, html, Callback};
use yew_router::prelude::{use_history, History};

use crate::router::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <div>
            <h1>{"Oh no! That page doesn't exists!!!"}</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
