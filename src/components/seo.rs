use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

pub const SITE_BASE_URL: &str = "https://solimouv.fly.dev";
pub const SITE_NAME: &str = "Solimouv'";
pub const DEFAULT_OG_IMAGE: &str = "/icons/icon-512.png";

#[component]
pub fn SeoHead(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] path: String,
    #[prop(into, default = DEFAULT_OG_IMAGE.to_string())] og_image: String,
    #[prop(into, default = "website".to_string())] og_type: String,
) -> impl IntoView {
    let full_title = format!("{title} — {SITE_NAME}");
    let canonical = format!("{SITE_BASE_URL}{path}");
    let image_url = if og_image.starts_with("http") {
        og_image.clone()
    } else {
        format!("{SITE_BASE_URL}{og_image}")
    };

    view! {
        <Title text=full_title.clone() />
        <Meta name="description" content=description.clone() />
        <Link rel="canonical" href=canonical.clone() />

        <Meta property="og:title" content=full_title.clone() />
        <Meta property="og:description" content=description.clone() />
        <Meta property="og:type" content=og_type />
        <Meta property="og:url" content=canonical />
        <Meta property="og:image" content=image_url.clone() />
        <Meta property="og:site_name" content=SITE_NAME />
        <Meta property="og:locale" content="fr_FR" />

        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content=full_title />
        <Meta name="twitter:description" content=description />
        <Meta name="twitter:image" content=image_url />
    }
}
