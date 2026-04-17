use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::auth::{get_current_user, Logout};

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    let user_resource = Resource::new(
        move || location.pathname.get(),
        |_| async move { get_current_user().await },
    );
    let logout_action = ServerAction::<Logout>::new();
    let (menu_open, set_menu_open) = signal(false);

    Effect::new(move || {
        if let Some(Ok(())) = logout_action.value().get() {
            let _ = web_sys::window()
                .expect("window")
                .location()
                .set_href("/");
        }
    });

    let close_menu = move |_| set_menu_open.set(false);

    view! {
        <header class="site-header">
            <div class="container header-inner">
                <A href="/" attr:class="brand" attr:aria-label="Solimouv' — retour à l'accueil">
                    <img
                        src="/LogoHeader.png"
                        alt="Solimouv'"
                        class="brand-logo"
                        width="344"
                        height="165"
                    />
                </A>

                <button
                    class="nav-toggle"
                    aria-label="Ouvrir le menu"
                    aria-expanded=move || if menu_open.get() { "true" } else { "false" }
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                >
                    <span class=move || if menu_open.get() { "nav-toggle__bar nav-toggle__bar--open" } else { "nav-toggle__bar" }></span>
                </button>

                <nav
                    class=move || if menu_open.get() { "site-nav site-nav--open" } else { "site-nav" }
                    aria-label="Navigation principale"
                >
                    <ul>
                        <li><A href="/" exact=true on:click=close_menu>"Accueil"</A></li>
                        <li><A href="/a-propos" on:click=close_menu>"À propos"</A></li>
                        <li><A href="/programme" on:click=close_menu>"Programme"</A></li>
                        <li><A href="/associations" on:click=close_menu>"Associations"</A></li>
                        <li><A href="/contact" on:click=close_menu>"Contact"</A></li>
                    </ul>
                </nav>

                <div class=move || if menu_open.get() { "header-auth header-auth--visible" } else { "header-auth" }>
                    <Suspense fallback=move || view! { <span class="auth-placeholder"></span> }>
                        {move || Suspend::new(async move {
                            match user_resource.await {
                                Ok(Some(user)) => view! {
                                    <div class="auth-user">
                                        <A href="/mon-espace" attr:class="btn-text auth-greeting">{user.prenom}</A>
                                        <ActionForm action=logout_action>
                                            <button type="submit" class="btn-text">"Déconnexion"</button>
                                        </ActionForm>
                                    </div>
                                }.into_any(),
                                _ => view! {
                                    <div class="auth-links">
                                        <A href="/connexion" attr:class="btn-text">"Connexion"</A>
                                        <A href="/inscription" attr:class="btn btn-glow btn-sm">"Inscription"</A>
                                    </div>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </header>

        // Overlay to close menu on tap outside
        {move || menu_open.get().then(|| view! {
            <div class="nav-overlay" on:click=move |_| set_menu_open.set(false)></div>
        })}
    }
}
