use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::seo::SeoHead;
use crate::auth::Register;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let register_action = ServerAction::<Register>::new();
    let value = register_action.value();

    Effect::new(move || {
        if let Some(Ok(_)) = value.get() {
            let _ = window().location().set_href("/");
        }
    });

    view! {
        <SeoHead
            title="Inscription"
            description="Créez votre compte Solimouv' et rejoignez le mouvement."
            path="/inscription"
        />

        <section class="auth-page">
            <div class="auth-card">
                <div class="auth-header">
                    <h1>"Créer un compte"</h1>
                    <p class="auth-subtitle">"Rejoignez le festival Solimouv'"</p>
                </div>

                <ActionForm action=register_action attr:class="auth-form">
                    <div class="form-group">
                        <label for="reg-prenom">"Prénom"</label>
                        <input
                            type="text"
                            id="reg-prenom"
                            name="prenom"
                            required
                            autocomplete="given-name"
                            placeholder="Votre prénom"
                        />
                    </div>
                    <div class="form-group">
                        <label for="reg-email">"Email"</label>
                        <input
                            type="email"
                            id="reg-email"
                            name="email"
                            required
                            autocomplete="email"
                            placeholder="vous@exemple.com"
                        />
                    </div>
                    <div class="form-group">
                        <label for="reg-password">"Mot de passe"</label>
                        <input
                            type="password"
                            id="reg-password"
                            name="password"
                            required
                            autocomplete="new-password"
                            minlength=6
                            placeholder="6 caractères minimum"
                        />
                    </div>

                    {move || {
                        value.get().and_then(|r| r.err()).map(|e| {
                            view! {
                                <p class="auth-error">{e.to_string()}</p>
                            }
                        })
                    }}

                    <button type="submit" class="btn btn-glow auth-submit" disabled=move || register_action.pending().get()>
                        {move || if register_action.pending().get() { "Création..." } else { "Créer mon compte" }}
                    </button>
                </ActionForm>

                <p class="auth-switch">
                    "Déjà inscrit ? "
                    <A href="/connexion">"Se connecter"</A>
                </p>
            </div>
        </section>
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no window")
}
