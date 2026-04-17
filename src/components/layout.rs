use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{footer::Footer, header::Header};

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <a class="skip-link" href="#main-content">"Aller au contenu"</a>
        <Header />
        <main id="main-content" class="site-main">
            <Outlet />
        </main>
        <Footer />
    }
}
