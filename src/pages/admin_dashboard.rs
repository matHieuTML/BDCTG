use leptos::prelude::*;

use crate::data::ACTIVITIES;

struct AtelierStat {
    slug: &'static str,
    inscrits: u32,
    capacity: u32,
    historique: [u32; 6],
}

const STATS: &[AtelierStat] = &[
    AtelierStat { slug: "basket-fauteuil", inscrits: 17, capacity: 20, historique: [12, 14, 11, 16, 18, 17] },
    AtelierStat { slug: "escalade-adaptive", inscrits: 15, capacity: 15, historique: [8, 10, 13, 12, 14, 15] },
    AtelierStat { slug: "danse-inclusive", inscrits: 19, capacity: 25, historique: [15, 17, 20, 22, 18, 19] },
    AtelierStat { slug: "secourisme-express", inscrits: 22, capacity: 30, historique: [18, 20, 25, 23, 21, 22] },
    AtelierStat { slug: "atelier-handicap", inscrits: 31, capacity: 40, historique: [22, 26, 28, 30, 34, 31] },
    AtelierStat { slug: "tir-arc-adapte", inscrits: 12, capacity: 12, historique: [6, 9, 10, 11, 12, 12] },
    AtelierStat { slug: "yoga-accessible", inscrits: 14, capacity: 20, historique: [8, 10, 12, 15, 13, 14] },
    AtelierStat { slug: "quiz-sport-inclusif", inscrits: 67, capacity: 100, historique: [35, 42, 50, 55, 61, 67] },
];

fn find_activity(slug: &str) -> Option<&'static crate::data::Activity> {
    ACTIVITIES.iter().find(|a| a.slug == slug)
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let total_inscrits: u32 = STATS.iter().map(|s| s.inscrits).sum();
    let total_capacity: u32 = STATS.iter().map(|s| s.capacity).sum();
    let taux_global = (total_inscrits as f64 / total_capacity as f64 * 100.0) as u32;

    let full_ateliers: Vec<_> = STATS.iter()
        .filter(|s| s.inscrits >= s.capacity)
        .filter_map(|s| find_activity(s.slug).map(|a| a.name))
        .collect();

    let has_alerts = !full_ateliers.is_empty();
    let alert_names = full_ateliers.join(", ");

    let mois = ["Nov", "Déc", "Jan", "Fév", "Mar", "Avr"];

    view! {
        <div class="admin">
            {if has_alerts {
                Some(view! {
                    <div class="admin-alert">
                        <div class="admin-alert__icon">"⚠️"</div>
                        <div class="admin-alert__body">
                            <strong>"Capacité atteinte"</strong>
                            <span>" — "{alert_names}" : complet(s). Envisagez d'augmenter la capacité ou de rediriger les participants."</span>
                        </div>
                    </div>
                })
            } else {
                None
            }}

            <div class="admin-header">
                <div>
                    <h1 class="admin-header__title">"Dashboard administrateur"</h1>
                    <p class="admin-header__sub">"Vue d'ensemble du festival Solimouv' — 17 avril 2026"</p>
                </div>
            </div>

            <div class="admin-kpis">
                <div class="kpi kpi--coral">
                    <span class="kpi__number">{total_inscrits}</span>
                    <span class="kpi__label">"Inscrits total"</span>
                </div>
                <div class="kpi kpi--sky">
                    <span class="kpi__number">"8"</span>
                    <span class="kpi__label">"Ateliers actifs"</span>
                </div>
                <div class="kpi kpi--mint">
                    <span class="kpi__number">{taux_global}"%"</span>
                    <span class="kpi__label">"Taux de remplissage"</span>
                </div>
                <div class="kpi kpi--warn">
                    <span class="kpi__number">{full_ateliers.len()}</span>
                    <span class="kpi__label">"Ateliers complets"</span>
                </div>
            </div>

            <div class="admin-grid">
                <section class="admin-panel admin-panel--wide">
                    <h2 class="admin-panel__title">"Affluence par atelier"</h2>
                    <div class="bars-chart">
                        {STATS.iter().map(|stat| {
                            let activity = find_activity(stat.slug);
                            let name = activity.map(|a| a.name).unwrap_or(stat.slug);
                            let icon = activity.map(|a| a.icon).unwrap_or("📌");
                            let pct = (stat.inscrits as f64 / stat.capacity as f64 * 100.0).min(100.0) as u32;
                            let is_full = stat.inscrits >= stat.capacity;
                            let bar_class = if is_full { "bar-row bar-row--full" } else { "bar-row" };

                            view! {
                                <div class=bar_class>
                                    <div class="bar-row__label">
                                        <span class="bar-row__icon">{icon}</span>
                                        <span class="bar-row__name">{name}</span>
                                        {if is_full {
                                            Some(view! { <span class="bar-row__badge">"COMPLET"</span> })
                                        } else {
                                            None
                                        }}
                                    </div>
                                    <div class="bar-row__track">
                                        <div
                                            class="bar-row__fill"
                                            style=format!("width: {}%", pct)
                                        ></div>
                                    </div>
                                    <span class="bar-row__count">{stat.inscrits}" / "{stat.capacity}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>

                <section class="admin-panel">
                    <h2 class="admin-panel__title">"Historique inscriptions"</h2>
                    <p class="admin-panel__sub">"Évolution sur 6 mois"</p>

                    <div class="sparklines">
                        {STATS.iter().map(|stat| {
                            let activity = find_activity(stat.slug);
                            let name = activity.map(|a| a.name).unwrap_or(stat.slug);
                            let icon = activity.map(|a| a.icon).unwrap_or("📌");
                            let max_val = *stat.historique.iter().max().unwrap_or(&1);
                            let is_full = stat.inscrits >= stat.capacity;

                            view! {
                                <div class=if is_full { "spark spark--alert" } else { "spark" }>
                                    <div class="spark__header">
                                        <span>{icon}" "{name}</span>
                                        <span class="spark__current">{stat.inscrits}</span>
                                    </div>
                                    <div class="spark__bars">
                                        {stat.historique.iter().enumerate().map(|(j, &val)| {
                                            let h = (val as f64 / max_val as f64 * 100.0) as u32;
                                            let is_last = j == stat.historique.len() - 1;
                                            view! {
                                                <div class="spark__col" title=format!("{}: {}", mois[j], val)>
                                                    <div
                                                        class=if is_last && is_full { "spark__bar spark__bar--alert" } else if is_last { "spark__bar spark__bar--current" } else { "spark__bar" }
                                                        style=format!("height: {}%", h)
                                                    ></div>
                                                    <span class="spark__month">{mois[j]}</span>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>
            </div>
        </div>
    }
}
