use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::seo::SeoHead;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <SeoHead
            title="Page introuvable"
            description="La page demandée est introuvable sur le site du festival Solimouv'."
            path="/404"
        />

        <section class="container page-intro" aria-labelledby="nf-title">
            <p class="eyebrow">"Erreur 404"</p>
            <h1 id="nf-title">"Cette page n'existe pas (ou plus)"</h1>
            <p class="lead">
                "Le lien est peut-être cassé ou la page a été déplacée."
            </p>
            <div class="cta-row">
                <A href="/" attr:class="btn btn-primary">"Retour à l'accueil"</A>
                <A href="/programme" attr:class="btn btn-ghost">"Voir le programme"</A>
            </div>
        </section>
    }
}
