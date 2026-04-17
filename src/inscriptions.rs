use leptos::prelude::*;
use server_fn::ServerFnError;

#[server(JoinEvent, "/api")]
pub async fn join_event() -> Result<bool, ServerFnError> {
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool indisponible"))?;

    let user = crate::auth::get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Non connecté"))?;

    let user_id: uuid::Uuid = user.id.parse()
        .map_err(|_| ServerFnError::new("ID invalide"))?;

    let existing: Option<(uuid::Uuid,)> = sqlx::query_as(
        "SELECT id FROM inscriptions WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    if existing.is_some() {
        return Ok(true);
    }

    sqlx::query("INSERT INTO inscriptions (user_id) VALUES ($1)")
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    Ok(true)
}

#[server(IsRegistered, "/api")]
pub async fn is_registered() -> Result<bool, ServerFnError> {
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool indisponible"))?;

    let user = match crate::auth::get_current_user().await? {
        Some(u) => u,
        None => return Ok(false),
    };

    let user_id: uuid::Uuid = user.id.parse()
        .map_err(|_| ServerFnError::new("ID invalide"))?;

    let row: Option<(uuid::Uuid,)> = sqlx::query_as(
        "SELECT id FROM inscriptions WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    Ok(row.is_some())
}

#[server(GenerateQrSvg, "/api")]
pub async fn generate_qr_svg() -> Result<String, ServerFnError> {
    use qrcode::QrCode;
    use qrcode::render::svg;

    let user = crate::auth::get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Non connecté"))?;

    let data = format!(
        "SOLIMOUV|{}|{}|{}",
        user.id, user.prenom, user.email
    );

    let code = QrCode::new(data.as_bytes())
        .map_err(|e| ServerFnError::new(format!("QR: {e}")))?;

    let svg_str = code.render::<svg::Color>()
        .min_dimensions(200, 200)
        .max_dimensions(300, 300)
        .dark_color(svg::Color("#0d0d1a"))
        .light_color(svg::Color("#ffffff"))
        .build();

    Ok(svg_str)
}
