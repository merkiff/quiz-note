use crate::config::SUPABASE_CONFIG;
use crate::models::{Certificate, Question, QuestionOption};
use crate::services::AuthService;
use gloo_net::http::Request;
use serde_json::json;

#[derive(Clone)]
pub struct SupabaseClient;

impl SupabaseClient {
    pub fn new() -> Self {
        Self {}
    }

    async fn get_auth_header(&self) -> Result<String, String> {
        if AuthService::is_token_expired() {
            web_sys::console::log_1(&"Token expired, refreshing...".into());
            AuthService::refresh_token().await?;
        }
        AuthService::get_session()
            .map(|session| format!("Bearer {}", session.access_token))
            .ok_or("로그인이 필요합니다".to_string())
    }

    // --- Certificate CRUD ---

    pub async fn get_all_certificates(&self) -> Result<Vec<Certificate>, String> {
        let auth_header = self.get_auth_header().await?;
        let url = format!("{}/rest/v1/certificates?select=*", SUPABASE_CONFIG.url);

        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            response.json().await.map_err(|e| e.to_string())
        } else {
            Err(format!("자격증 불러오기 실패: {}", response.text().await.unwrap_or_default()))
        }
    }

    pub async fn create_certificate(&self, cert: &Certificate) -> Result<(), String> {
        let auth_header = self.get_auth_header().await?;
        let url = format!("{}/rest/v1/certificates", SUPABASE_CONFIG.url);

        let response = Request::post(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(cert).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("자격증 생성 실패: {}", response.text().await.unwrap_or_default()))
        }
    }

    pub async fn delete_certificate(&self, id: &str) -> Result<(), String> {
        let auth_header = self.get_auth_header().await?;
        let url = format!("{}/rest/v1/certificates?id=eq.{}", SUPABASE_CONFIG.url, id);

        let response = Request::delete(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("자격증 삭제 실패: {}", response.text().await.unwrap_or_default()))
        }
    }


    // --- Question & Option CRUD ---

    pub async fn get_questions_by_certificate(&self, cert_id: &str) -> Result<Vec<Question>, String> {
        let auth_header = self.get_auth_header().await?;
        // 한번의 요청으로 문제와 관련 보기들을 함께 가져옵니다.
        let url = format!(
            "{}/rest/v1/questions?certificate_id=eq.{}&select=*,question_options(*)",
            SUPABASE_CONFIG.url, cert_id
        );

        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            let mut questions: Vec<Question> = response.json().await.map_err(|e| e.to_string())?;
            // 보기들을 display_order 순서로 정렬
            for q in questions.iter_mut() {
                q.options.sort_by_key(|opt| opt.display_order);
            }
            Ok(questions)
        } else {
            Err(format!("문제 불러오기 실패: {}", response.text().await.unwrap_or_default()))
        }
    }

    pub async fn create_question(&self, question: &Question) -> Result<(), String> {
        let auth_header = self.get_auth_header().await?;

        // 1. 문제 생성
        let q_url = format!("{}/rest/v1/questions", SUPABASE_CONFIG.url);
        let q_res = Request::post(&q_url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(question).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;

        if !q_res.ok() {
            return Err(format!("문제 생성 실패: {}", q_res.text().await.unwrap_or_default()));
        }

        // 2. 보기들 생성
        if !question.options.is_empty() {
            let opt_url = format!("{}/rest/v1/question_options", SUPABASE_CONFIG.url);
            let opt_res = Request::post(&opt_url)
                .header("apikey", SUPABASE_CONFIG.anon_key)
                .header("Authorization", &auth_header)
                .header("Content-Type", "application/json")
                .json(&question.options).map_err(|e| e.to_string())?
                .send().await.map_err(|e| e.to_string())?;

            if !opt_res.ok() {
                // 여기서 실패하면 방금 만든 문제를 삭제해주는 것이 좋지만, 일단 오류 메시지만 반환
                return Err(format!("보기 생성 실패: {}", opt_res.text().await.unwrap_or_default()));
            }
        }
        Ok(())
    }
    
    pub async fn update_question_stats(&self, question: &Question) -> Result<(), String> {
        let auth_header = self.get_auth_header().await?;
        let url = format!("{}/rest/v1/questions?id=eq.{}", SUPABASE_CONFIG.url, question.id);
        
        let body = json!({
            "attempt_count": question.attempt_count,
            "correct_count": question.correct_count,
            "last_attempt": question.last_attempt
        });

        let response = Request::patch(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(&body).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;
        
        if response.ok() {
            Ok(())
        } else {
            Err(format!("문제 통계 업데이트 실패: {}", response.text().await.unwrap_or_default()))
        }
    }

    pub async fn delete_question(&self, id: &str) -> Result<(), String> {
        let auth_header = self.get_auth_header().await?;
        let url = format!("{}/rest/v1/questions?id=eq.{}", SUPABASE_CONFIG.url, id);

        let response = Request::delete(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("문제 삭제 실패: {}", response.text().await.unwrap_or_default()))
        }
    }
}