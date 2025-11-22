use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectCardProps {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub github: String,
    pub overleaf: String,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    let flipped = use_state(|| false);

    let onclick_toggle = {
        let flipped = flipped.clone();
        Callback::from(move |_| flipped.set(!*flipped))
    };

    let card_class = if *flipped {
        "project-card is-flipped"
    } else {
        "project-card"
    };

    html! {
        <article class={card_class}>
            <div class="project-card__inner">
                // ----- FACE AVANT -----
                <div class="project-card__face project-card__face--front">
                    <div class="project-card__image">
                        { "Image" }
                    </div>

                    <div class="project-card__content">
                        <div class="project-card__text">
                            <h2 class="project-card__title">{ &props.title }</h2>
                            <p class="project-card__subtitle">{ &props.subtitle }</p>
                            <p class="project-card__description">
                                { &props.description }
                            </p>
                        </div>

                        <div class="project-card__actions">
                            <button class="btn btn--primary project-card__button"
                                onclick={onclick_toggle.clone()}>
                                { "Voir plus" }
                            </button>
                        </div>
                    </div>
                </div>

                // ----- FACE ARRIÈRE -----
                <div class="project-card__face project-card__face--back">
                    <div class="project-card__back-content">
                        <h3 class="project-card__back-title">{ &props.title }</h3>
                        <p class="project-card__back-subtitle">{ "Liens du projet" }</p>

                        <div class="project-card__links">
                            <a
                                class="project-link"
                                href={props.github.clone()}
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                { "GitHub" }
                            </a>

                            <a
                                class="project-link"
                                href={props.overleaf.clone()}
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                { "Overleaf" }
                            </a>
                        </div>

                        <button
                            class="btn btn--primary project-card__button"
                            onclick={onclick_toggle}
                        >
                            { "Retour" }
                        </button>
                    </div>
                </div>
            </div>
        </article>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = vec![
        ProjectCardProps {
            title: "Projet 1".into(),
            subtitle: "Sous titre".into(),
            description: "Description (200 char max)".into(),
            github: "https://github.com/ton-compte/projet1".into(),
            overleaf: "https://www.overleaf.com".into(),
        },
        ProjectCardProps {
            title: "Projet 2".into(),
            subtitle: "Sous titre".into(),
            description: "Description (200 char max)".into(),
            github: "https://github.com/ton-compte/projet2".into(),
            overleaf: "https://www.overleaf.com".into(),
        },
    ];

    html! {
        <main class="section-wrapper projects-page">
            <section class="projects">
                <h1 class="projects__title">{ "Projets" }</h1>

                <div class="projects__grid">
                    {
                        for projects.into_iter().map(|p| html! {
                            <ProjectCard
                                title={p.title}
                                subtitle={p.subtitle}
                                description={p.description}
                                github={p.github}
                                overleaf={p.overleaf}
                            />
                        })
                    }
                </div>
            </section>
        </main>
    }
}
