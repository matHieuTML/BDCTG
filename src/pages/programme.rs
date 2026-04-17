use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::seo::SeoHead;
use crate::data::ACTIVITIES;
use crate::auth::get_current_user;
use crate::inscriptions::{is_registered, JoinEvent};

#[component]
pub fn ProgrammePage() -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| async move { get_current_user().await });
    let registered_resource = Resource::new(|| (), |_| async move { is_registered().await });
    let join_action = ServerAction::<JoinEvent>::new();

    Effect::new(move || {
        if let Some(Ok(true)) = join_action.value().get() {
            let _ = web_sys::window()
                .expect("window")
                .location()
                .set_href("/mon-espace");
        }
    });

    let color_accents = ["coral", "sky", "mint", "coral", "sky", "mint", "coral", "sky"];

    view! {
        <SeoHead
            title="Programme & Ateliers"
            description="Découvrez les 8 ateliers sportifs inclusifs du festival Solimouv' : basket fauteuil, escalade, danse, secourisme et plus."
            path="/programme"
        />

        <section class="prog-hero">
            <div class="prog-hero__bg">
                <div class="prog-hero__orb prog-hero__orb--1"></div>
                <div class="prog-hero__orb prog-hero__orb--2"></div>
                <div class="prog-hero__orb prog-hero__orb--3"></div>
                <div class="prog-hero__noise"></div>
            </div>
            <div class="container prog-hero__content">
                <span class="hero-eyebrow">"17 Avril 2026 · Lyon"</span>
                <h1 class="prog-hero__title">"Le programme"</h1>
                <p class="prog-hero__lead">
                    "8 ateliers sportifs inclusifs encadrés par nos associations partenaires. "
                    "Venez découvrir, essayer, et choisir vos activités le jour J."
                </p>
            </div>
        </section>

        <section class="ateliers" aria-labelledby="ateliers-title">
            <div class="container">
                <div class="ateliers__header">
                    <h2 id="ateliers-title" class="ateliers__title">"Les ateliers"</h2>
                    <p class="ateliers__sub">"Chaque atelier est ouvert à toutes et tous, quel que soit votre niveau ou votre mobilité."</p>
                </div>

                <div class="ateliers__grid">
                    {ACTIVITIES.iter().enumerate().map(|(i, activity)| {
                        let accent = color_accents[i % color_accents.len()];
                        let delay = format!("{}ms", i * 80);

                        view! {
                            <article
                                class=format!("atelier-card atelier-card--{}", accent)
                                style=format!("animation-delay: {}", delay)
                            >
                                <div class="atelier-card__glow"></div>
                                <div class="atelier-card__inner">
                                    <span class="atelier-card__icon">{activity.icon}</span>
                                    <h3 class="atelier-card__name">{activity.name}</h3>
                                    <p class="atelier-card__assoc">{activity.association}</p>
                                    <p class="atelier-card__desc">{activity.description}</p>
                                    <div class="atelier-card__footer">
                                        <span class="atelier-tag">"🕐 "{activity.horaire}</span>
                                        <span class="atelier-tag">"📍 "{activity.lieu}</span>
                                    </div>
                                </div>
                            </article>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>

        <section class="prog-info">
            <div class="container">
                <div class="prog-info__cards">
                    <div class="prog-info__card prog-info__card--coral">
                        <span class="prog-info__icon">"📍"</span>
                        <div>
                            <strong>"Complexe sportif municipal"</strong>
                            <p>"12 rue du Sport, 69001 Lyon"</p>
                        </div>
                    </div>
                    <div class="prog-info__card prog-info__card--sky">
                        <span class="prog-info__icon">"📅"</span>
                        <div>
                            <strong>"17 avril 2026"</strong>
                            <p>"De 9h00 à 19h00"</p>
                        </div>
                    </div>
                    <div class="prog-info__card prog-info__card--mint">
                        <span class="prog-info__icon">"💰"</span>
                        <div>
                            <strong>"Entrée gratuite"</strong>
                            <p>"Ouvert à toutes et tous"</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="prog-cta">
            <div class="prog-cta__bg">
                <div class="prog-cta__ray prog-cta__ray--1"></div>
                <div class="prog-cta__ray prog-cta__ray--2"></div>
            </div>
            <div class="container prog-cta__inner">
                <h2>"Rejoignez le mouvement"</h2>
                <p>"Inscrivez-vous à l'événement et choisissez vos ateliers le jour J sur place."</p>
                <div class="prog-cta__actions">
                    <Suspense fallback=move || view! { <span class="auth-placeholder"></span> }>
                        {move || Suspend::new(async move {
                            let user = user_resource.await;
                            let is_logged_in = matches!(user, Ok(Some(_)));

                            if !is_logged_in {
                                return view! {
                                    <A href="/inscription" attr:class="btn btn-glow btn-lg">"Créer un compte"</A>
                                    <A href="/connexion" attr:class="btn btn-outline-light btn-lg">"Se connecter"</A>
                                }.into_any();
                            }

                            let is_reg = registered_resource.await;
                            let registered = matches!(is_reg, Ok(true));

                            if registered {
                                view! {
                                    <div class="prog-cta__confirmed">
                                        <span class="prog-cta__check">"✓"</span>
                                        <p>"Vous êtes inscrit(e) !"</p>
                                        <A href="/mon-espace" attr:class="btn btn-outline-light btn-lg">"Mon espace"</A>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <ActionForm action=join_action>
                                        <button type="submit" class="btn btn-glow btn-lg" disabled=move || join_action.pending().get()>
                                            {move || if join_action.pending().get() { "Inscription..." } else { "Rejoindre l'événement" }}
                                        </button>
                                    </ActionForm>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </section>
    }
}
