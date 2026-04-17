use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};

use crate::components::layout::Layout;
use crate::pages::{
    about::AboutPage, associations::AssociationsPage, contact::ContactPage,
    dashboard::DashboardPage, home::HomePage, login::LoginPage, not_found::NotFoundPage,
    programme::ProgrammePage, register::RegisterPage,
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
        <Meta name="theme-color" content="#FC547D" />
        <Link rel="manifest" href="/manifest.json" />
        <Link rel="icon" href="/favicon.ico" />
        <Link rel="apple-touch-icon" href="/icons/apple-touch-icon.png" />

        <Router>
            <Routes fallback=NotFoundPage>
                <ParentRoute path=StaticSegment("") view=Layout>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("a-propos") view=AboutPage />
                    <Route path=StaticSegment("programme") view=ProgrammePage />
                    <Route path=StaticSegment("associations") view=AssociationsPage />
                    <Route path=StaticSegment("contact") view=ContactPage />
                    <Route path=StaticSegment("connexion") view=LoginPage />
                    <Route path=StaticSegment("inscription") view=RegisterPage />
                    <Route path=StaticSegment("mon-espace") view=DashboardPage />
                </ParentRoute>
            </Routes>
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
