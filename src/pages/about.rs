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
                                { "Je suis un passionné de technologies embarquées et d’IoT, animé par l’envie de créer des systèmes intelligents et performants. 
                                J’aime transformer une idée en solution concrète, qu’il s’agisse de code bas niveau, de design électronique ou d’optimisation hardware. 
                                Curieux, méthodique et toujours en quête de nouveaux défis, je m’investis dans chaque projet avec l’objectif de proposer des solutions fiables, 
                                propres et efficaces. Mon approche : comprendre, concevoir et améliorer en continu." }
                            </p>
                        </div>

                        <div class="about__section">
                            <h3 class="about__section-title">{ "Mon parcours" }</h3>
                            <p class="about__section-text">
                                { "Mon cheminement a commencé avec un BAC STI2D, puis un BTS en électronique qui m’a donné une base technique solide. 
                                Après une prépa orientée mathématiques, j’ai rejoint l’ESIEE Paris en cycle ingénieur Systèmes Embarqués, tout en travaillant en alternance chez Paprec. 
                                Aujourd’hui, je conçois des systèmes embarqués, des cartes électroniques et des infrastructures réseau sécurisées (RADIUS, filtrage MAC). 
                                J’ai aussi eu l’opportunité de passer par le service informatique de la mairie de Maisons-Alfort, ce qui m’a permis d’aborder des environnements 
                                professionnels variés et concrets." }
                            </p>
                        </div>

                        <div class="about__section">
                            <h3 class="about__section-title">{ "Soft Skills" }</h3>
                            <p class="about__section-text">
                                { "Je m’appuie sur un esprit analytique, une forte capacité d’adaptation et un réel goût pour la résolution de problèmes. 
                                Travailler sur des projets techniques m’a appris à être rigoureux, autonome, mais aussi à collaborer efficacement avec des équipes aux profils différents. 
                                Mes loisirs comme l’e-sport, les mathématiques et la programmation nourrissent mon sens de la stratégie, ma créativité et ma persévérance. 
                                J’avance toujours avec l’idée d’apprendre, de progresser et d’aller plus loin techniquement." }
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
