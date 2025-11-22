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
            <section class="skills">
                <h3 class="skills__title">{ "Mes Compétences" }</h3>

                <div class="skills__grid">

                    // --- Skill 1 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 1" }</div>
                    </div>

                    // --- Skill 2 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 2" }</div>
                    </div>

                    // --- Skill 3 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 3" }</div>
                    </div>

                    // --- Skill 4 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 4" }</div>
                    </div>

                    // --- Skill 5 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 5" }</div>
                    </div>

                    // --- Skill 6 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 6" }</div>
                    </div>

                    // --- Skill 7 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 7" }</div>
                    </div>

                    // --- Skill 8 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 8" }</div>
                    </div>

                    // --- Skill 9 ---
                    <div class="skill-card">
                        <div class="skill-card__icon">{ "Icone" }</div>
                        <div class="skill-card__label">{ "Skill 9" }</div>
                    </div>

                </div>
            </section>
        </main>
    }
}
