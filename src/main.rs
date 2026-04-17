#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use solimouv::app::{shell, App};
    use solimouv::fileserv::file_and_error_handler;
    use solimouv::server::db::init_pool;
    use solimouv::server::seo::{robots_handler, sitemap_handler};
    use tower_http::compression::CompressionLayer;
    use tower_http::trace::TraceLayer;

    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "solimouv=info,tower_http=info,sqlx=warn".into()),
        )
        .init();

    let pool = init_pool()
        .await
        .expect("Failed to initialize Postgres pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations appliquées avec succès.");

    let conf = get_configuration(None).expect("Failed to read Leptos config");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let pool_for_ctx = pool.clone();
    let app = Router::new()
        .route("/sitemap.xml", get(sitemap_handler))
        .route("/robots.txt", get(robots_handler))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(pool_for_ctx.clone()),
            {
                let opts = leptos_options.clone();
                move || shell(opts.clone())
            },
        )
        .fallback(file_and_error_handler)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    tracing::info!("🚀 Serveur Solimouv' en écoute sur http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server crashed");
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Binaire vide en mode WASM — l'entry point hydrate est dans lib.rs
}
