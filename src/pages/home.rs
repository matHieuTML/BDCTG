use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::seo::SeoHead;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <SeoHead
            title="Le festival du sport pour tous"
            description="Solimouv' — le festival sportif inclusif organisé par Up Sport! à Paris. Découvrez le programme, les associations partenaires et trouvez votre sport."
            path="/"
        />
        <SchemaOrgEvent />

        // ---- HERO full-bleed ----
        <section class="hero" aria-labelledby="hero-title">
            <div class="hero-bg" aria-hidden="true">
                <div class="hero-orb hero-orb--coral"></div>
                <div class="hero-orb hero-orb--sky"></div>
                <div class="hero-orb hero-orb--mint"></div>
                <div class="hero-grid-lines" aria-hidden="true"></div>
            </div>
            <div class="hero-content container">
                <p class="hero-eyebrow">"Festival 2026 · Paris · 17 Avril"</p>
                <h1 id="hero-title" class="hero-title">
                    "Solimouv'"
                </h1>
                <p class="hero-subtitle">"Le festival du sport pour tous"</p>
                <p class="hero-lead">
                    "Un terrain de jeu où chaque personne a sa place — quels que soient "
                    "le genre, l'origine ou le parcours de vie."
                </p>
                <div class="hero-ctas">
                    <A href="/programme" attr:class="btn btn-glow">"Découvrir le programme"</A>
                    <A href="/associations" attr:class="btn btn-outline-light">"Nos partenaires"</A>
                </div>
            </div>
        </section>

        // ---- STATS band ----
        <section class="stats-band" aria-labelledby="stats-title">
            <div class="container">
                <h2 id="stats-title" class="visually-hidden">"Chiffres clés"</h2>
                <ul class="stats-row">
                    <li class="stat-item">
                        <span class="stat-number">"500+"</span>
                        <span class="stat-desc">"participants"</span>
                    </li>
                    <li class="stat-divider" aria-hidden="true"></li>
                    <li class="stat-item">
                        <span class="stat-number">"13"</span>
                        <span class="stat-desc">"associations"</span>
                    </li>
                    <li class="stat-divider" aria-hidden="true"></li>
                    <li class="stat-item">
                        <span class="stat-number">"92%"</span>
                        <span class="stat-desc">"veulent revenir"</span>
                    </li>
                    <li class="stat-divider" aria-hidden="true"></li>
                    <li class="stat-item">
                        <span class="stat-number">"78%"</span>
                        <span class="stat-desc">"inclusion renforcée"</span>
                    </li>
                </ul>
            </div>
        </section>

        // ---- MISSION ----
        <section class="mission container" aria-labelledby="mission-title">
            <div class="mission-intro">
                <span class="tag tag--coral">"Notre mission"</span>
                <h2 id="mission-title">"Rendre le sport accessible à toutes et tous"</h2>
                <p class="mission-text">
                    "Up Sport! accompagne chaque semaine "
                    <strong>"250 personnes"</strong>
                    " à travers 30 séances. Solimouv' est la vitrine annuelle de cet engagement — "
                    "un festival où associations, bénévoles et publics se retrouvent."
                </p>
            </div>
        </section>

        // ---- PROGRAMMES ----
        <section class="programs" aria-labelledby="programs-title">
            <div class="container">
                <span class="tag tag--mint">"4 programmes"</span>
                <h2 id="programs-title">"Au cœur de l'action"</h2>
                <div class="programs-grid">
                    <article class="program-card program-card--1">
                        <span class="program-number">"01"</span>
                        <h3>"Public exilé"</h3>
                        <p>"Accompagnement sportif des personnes réfugiées et primo-arrivantes."</p>
                    </article>
                    <article class="program-card program-card--2">
                        <span class="program-number">"02"</span>
                        <h3>"Public féminin"</h3>
                        <p>"Lever les freins à la pratique sportive des femmes."</p>
                    </article>
                    <article class="program-card program-card--3">
                        <span class="program-number">"03"</span>
                        <h3>"Activité physique adaptée"</h3>
                        <p>"Sport adapté pour les personnes en situation de handicap."</p>
                    </article>
                    <article class="program-card program-card--4">
                        <span class="program-number">"04"</span>
                        <h3>"Insertion professionnelle"</h3>
                        <p>"Le sport comme levier de remobilisation et d'insertion."</p>
                    </article>
                </div>
            </div>
        </section>

        // ---- CTA final ----
        <section class="cta-section" aria-labelledby="cta-title">
            <div class="container cta-inner">
                <h2 id="cta-title">"Rejoignez le mouvement"</h2>
                <p>"Bénévole, partenaire, participant — il y a une place pour vous."</p>
                <div class="hero-ctas">
                    <A href="/contact" attr:class="btn btn-glow">"Nous contacter"</A>
                    <A href="/a-propos" attr:class="btn btn-outline-light">"En savoir plus"</A>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SchemaOrgEvent() -> impl IntoView {
    let json_ld = r#"{
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "Organization",
      "@id": "https://solimouv.fly.dev/#organization",
      "name": "Up Sport!",
      "url": "https://www.unispourlesport.paris/",
      "logo": "https://solimouv.fly.dev/icons/icon-512.png",
      "sameAs": [
        "https://www.facebook.com/UpSport.UNis/",
        "https://www.instagram.com/unispourlesport/",
        "https://www.tiktok.com/@upsportparis"
      ]
    },
    {
      "@type": "Event",
      "name": "Solimouv'",
      "description": "Festival du sport pour tous organisé par Up Sport! et un collectif d'associations parisiennes.",
      "eventStatus": "https://schema.org/EventScheduled",
      "eventAttendanceMode": "https://schema.org/OfflineEventAttendanceMode",
      "location": {
        "@type": "Place",
        "name": "Centre Sportif Charles Moureu",
        "address": {
          "@type": "PostalAddress",
          "addressLocality": "Paris",
          "addressCountry": "FR"
        }
      },
      "organizer": { "@id": "https://solimouv.fly.dev/#organization" },
      "url": "https://solimouv.fly.dev/"
    }
  ]
}"#;

    view! {
        <script type="application/ld+json" inner_html=json_ld></script>
    }
}
