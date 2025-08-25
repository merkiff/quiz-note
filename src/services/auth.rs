use crate::config::SUPABASE_CONFIG;
use base64::{engine::general_purpose, Engine as _};
use gloo::storage::{LocalStorage, Storage};
use gloo_net::http::Request;
use js_sys;
use serde::{Deserialize, Serialize};
use web_sys::window;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

pub struct AuthService;

impl AuthService {
    const SESSION_KEY: &'static str = "quiz_note_session";

    pub async fn sign_in_with_email(email: &str) -> Result<(), String> {
        let url = format!("{}/auth/v1/otp", SUPABASE_CONFIG.url);

        // 현재 페이지 URL을 기반으로 리디렉션 URL을 동적으로 생성
        let location = window().unwrap().location();
        let redirect_url = format!(
            "{}//{}",
            location.protocol().unwrap(),
            location.host().unwrap()
        );

        web_sys::console::log_1(&format!("Email redirect URL: {}", redirect_url).into());

        let body = serde_json::json!({
            "email": email,
            "options": {
                "emailRedirectTo": redirect_url,
            }
        });

        let response = Request::post(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.ok() {
            Ok(())
        } else {
            let error = response
                .text()
                .await
                .unwrap_or_else(|_| "로그인 요청 실패".to_string());
            web_sys::console::error_1(&format!("Supabase error: {}", error).into());
            Err(error)
        }
    }

    pub async fn sign_out() -> Result<(), String> {
        if let Some(session) = Self::get_session() {
            let url = format!("{}/auth/v1/logout", SUPABASE_CONFIG.url);
            let _ = Request::post(&url)
                .header("apikey", SUPABASE_CONFIG.anon_key)
                .header("Authorization", &format!("Bearer {}", session.access_token))
                .send()
                .await;
        }

        LocalStorage::delete(Self::SESSION_KEY);
        // 페이지를 새로고침하여 로그인 페이지로 리디렉션
        window().unwrap().location().reload().unwrap();
        Ok(())
    }

    pub fn save_session(session: Session) {
        let _ = LocalStorage::set(Self::SESSION_KEY, session);
    }

    pub fn get_session() -> Option<Session> {
        LocalStorage::get(Self::SESSION_KEY).ok()
    }

    pub fn is_authenticated() -> bool {
        !Self::is_token_expired()
    }

    pub fn get_current_user() -> Option<User> {
        Self::get_session().map(|s| s.user)
    }

    pub async fn handle_auth_callback() -> Result<bool, String> {
        let location = window().unwrap().location();
        let hash = location.hash().unwrap_or_default();

        if !hash.contains("access_token") {
            return Ok(false);
        }

        let params: std::collections::HashMap<String, String> = hash
            .trim_start_matches('#')
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.split('=');
                Some((
                    parts.next()?.to_string(),
                    parts.next()?.replace("%23", "#").to_string(), // Handle encoded '#'
                ))
            })
            .collect();

        if let (Some(access_token), Some(refresh_token)) =
            (params.get("access_token"), params.get("refresh_token"))
        {
            let user = Self::get_user_info(access_token).await?;
            let session = Session {
                access_token: access_token.clone(),
                refresh_token: refresh_token.clone(),
                user,
            };
            Self::save_session(session);

            // URL에서 토큰 정보 제거
            location.set_hash("").unwrap();
            return Ok(true);
        }

        Ok(false)
    }

    async fn get_user_info(access_token: &str) -> Result<User, String> {
        let url = format!("{}/auth/v1/user", SUPABASE_CONFIG.url);
        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err("Failed to get user info".to_string());
        }

        response.json::<User>().await.map_err(|e| e.to_string())
    }

    pub async fn refresh_token() -> Result<(), String> {
        let session = Self::get_session().ok_or("No session found")?;
        let url = format!(
            "{}/auth/v1/token?grant_type=refresh_token",
            SUPABASE_CONFIG.url
        );
        let body = serde_json::json!({ "refresh_token": session.refresh_token });

        let response = Request::post(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.ok() {
            let auth_response: serde_json::Value =
                response.json().await.map_err(|e| e.to_string())?;
            if let (Some(access_token), Some(refresh_token)) = (
                auth_response["access_token"].as_str(),
                auth_response["refresh_token"].as_str(),
            ) {
                let mut updated_session = session;
                updated_session.access_token = access_token.to_string();
                updated_session.refresh_token = refresh_token.to_string();
                Self::save_session(updated_session);
                Ok(())
            } else {
                Err("Failed to parse token refresh response".to_string())
            }
        } else {
            LocalStorage::delete(Self::SESSION_KEY);
            Err("Session expired. Please log in again.".to_string())
        }
    }

    pub fn is_token_expired() -> bool {
        let session = match Self::get_session() {
            Some(s) => s,
            None => return true,
        };

        let parts: Vec<&str> = session.access_token.split('.').collect();
        if parts.len() != 3 {
            return true;
        }

        let payload = match general_purpose::URL_SAFE_NO_PAD.decode(parts[1]) {
            Ok(p) => p,
            Err(_) => return true,
        };

        let claims: serde_json::Value = match serde_json::from_slice(&payload) {
            Ok(c) => c,
            Err(_) => return true,
        };

        claims["exp"]
            .as_i64()
            .map(|exp| (js_sys::Date::now() / 1000.0) as i64 > exp)
            .unwrap_or(true)
    }
}