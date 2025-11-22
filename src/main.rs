use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod pages;

use routes::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::Projects => html! { <pages::projects::Projects /> },
        Route::About => html! { <pages::about::About /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
