use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct SkillCardProps {
    pub icon: &'static str,
    pub label: &'static str,
}

#[function_component(SkillCard)]
fn skill_card(props: &SkillCardProps) -> Html {
    html! {
        <div class="skill-card">
            <div class="skill-card__icon">{ props.icon }</div>
            <div class="skill-card__label">{ props.label }</div>
        </div>
    }
}

#[function_component(Skills)]
pub fn skills() -> Html {
    const SKILLS: [&str; 9] = [
        "C / C++",
        "Assembleur",
        "Linux",
        "Visual Studio Code",
        "EasyEDA",
        "Proteus",
        "HTML / CSS",
        "Web 3.0",
        "Office 360",
    ];

    html! {
        <main class="section-wrapper skills-page">
            <div class="skills">
                <h3 class="skills__title">{ "Mes Compétences" }</h3>

                <div class="skills__grid">
                    {
                        SKILLS.iter().map(|label| {
                            html! {
                                <SkillCard icon="Icone" label={*label} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </main>
    }
}
