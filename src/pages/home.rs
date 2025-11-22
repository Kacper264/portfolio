use yew::prelude::*;
use yew_router::prelude::*;  // <-- importe Link et Routable
use crate::routes::Route;     // <-- importe ton enum Route

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main class="section-wrapper">
            <section class="section">
                <div class="section__left">

                    <h1 class="name">{ "Kacper Wojtowicz" }</h1>

                    <h2 class="role">
                        { "Ingénieur – Développeur IoT & Systèmes Embarqués" }
                    </h2>

                    <p class="intro">
                        { "Paragraphes intro" }
                    </p>

                    <div class="buttons">
                        <Link<Route> to={Route::Projects}>
                            <button class="btn btn--primary">
                                { "Voir mes projets" }
                            </button>
                        </Link<Route>>

                        <Link<Route> to={Route::About}>
                            <button class="btn btn--ghost">
                                { "A propos" }
                            </button>
                        </Link<Route>>
                    </div>
                </div>

                <div class="section__right">
                    <div class="photo-placeholder">
                        <span>{ "Photo" }</span>
                    </div>
                </div>
            </section>
        </main>
    }
}
