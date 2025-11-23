use yew::prelude::*;
use yew_router::prelude::*;  // <-- importe Link et Routable
use crate::routes::Route;     // <-- importe ton enum Route

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            // ------- CONTENU EXISTANT -------
            <main class="section-wrapper">
                <section class="section">
                    <div class="section__left">
                        <h1 class="name">{ "Kacper Wojtowicz" }</h1>

                        <h2 class="role">
                            { "Ingénieur – Développeur IoT & Systèmes Embarqués" }
                        </h2>

                        <p class="intro">
                            { "Passionné par les systèmes embarqués et les technologies IoT, je développe des solutions alliant électronique, programmation bas niveau et conception de cartes. 
                            Actuellement en cycle ingénieur à l’ESIEE Paris et alternant chez Paprec, j'interviens sur des projets alliant design hardware, sécurité réseau et développement logiciel. 
                            Curieux, rigoureux et animé par l’envie d’apprendre, je cherche constamment à créer des systèmes efficaces, optimisés et fiables." }
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
        </>
    }
}

