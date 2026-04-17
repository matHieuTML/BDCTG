use axum::{http::header, response::IntoResponse};

const BASE_URL: &str = "https://solimouv.fly.dev";
const ROUTES: &[(&str, &str, &str)] = &[
    ("/", "1.0", "weekly"),
    ("/a-propos", "0.8", "monthly"),
    ("/programme", "0.9", "weekly"),
    ("/associations", "0.7", "monthly"),
    ("/contact", "0.5", "yearly"),
];

pub async fn sitemap_handler() -> impl IntoResponse {
    let today = "2026-04-16";
    let mut body = String::with_capacity(2048);
    body.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    body.push('\n');
    body.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    body.push('\n');
    for (path, priority, changefreq) in ROUTES {
        body.push_str(&format!(
            "  <url>\n    <loc>{BASE_URL}{path}</loc>\n    <lastmod>{today}</lastmod>\n    <changefreq>{changefreq}</changefreq>\n    <priority>{priority}</priority>\n  </url>\n"
        ));
    }
    body.push_str("</urlset>\n");

    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        body,
    )
}

pub async fn robots_handler() -> impl IntoResponse {
    let body = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {BASE_URL}/sitemap.xml\n"
    );
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body)
}
