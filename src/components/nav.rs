use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <header class="top-nav">
            <nav class="top-nav__inner">
                <Link<Route> to={Route::Home} classes="top-nav__logo">
                    { "KW" }
                </Link<Route>>

                <div class="top-nav__links">
                    <Link<Route> to={Route::About} classes="top-nav__link">
                        { "À propos" }
                    </Link<Route>>

                    <Link<Route> to={Route::Projects} classes="top-nav__link">
                        { "Projets" }
                    </Link<Route>>

                    <Link<Route> to={Route::Contact} classes="top-nav__link">
                        { "Contact" }
                    </Link<Route>>
                </div>
            </nav>
        </header>
    }
}
