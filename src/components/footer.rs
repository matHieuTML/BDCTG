use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container footer-inner">
                <div class="footer-brand">
                    <p class="footer-title">"Solimouv'"</p>
                    <p class="footer-tagline">"Le festival du sport pour tous, par Up Sport!"</p>
                </div>
                <nav class="footer-nav" aria-label="Pied de page">
                    <ul>
                        <li><A href="/a-propos">"À propos"</A></li>
                        <li><A href="/programme">"Programme"</A></li>
                        <li><A href="/associations">"Associations"</A></li>
                        <li><A href="/contact">"Contact"</A></li>
                    </ul>
                </nav>
                <p class="footer-meta">
                    "© 2026 Up Sport! — Association loi 1901 — "
                    <a href="https://www.unispourlesport.paris/" rel="noopener" target="_blank">
                        "unispourlesport.paris"
                    </a>
                </p>
            </div>
        </footer>
    }
}
