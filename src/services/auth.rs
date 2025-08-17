use crate::config::SUPABASE_CONFIG;
use gloo::storage::{LocalStorage, Storage};
use gloo_net::http::Request;
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

    // Magic Link 로그인 요청
    pub async fn sign_in_with_email(email: &str) -> Result<(), String> {
        let url = format!("{}/auth/v1/otp", SUPABASE_CONFIG.url);

        let redirect_url = if window().unwrap().location().hostname().unwrap() == "localhost" {
            window().unwrap().location().origin().unwrap()
        } else {
            "https://YOUR_USERNAME.github.io/quiz-note".to_string() // 실제 GitHub Pages URL
        };

        let body = serde_json::json!({
            "email": email,
            "type": "magiclink",
            "options": {
                "emailRedirectTo": redirect_url
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
            Err(error)
        }
    }

    // 로그아웃
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

        window().unwrap().location().set_href("/login").unwrap();

        Ok(())
    }

    // 세션 저장
    pub fn save_session(session: Session) {
        let _ = LocalStorage::set(Self::SESSION_KEY, session);
    }

    // 세션 가져오기
    pub fn get_session() -> Option<Session> {
        LocalStorage::get(Self::SESSION_KEY).ok()
    }

    // 로그인 여부 확인
    pub fn is_authenticated() -> bool {
        Self::get_session().is_some()
    }

    // 현재 사용자
    pub fn get_current_user() -> Option<User> {
        Self::get_session().map(|s| s.user)
    }

    // Magic Link 콜백 처리
    pub async fn handle_auth_callback() -> Result<(), String> {
        let location = window().unwrap().location();
        let hash = location.hash().unwrap_or_default();

        if hash.contains("access_token") {
            // URL fragment에서 토큰 추출
            let params: std::collections::HashMap<String, String> = hash
                .trim_start_matches('#')
                .split('&')
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    Some((parts.next()?.to_string(), parts.next()?.to_string()))
                })
                .collect();

            if let (Some(access_token), Some(refresh_token)) =
                (params.get("access_token"), params.get("refresh_token"))
            {
                // 사용자 정보 가져오기
                let user = Self::get_user_info(access_token).await?;

                // 세션 저장
                let session = Session {
                    access_token: access_token.clone(),
                    refresh_token: refresh_token.clone(),
                    user,
                };

                Self::save_session(session);

                // URL 정리
                location.set_hash("").unwrap();

                // 홈으로 이동
                location.set_href("/").unwrap();
            }
        }

        Ok(())
    }

    // 사용자 정보 가져오기
    async fn get_user_info(access_token: &str) -> Result<User, String> {
        let url = format!("{}/auth/v1/user", SUPABASE_CONFIG.url);

        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let user_data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        Ok(User {
            id: user_data["id"].as_str().unwrap_or_default().to_string(),
            email: user_data["email"].as_str().unwrap_or_default().to_string(),
        })
    }
}
