use leptos::prelude::*;
use server_fn::ServerFnError;

use crate::models::UserInfo;

#[server(Register, "/api")]
pub async fn register(
    email: String,
    password: String,
    prenom: String,
) -> Result<UserInfo, ServerFnError> {
    use bcrypt::{hash, DEFAULT_COST};
    use leptos_axum::ResponseOptions;
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool indisponible"))?;

    let email = email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(ServerFnError::new("Email invalide"));
    }
    let prenom = prenom.trim().to_string();
    if prenom.is_empty() {
        return Err(ServerFnError::new("Le prénom est requis"));
    }
    if password.len() < 6 {
        return Err(ServerFnError::new("Le mot de passe doit contenir au moins 6 caractères"));
    }

    let exists: Option<(bool,)> =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(&email)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    if exists.map(|r| r.0).unwrap_or(false) {
        return Err(ServerFnError::new("Un compte existe déjà avec cet email"));
    }

    let password_hash = hash(password.as_bytes(), DEFAULT_COST)
        .map_err(|e| ServerFnError::new(format!("Hash: {e}")))?;

    let row: (uuid::Uuid, String, String, String) = sqlx::query_as(
        "INSERT INTO users (email, prenom, password_hash, role) VALUES ($1, $2, $3, 'user') RETURNING id, email, prenom, role",
    )
    .bind(&email)
    .bind(&prenom)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    let session_id: (uuid::Uuid,) =
        sqlx::query_as("INSERT INTO sessions (user_id) VALUES ($1) RETURNING id")
            .bind(row.0)
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Session: {e}")))?;

    let response = use_context::<ResponseOptions>()
        .ok_or_else(|| ServerFnError::new("Pas de ResponseOptions"))?;
    response.insert_header(
        http::header::SET_COOKIE,
        http::HeaderValue::from_str(&format!(
            "session_id={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
            session_id.0
        ))
        .unwrap(),
    );

    Ok(UserInfo {
        id: row.0.to_string(),
        email: row.1,
        prenom: row.2,
        role: row.3,
    })
}

#[server(Login, "/api")]
pub async fn login(email: String, password: String) -> Result<UserInfo, ServerFnError> {
    use bcrypt::verify;
    use leptos_axum::ResponseOptions;
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool indisponible"))?;

    let email = email.trim().to_lowercase();

    let row: Option<(uuid::Uuid, String, String, String, String)> = sqlx::query_as(
        "SELECT id, email, prenom, password_hash, role FROM users WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    let (user_id, user_email, prenom, password_hash, role) =
        row.ok_or_else(|| ServerFnError::new("Email ou mot de passe incorrect"))?;

    let valid = verify(password.as_bytes(), &password_hash)
        .map_err(|e| ServerFnError::new(format!("Verify: {e}")))?;

    if !valid {
        return Err(ServerFnError::new("Email ou mot de passe incorrect"));
    }

    let session_id: (uuid::Uuid,) =
        sqlx::query_as("INSERT INTO sessions (user_id) VALUES ($1) RETURNING id")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Session: {e}")))?;

    let response = use_context::<ResponseOptions>()
        .ok_or_else(|| ServerFnError::new("Pas de ResponseOptions"))?;
    response.insert_header(
        http::header::SET_COOKIE,
        http::HeaderValue::from_str(&format!(
            "session_id={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
            session_id.0
        ))
        .unwrap(),
    );

    Ok(UserInfo {
        id: user_id.to_string(),
        email: user_email,
        prenom,
        role,
    })
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use leptos_axum::ResponseOptions;
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool indisponible"))?;

    if let Some(session_id) = get_session_id_from_cookie().await {
        let _ = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(session_id)
            .execute(&pool)
            .await;
    }

    let response = use_context::<ResponseOptions>()
        .ok_or_else(|| ServerFnError::new("Pas de ResponseOptions"))?;
    response.insert_header(
        http::header::SET_COOKIE,
        http::HeaderValue::from_str("session_id=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0")
            .unwrap(),
    );

    Ok(())
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user() -> Result<Option<UserInfo>, ServerFnError> {
    use sqlx::PgPool;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool indisponible"))?;

    let session_id = match get_session_id_from_cookie().await {
        Some(id) => id,
        None => return Ok(None),
    };

    let row: Option<(uuid::Uuid, String, String, String)> = sqlx::query_as(
        "SELECT u.id, u.email, u.prenom, u.role FROM users u JOIN sessions s ON s.user_id = u.id WHERE s.id = $1 AND s.expires_at > NOW()",
    )
    .bind(session_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB: {e}")))?;

    Ok(row.map(|(id, email, prenom, role)| UserInfo {
        id: id.to_string(),
        email,
        prenom,
        role,
    }))
}

#[cfg(feature = "ssr")]
async fn get_session_id_from_cookie() -> Option<uuid::Uuid> {
    use leptos_axum::extract;

    let headers: http::HeaderMap = extract().await.ok()?;
    let cookie_header = headers.get(http::header::COOKIE)?.to_str().ok()?;

    for part in cookie_header.split(';') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("session_id=") {
            return value.parse::<uuid::Uuid>().ok();
        }
    }
    None
}
