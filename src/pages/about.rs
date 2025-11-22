use yew::prelude::*;
use crate::Route;
use yew_router::prelude::Link;
#[function_component(About)]
pub fn about() -> Html {
    html! {
        <main class="section-wrapper about-page">
            <section class="about">
                <header class="about__header">
                    <h1 class="about__title">{ "A propos" }</h1>
                    <h2 class="about__subtitle">
                        { "Ingénieur – Développeur IoT & Systèmes Embarqués" }
                    </h2>
                    <p class="about__intro">
                        { "Paragraphe de présentation" }
                    </p>
                </header>

                <div class="about__body">
                    <div class="about__photo">
                        <div class="about__photo-circle">
                            { "Photo" }
                        </div>
                    </div>

                    <div class="about__card">
                        <div class="about__section">
                            <h3 class="about__section-title">{ "Qui suis-je ?" }</h3>
                            <p class="about__section-text">
                                { "Paragraphe (6–7 lignes)" }
                            </p>
                        </div>

                        <div class="about__section">
                            <h3 class="about__section-title">{ "Parcours" }</h3>
                            <p class="about__section-text">
                                { "Paragraphe (6–7 lignes)" }
                            </p>
                        </div>

                        <div class="about__section">
                            <h3 class="about__section-title">{ "Soft Skills" }</h3>
                            <p class="about__section-text">
                                { "Paragraphe (6–7 lignes)" }
                            </p>
                        </div>

                        <div class="about__buttons">
                            <a
                                class="btn btn--primary about__button"
                                href="/cv.pdf"         // mets ici le vrai chemin de ton CV
                            >
                                { "Télécharger mon CV" }
                            </a>

                            <Link<Route> to={Route::Contact}>
                                <button class="btn btn--ghost about__button">
                                    { "Me contacter" }
                                </button>
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
