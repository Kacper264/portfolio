use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <main class="section-wrapper contact-page">
            <section class="contact">
                <header class="contact__header">
                    <h1 class="contact__title">{ "Me contacter" }</h1>
                    <p class="contact__subtitle">
                        { "N'hésitez pas à me contacter pour un projet, une opportunité ou simplement échanger." }
                    </p>
                </header>

                <div class="contact__body">
                    // ------- Carte formulaire -------
                    <div class="contact__card contact__card--form">
                        <div class="contact-form">
                            <input
                                class="contact-form__input"
                                type="text"
                                placeholder="Nom"
                            />

                            <input
                                class="contact-form__input"
                                type="email"
                                placeholder="Email"
                            />

                            <input
                                class="contact-form__input"
                                type="text"
                                placeholder="Objet"
                            />

                            <textarea
                                class="contact-form__textarea"
                                placeholder="Message"
                                rows={6}
                            />

                            <div class="contact-form__actions">
                                <button
                                    type="button"
                                    class="btn btn--primary contact-form__button"
                                >
                                    { "Envoyer" }
                                </button>
                            </div>
                        </div>
                    </div>

                    // ------- Carte informations -------
                    <div class="contact__card contact__card--info">
                        <h2 class="contact-info__title">{ "Mes informations" }</h2>

                        <div class="contact-info__list">
                            <div class="contact-info__item">
                                <span class="contact-info__label">{ "Adresse" }</span>
                                <span class="contact-info__value">{ "37 Rue Mozart, 93700 Drancy" }</span>
                            </div>

                            <div class="contact-info__item">
                                <span class="contact-info__label">{ "Téléphone" }</span>
                                <span class="contact-info__value">{ "07 83 06 19 78" }</span>
                            </div>

                            <div class="contact-info__item">
                                <span class="contact-info__label">{ "Email" }</span>
                                <span class="contact-info__value">{ "wojtowiczkacper264@gmail.com" }</span>
                            </div>

                            <div class="contact-info__item">
                                <span class="contact-info__label">{ "Disponibilité" }</span>
                                <span class="contact-info__value">{ "À compléter" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                // ------- Boutons ronds réseaux -------
                <div class="contact__socials">
                    <a
                        href="https://github.com/ton-github"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="contact-social"
                    >
                    { "GitHub" }
                    </a>

                    <a
                        href="https://www.linkedin.com/in/ton-linkedin"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="contact-social"
                    >
                    { "LinkedIn" }
                    </a>

                    <a
                        href="/contact-mail"   // 👉 nouvelle page, pas un mailto
                        class="contact-social"
                    >
                    { "Mail" }
                    </a>
                </div>
            </section>
        </main>
    }
}
