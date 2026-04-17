use leptos::prelude::*;

use crate::components::seo::SeoHead;

#[component]
pub fn AssociationsPage() -> impl IntoView {
    view! {
        <SeoHead
            title="Associations partenaires"
            description="Les associations et partenaires institutionnels du festival Solimouv' : un collectif engagé pour un sport accessible et inclusif."
            path="/associations"
        />

        <section class="container page-intro" aria-labelledby="associations-title">
            <p class="eyebrow">"Partenaires"</p>
            <h1 id="associations-title">"Nos partenaires"</h1>
            <p class="lead">
                "Solimouv' est rendu possible grâce au soutien d'institutions publiques "
                "et d'acteurs engagés pour le sport inclusif."
            </p>
        </section>

        <section class="container section" aria-labelledby="partners-title">
            <h2 id="partners-title" class="visually-hidden">"Logos des partenaires"</h2>
            <ul class="logo-grid" role="list">
                <li class="logo-item">
                    <img
                        src="/asso-logos/up-sport.svg"
                        alt="Up Sport! — organisateur du festival Solimouv'"
                        loading="lazy"
                        width="320"
                        height="120"
                    />
                </li>
                <li class="logo-item">
                    <img
                        src="/asso-logos/Ville_de_Paris_logo.svg.png"
                        alt="Ville de Paris"
                        loading="lazy"
                        width="200"
                        height="203"
                    />
                </li>
                <li class="logo-item">
                    <img
                        src="/asso-logos/ans-logo-ref-cmjn-8ce00-jpg-1240.jpg"
                        alt="Agence Nationale du Sport"
                        loading="lazy"
                        width="520"
                        height="182"
                    />
                </li>
                <li class="logo-item">
                    <img
                        src="/asso-logos/images.png"
                        alt="France Travail"
                        loading="lazy"
                        width="336"
                        height="150"
                    />
                </li>
            </ul>
            <p class="muted mt">
                "Vous représentez une association et souhaitez rejoindre Solimouv' ? "
                "Contactez Up Sport! via la page "
                <a href="/contact">"Contact"</a>
                "."
            </p>
        </section>
    }
}
