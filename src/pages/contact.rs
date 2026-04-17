use leptos::prelude::*;

use crate::components::seo::SeoHead;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <SeoHead
            title="Contact"
            description="Contactez Up Sport! et l'équipe Solimouv' — questions, partenariats, bénévolat : toutes les voies pour nous rejoindre."
            path="/contact"
        />

        <section class="container page-intro" aria-labelledby="contact-title">
            <p class="eyebrow">"Contact"</p>
            <h1 id="contact-title">"Échangeons"</h1>
            <p class="lead">
                "Une question sur le festival, une envie de bénévolat, un projet de partenariat ? "
                "L'équipe Up Sport! vous répond."
            </p>
        </section>

        <section class="container section grid grid-2" aria-labelledby="channels-title">
            <h2 id="channels-title" class="visually-hidden">"Canaux de contact"</h2>

            <article class="card">
                <h3>"Écrire à Up Sport!"</h3>
                <p>"Site officiel :"</p>
                <p>
                    <a href="https://www.unispourlesport.paris/" rel="noopener" target="_blank">
                        "unispourlesport.paris"
                    </a>
                </p>
                <p class="muted mt">
                    "Pour contact direct, passez par le formulaire du site officiel "
                    "en attendant l'ouverture du formulaire Solimouv'."
                </p>
            </article>

            <article class="card">
                <h3>"Nous suivre"</h3>
                <ul class="bullet-list">
                    <li>
                        <a href="https://www.facebook.com/UpSport.UNis/" rel="noopener" target="_blank">"Facebook"</a>
                    </li>
                    <li>
                        <a href="https://www.instagram.com/unispourlesport/" rel="noopener" target="_blank">"Instagram"</a>
                    </li>
                    <li>
                        <a href="https://www.tiktok.com/@upsportparis" rel="noopener" target="_blank">"TikTok"</a>
                    </li>
                </ul>
            </article>
        </section>
    }
}
