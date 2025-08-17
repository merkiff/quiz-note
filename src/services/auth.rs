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

        // 현재 페이지의 전체 URL 사용 (해시 제외)
        // GitHub Pages 배포 여부 확인
        let location = window().unwrap().location();
        let hostname = location.hostname().unwrap();
        // 명시적으로 전체 URL 설정
        let redirect_url = if hostname == "localhost" {
            "http://localhost:8080/"
        } else {
            "https://merkiff.github.io/quiz-note/" // 전체 경로 포함!
        };

        web_sys::console::log_1(&format!("Email redirect URL: {}", redirect_url).into());

        let body = serde_json::json!({
            "email": email,
            "type": "magiclink",
            "options": {
                "emailRedirectTo": "https://merkiff.github.io/quiz-note/",
                "shouldCreateUser": true
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
        let pathname = location.pathname().unwrap();
        let href = location.href().unwrap();

        // 디버깅 로그
        web_sys::console::log_1(&format!("Current URL: {}", href).into());
        web_sys::console::log_1(&format!("Pathname: {}", pathname).into());
        web_sys::console::log_1(&format!("Hash: {}", hash).into());

        // quiz-note 경로가 아니고 access_token이 있으면 리다이렉트
        if !pathname.contains("quiz-note") && hash.contains("access_token") {
            web_sys::console::log_1(&"Redirecting to quiz-note...".into());

            // window.location.replace 사용 (히스토리에 남지 않음)
            let new_url = format!("https://merkiff.github.io/quiz-note/{}", hash);
            web_sys::console::log_1(&format!("Redirecting to: {}", new_url).into());

            location.replace(&new_url).unwrap_or_else(|e| {
                web_sys::console::error_1(&format!("Redirect failed: {:?}", e).into());
            });

            return Ok(());
        }

        // quiz-note 경로에서 토큰 처리
        if pathname.contains("quiz-note") && hash.contains("access_token") {
            web_sys::console::log_1(&"Processing auth token...".into());

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
                web_sys::console::log_1(&"Tokens found, getting user info...".into());

                // 사용자 정보 가져오기
                let user = Self::get_user_info(access_token).await?;

                // 세션 저장
                let session = Session {
                    access_token: access_token.clone(),
                    refresh_token: refresh_token.clone(),
                    user,
                };

                Self::save_session(session);

                web_sys::console::log_1(&"Session saved successfully".into());

                // URL 정리 - 해시 제거
                location.set_hash("").unwrap();
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
