use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/solimouv.css" />
        <Title text="Solimouv' — Festival sportif inclusif" />
        <Meta
            name="description"
            content="Festival Solimouv' organisé par Up Sport! — le sport pour tous, quels que soient le genre, l'origine ou le parcours de vie."
        />
        <Meta name="theme-color" content="#0A7EA4" />
        <Link rel="manifest" href="/manifest.json" />
        <Link rel="icon" href="/favicon.ico" />
        <Link rel="apple-touch-icon" href="/icons/icon-192.png" />

        <Router>
            <main>
                <Routes fallback=|| view! { <p>"Page introuvable."</p> }>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>

        <ServiceWorkerRegistration />
    }
}

#[component]
fn ServiceWorkerRegistration() -> impl IntoView {
    view! {
        <script>
            r#"if ('serviceWorker' in navigator) { window.addEventListener('load', () => { navigator.serviceWorker.register('/sw.js').catch(console.error); }); }"#
        </script>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let db_time = Resource::new(|| (), |_| async move { get_db_time().await });

    view! {
        <section class="hero">
            <h1>"Hello Solimouv' 🏅"</h1>
            <p class="tagline">"Le festival du sport pour tous, par Up Sport!"</p>
            <div class="card">
                <h2>"Statut infrastructure"</h2>
                <Suspense fallback=move || view! { <p>"Vérification de la connexion Postgres…"</p> }>
                    {move || Suspend::new(async move {
                        match db_time.await {
                            Ok(time) => view! {
                                <p class="status ok">
                                    <span>"✅ Postgres connecté — heure serveur : "</span>
                                    <code>{time}</code>
                                </p>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="status ko">
                                    "❌ Erreur Postgres : " {e.to_string()}
                                </p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>
            <footer>
                <p>"Spike Leptos 0.7 + Axum + sqlx + Fly.io"</p>
            </footer>
        </section>
    }
}

#[server(GetDbTime, "/api")]
pub async fn get_db_time() -> Result<String, ServerFnError> {
    use crate::server::db::db_now;
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available in server context"))?;

    db_now(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("DB error: {}", e)))
}
