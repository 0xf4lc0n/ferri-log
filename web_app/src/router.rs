use crate::components::{Computers, Logs, NotFound};
use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/computers")]
    Computers,
    #[at("/logs")]
    Logs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { "Home" },
        Route::Computers => html! {<Computers/> },
        Route::Logs => html! { <Logs/> },
        Route::NotFound => html! { <NotFound/> },
    }
}
