use yew::prelude::*;
use yew_router::prelude::*;
mod components;  
mod routes;
mod pages;
use crate::components::nav::Nav; 
use routes::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::Projects => html! { <pages::projects::Projects /> },
        Route::About => html! { <pages::about::About /> },
        Route::Contact => html! { <pages::contact::Contact /> },
        Route::Links => todo!(),
        Route::Competence => html! { <pages::skills::Skills />},
    }
}
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
