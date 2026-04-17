use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::seo::SeoHead;
use crate::auth::get_current_user;
use crate::inscriptions::{is_registered, generate_qr_svg, JoinEvent};
use crate::pages::admin_dashboard::AdminDashboard;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| async move { get_current_user().await });

    view! {
        <Suspense fallback=move || view! {
            <section class="auth-page">
                <div class="auth-card"><p>"Chargement..."</p></div>
            </section>
        }>
            {move || Suspend::new(async move {
                let user = user_resource.await;
                match user {
                    Ok(Some(user)) if user.role == "admin" => view! {
                        <SeoHead
                            title="Administration"
                            description="Dashboard administrateur Solimouv'"
                            path="/mon-espace"
                        />
                        <section class="dashboard admin-wrap">
                            <div class="container">
                                <AdminDashboard />
                            </div>
                        </section>
                    }.into_any(),
                    Ok(Some(user)) => {
                        view! {
                            <SeoHead
                                title="Mon espace"
                                description="Votre espace personnel Solimouv'"
                                path="/mon-espace"
                            />
                            <UserDashboardInner user=user />
                        }.into_any()
                    },
                    _ => view! {
                        <SeoHead
                            title="Mon espace"
                            description="Connectez-vous pour accéder à votre espace Solimouv'"
                            path="/mon-espace"
                        />
                        <section class="auth-page">
                            <div class="auth-card" style="text-align:center">
                                <h1>"Accès réservé"</h1>
                                <p class="auth-subtitle">"Connectez-vous pour accéder à votre espace"</p>
                                <div style="margin-top:1.5rem">
                                    <A href="/connexion" attr:class="btn btn-glow">"Se connecter"</A>
                                </div>
                            </div>
                        </section>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn UserDashboardInner(user: crate::models::UserInfo) -> impl IntoView {
    let registered_resource = Resource::new(|| (), |_| async move { is_registered().await });
    let qr_resource = Resource::new(|| (), |_| async move { generate_qr_svg().await });
    let join_action = ServerAction::<JoinEvent>::new();

    Effect::new(move || {
        if join_action.value().get().is_some() {
            registered_resource.refetch();
            qr_resource.refetch();
        }
    });

    let prenom = user.prenom.clone();
    let prenom2 = user.prenom.clone();
    let email = user.email.clone();

    view! {
        <section class="dashboard">
            <div class="container">
                <div class="dashboard-header">
                    <h1 class="dashboard-title">"Bonjour, "{prenom}" 👋"</h1>
                    <p class="dashboard-subtitle">"Votre espace personnel Solimouv'"</p>
                </div>

                <div class="dashboard-grid">
                    <div class="dashboard-main">
                        <div class="dash-card">
                            <h2>"Mon inscription"</h2>
                            <Suspense fallback=move || view! { <p class="muted">"Chargement..."</p> }>
                                {move || Suspend::new(async move {
                                    let is_reg = registered_resource.await;
                                    let registered = matches!(is_reg, Ok(true));

                                    if registered {
                                        view! {
                                            <div class="dash-registered">
                                                <div class="dash-registered__badge">
                                                    <span class="dash-registered__check">"✓"</span>
                                                    <div>
                                                        <strong>"Inscrit(e) au festival"</strong>
                                                        <p>"Vous participerez au festival Solimouv' le 17 avril 2026"</p>
                                                    </div>
                                                </div>
                                                <p class="dash-registered__note">
                                                    "Choisissez vos ateliers directement sur place le jour de l'événement. "
                                                    "Consultez le "
                                                    <A href="/programme">"programme"</A>
                                                    " pour préparer votre journée."
                                                </p>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="dash-empty">
                                                <p>"Vous n'êtes pas encore inscrit(e) à l'événement."</p>
                                                <ActionForm action=join_action>
                                                    <button type="submit" class="btn btn-glow" disabled=move || join_action.pending().get()>
                                                        {move || if join_action.pending().get() { "Inscription..." } else { "Rejoindre l'événement" }}
                                                    </button>
                                                </ActionForm>
                                            </div>
                                        }.into_any()
                                    }
                                })}
                            </Suspense>
                        </div>

                        <div class="dash-card info-recap">
                            <h2>"Infos pratiques"</h2>
                            <ul class="recap-list">
                                <li><strong>"📅 Date"</strong><span>"Vendredi 17 avril 2026"</span></li>
                                <li><strong>"🕐 Horaires"</strong><span>"9h00 – 19h00"</span></li>
                                <li><strong>"📍 Lieu"</strong><span>"Complexe sportif municipal, 12 rue du Sport, 69001 Lyon"</span></li>
                                <li><strong>"💰 Tarif"</strong><span>"Gratuit — ouvert à toutes et tous"</span></li>
                            </ul>
                        </div>
                    </div>

                    <div class="dashboard-side">
                        <div class="dash-card qr-card">
                            <h2>"Mon QR code"</h2>
                            <p class="qr-hint">"Présentez ce code à l'entrée du festival"</p>
                            <Suspense fallback=move || view! { <div class="qr-placeholder"></div> }>
                                {move || Suspend::new(async move {
                                    match qr_resource.await {
                                        Ok(svg) => view! {
                                            <div class="qr-wrapper" inner_html=svg></div>
                                        }.into_any(),
                                        Err(_) => view! {
                                            <p class="muted">"QR code indisponible"</p>
                                        }.into_any(),
                                    }
                                })}
                            </Suspense>
                            <p class="qr-name">{prenom2}" · "{email}</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
