use leptos::prelude::*;

use crate::components::seo::SeoHead;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <SeoHead
            title="À propos · Up Sport!"
            description="Up Sport! est une association parisienne fondée en 2016 qui rend le sport accessible à tous. Solimouv' est son festival inclusif annuel."
            path="/a-propos"
        />

        <section class="about-hero" aria-labelledby="about-title">
            <div class="about-hero__img-wrap">
                <img
                    src="/about/tempImagePUhZPg-scaled.jpg"
                    alt="Photo de groupe des bénévoles et participants Up Sport! lors d'une séance de tennis de table au centre sportif"
                    class="about-hero__img"
                    width="1200"
                    height="800"
                />
                <div class="about-hero__overlay"></div>
            </div>
            <div class="container about-hero__content">
                <h1 id="about-title">"Up Sport! & Solimouv'"</h1>
                <p class="about-hero__sub">"L'inclusion en mouvement depuis 2016"</p>
            </div>
        </section>

        // --- Chiffres clés ---
        <section class="about-stats" aria-label="Chiffres clés">
            <div class="container">
                <ul class="about-stats__grid" role="list">
                    <li>
                        <span class="about-stats__number">"250"</span>
                        <span class="about-stats__label">"personnes accompagnées / semaine"</span>
                    </li>
                    <li>
                        <span class="about-stats__number">"30"</span>
                        <span class="about-stats__label">"séances hebdomadaires"</span>
                    </li>
                    <li>
                        <span class="about-stats__number">"500+"</span>
                        <span class="about-stats__label">"participants au festival"</span>
                    </li>
                    <li>
                        <span class="about-stats__number">"13"</span>
                        <span class="about-stats__label">"associations partenaires"</span>
                    </li>
                </ul>
            </div>
        </section>

        // --- Contenu principal ---
        <section class="container about-body">
            <div class="about-section">
                <h2>"L'association Up Sport!"</h2>
                <p>
                    "Créée en 2016, Up Sport! est une association sportive et solidaire parisienne "
                    "qui utilise le sport comme levier d'inclusion sociale. Composée de "
                    <strong>"5 salariés et une vingtaine de bénévoles"</strong>
                    ", elle accompagne chaque semaine des personnes en situation de vulnérabilité "
                    "à travers des activités régulières : badminton, futsal, basketball, zumba, "
                    "natation, course à pied, yoga, volleyball et bien d'autres."
                </p>
                <h3>"Quatre programmes structurants"</h3>
                <ul class="about-pills" role="list" aria-label="Programmes">
                    <li class="about-pill about-pill--coral">"Public exilé"</li>
                    <li class="about-pill about-pill--sky">"Public féminin"</li>
                    <li class="about-pill about-pill--mint">"Activité physique adaptée"</li>
                    <li class="about-pill about-pill--coral">"Insertion professionnelle"</li>
                </ul>
                <p class="about-muted">
                    "En février 2026, l'association a inauguré son "
                    <strong>"Espace Sport Santé Société"</strong>
                    " au 10 avenue de la Porte de Ménilmontant (Paris 20ᵉ), "
                    "un lieu dédié au mouvement, à la santé et au lien social, accessible à tous les publics."
                </p>
            </div>

            <div class="about-section">
                <h2>"Le festival Solimouv'"</h2>
                <p>
                    "Dans une ville où 20 à 30 % des habitants ne pratiquent pas d'activité physique régulière, "
                    "Solimouv' ambitionne de démocratiser l'accès au sport et de lutter contre toutes les formes "
                    "de discrimination : grossophobie, stéréotypes de genre, discriminations LGBTQIA+."
                </p>
                <p>
                    "La 1ʳᵉ édition s'est tenue le "
                    <strong>"12 juillet 2025"</strong>
                    " au Centre Sportif Charles Moureu (Paris 13ᵉ). Au programme : "
                    "sports collectifs (football, handball, basketball, cricket), fitness (zumba, tai-chi, muay-thai, yoga), "
                    "sports adaptés (cécifoot, baskin, boccia) et ateliers de sensibilisation."
                </p>
                <div class="about-results">
                    <div class="about-results__item">
                        <span class="about-results__pct">"92%"</span>
                        <span>"souhaitent revenir"</span>
                    </div>
                    <div class="about-results__item">
                        <span class="about-results__pct">"85%"</span>
                        <span>"ont découvert de nouvelles activités"</span>
                    </div>
                    <div class="about-results__item">
                        <span class="about-results__pct">"78%"</span>
                        <span>"se sentent renforcés dans leur inclusion"</span>
                    </div>
                </div>
            </div>

            <div class="about-section">
                <h2>"Nos valeurs"</h2>
                <ul class="about-values" role="list">
                    <li>
                        <span class="about-values__icon" aria-hidden="true">"🤝"</span>
                        <div>
                            <strong>"Mixité & Diversité"</strong>
                            <p>"Toutes origines, tous parcours, tous niveaux — le sport rassemble"</p>
                        </div>
                    </li>
                    <li>
                        <span class="about-values__icon" aria-hidden="true">"💛"</span>
                        <div>
                            <strong>"Solidarité"</strong>
                            <p>"Le sport comme vecteur de lien social et de transformation"</p>
                        </div>
                    </li>
                    <li>
                        <span class="about-values__icon" aria-hidden="true">"🏅"</span>
                        <div>
                            <strong>"Inclusion"</strong>
                            <p>"Un accès au sport pour chacun(e), sans condition ni jugement"</p>
                        </div>
                    </li>
                    <li>
                        <span class="about-values__icon" aria-hidden="true">"🌱"</span>
                        <div>
                            <strong>"Bienveillance"</strong>
                            <p>"Un cadre de confiance où chaque personne peut s'épanouir"</p>
                        </div>
                    </li>
                </ul>
            </div>

            <div class="about-section about-section--partners">
                <h2>"Le collectif Solimouv'"</h2>
                <p>
                    "Le festival est porté par un collectif de "
                    <strong>"13 structures associatives"</strong>
                    " mobilisées pour un sport ouvert à tous :"
                </p>
                <ul class="about-partners" role="list" aria-label="Associations partenaires">
                    <li>"Up Sport!"</li>
                    <li>"Yoga & Sport with Refugees"</li>
                    <li>"Les Hijabeuses"</li>
                    <li>"NOVOSPORT"</li>
                    <li>"MooveToi"</li>
                    <li>"Front Runners Paris"</li>
                    <li>"Sine Qua Non"</li>
                    <li>"Crewsing"</li>
                    <li>"UFOLEP"</li>
                    <li>"Fondation PiLeJe"</li>
                    <li>"Fondation Força"</li>
                    <li>"Kainoss"</li>
                    <li>"et d'autres…"</li>
                </ul>
            </div>
        </section>

        // --- PWA Install ---
        <section class="about-pwa" aria-labelledby="pwa-title">
            <div class="container about-pwa__inner">
                <div class="about-pwa__text">
                    <h2 id="pwa-title">"Solimouv' dans votre poche"</h2>
                    <p>
                        "Installez l'application Solimouv' sur votre téléphone pour accéder au programme, "
                        "vous inscrire et ne rien manquer du festival — même hors connexion."
                    </p>
                    <div class="about-pwa__steps">
                        <div class="about-pwa__step">
                            <span class="about-pwa__step-num" aria-hidden="true">"1"</span>
                            <p>
                                "Ouvrez "
                                <strong>"solimouv.fly.dev"</strong>
                                " dans votre navigateur"
                            </p>
                        </div>
                        <div class="about-pwa__step">
                            <span class="about-pwa__step-num" aria-hidden="true">"2"</span>
                            <p>
                                "Appuyez sur "
                                <strong>"Partager"</strong>
                                " (iOS) ou "
                                <strong>"⋮ Menu"</strong>
                                " (Android)"
                            </p>
                        </div>
                        <div class="about-pwa__step">
                            <span class="about-pwa__step-num" aria-hidden="true">"3"</span>
                            <p>
                                "Sélectionnez "
                                <strong>"\"Sur l'écran d'accueil\""</strong>
                            </p>
                        </div>
                    </div>
                </div>
                <div class="about-pwa__visual" aria-hidden="true">
                    <div class="about-pwa__phone">
                        <div class="about-pwa__phone-notch"></div>
                        <img
                            src="/icons/icon-192.png"
                            alt=""
                            class="about-pwa__phone-icon"
                            width="192"
                            height="192"
                        />
                        <span class="about-pwa__phone-label">"Solimouv'"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
