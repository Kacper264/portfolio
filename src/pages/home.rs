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

                <section class="skills">
                    <h3 class="skills__title">{ "Mes Compétences" }</h3>

                    <div class="skills__grid">
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 1" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 2" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 3" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 4" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 5" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 6" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 7" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 8" }</div></div>
                        <div class="skill-card"><div class="skill-card__icon">{ "Icone" }</div><div class="skill-card__label">{ "Skill 9" }</div></div>
                    </div>
                </section>
            </main>
        </>
    }
}

