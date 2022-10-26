mod components;
mod models;
mod router;

use components::Header;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header/>
            <div class="container">
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
