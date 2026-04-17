use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::seo::SeoHead;
use crate::auth::Login;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();
    let value = login_action.value();

    Effect::new(move || {
        if let Some(Ok(_)) = value.get() {
            let _ = window().location().set_href("/");
        }
    });

    view! {
        <SeoHead
            title="Connexion"
            description="Connectez-vous à votre espace Solimouv'."
            path="/connexion"
        />

        <section class="auth-page">
            <div class="auth-card">
                <div class="auth-header">
                    <h1>"Connexion"</h1>
                    <p class="auth-subtitle">"Accédez à votre espace Solimouv'"</p>
                </div>

                <ActionForm action=login_action attr:class="auth-form">
                    <div class="form-group">
                        <label for="login-email">"Email"</label>
                        <input
                            type="email"
                            id="login-email"
                            name="email"
                            required
                            autocomplete="email"
                            placeholder="vous@exemple.com"
                        />
                    </div>
                    <div class="form-group">
                        <label for="login-password">"Mot de passe"</label>
                        <input
                            type="password"
                            id="login-password"
                            name="password"
                            required
                            autocomplete="current-password"
                            minlength=6
                            placeholder="••••••••"
                        />
                    </div>

                    {move || {
                        value.get().and_then(|r| r.err()).map(|e| {
                            view! {
                                <p class="auth-error">{e.to_string()}</p>
                            }
                        })
                    }}

                    <button type="submit" class="btn btn-glow auth-submit" disabled=move || login_action.pending().get()>
                        {move || if login_action.pending().get() { "Connexion..." } else { "Se connecter" }}
                    </button>
                </ActionForm>

                <p class="auth-switch">
                    "Pas encore de compte ? "
                    <A href="/inscription">"Créer un compte"</A>
                </p>
            </div>
        </section>
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no window")
}
