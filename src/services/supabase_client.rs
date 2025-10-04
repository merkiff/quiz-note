use crate::config::SUPABASE_CONFIG;
use crate::models::{Certificate, Question, QuestionOption};
use crate::services::AuthService;
use gloo_net::http::{RequestBuilder, Request};
use serde_json::json;

#[derive(Clone)]
pub struct SupabaseClient;

impl SupabaseClient {
    pub fn new() -> Self {
        Self {}
    }

    async fn get_auth_header_string(&self) -> Result<String, String> {
        if AuthService::is_token_expired() {
            web_sys::console::log_1(&"Token expired, refreshing...".into());
            AuthService::refresh_token().await?;
        }
        AuthService::get_session()
            .map(|session| format!("Bearer {}", session.access_token))
            .ok_or_else(|| "로그인이 필요합니다".to_string())
    }

    async fn request_builder(&self, method: &str, url: &str) -> Result<RequestBuilder, String> {
        let auth_header = self.get_auth_header_string().await?;

        let builder = match method {
            "GET" => Request::get(url),
            "POST" => Request::post(url),
            "PATCH" => Request::patch(url),
            "DELETE" => Request::delete(url),
            _ => return Err("Unsupported HTTP method".to_string()),
        };

        Ok(builder
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header))
    }

    // --- Certificate CRUD ---
    // (get_all_certificates, get_certificate_by_id, create_certificate, delete_certificate 함수는 변경 없음)
    pub async fn get_all_certificates(&self) -> Result<Vec<Certificate>, String> {
        let url = format!("{}/rest/v1/certificates?select=*", SUPABASE_CONFIG.url);
        let response = self.request_builder("GET", &url).await?.send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            response.json().await.map_err(|e| e.to_string())
        } else {
            Err(format!("자격증 불러오기 실패: {}", response.text().await.unwrap_or_default()))
        }
    }
    
    pub async fn get_certificate_by_id(&self, id: &str) -> Result<Certificate, String> {
        let url = format!("{}/rest/v1/certificates?id=eq.{}&select=*&limit=1", SUPABASE_CONFIG.url, id);
        let response = self.request_builder("GET", &url).await?.send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            let mut certs: Vec<Certificate> = response.json().await.map_err(|e| e.to_string())?;
            certs.pop().ok_or_else(|| "해당 자격증을 찾을 수 없습니다.".to_string())
        } else {
            Err(format!("자격증 불러오기 실패: {}", response.text().await.unwrap_or_default()))
        }
    }


    pub async fn create_certificate(&self, cert: &Certificate) -> Result<(), String> {
        let url = format!("{}/rest/v1/certificates", SUPABASE_CONFIG.url);
        let response = self.request_builder("POST", &url).await?
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
        let url = format!("{}/rest/v1/certificates?id=eq.{}", SUPABASE_CONFIG.url, id);
        let response = self.request_builder("DELETE", &url).await?.send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("자격증 삭제 실패: {}", response.text().await.unwrap_or_default()))
        }
    }


    // --- Question & Option CRUD ---
    // (get_questions_by_certificate, get_question_by_id 함수는 변경 없음)
    pub async fn get_questions_by_certificate(&self, cert_id: &str) -> Result<Vec<Question>, String> {
        let url = format!(
            "{}/rest/v1/questions?certificate_id=eq.{}&select=*,question_options(*)",
            SUPABASE_CONFIG.url, cert_id
        );
        let response = self.request_builder("GET", &url).await?.send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            let mut questions: Vec<Question> = response.json().await.map_err(|e| e.to_string())?;
            for q in questions.iter_mut() {
                q.options.sort_by_key(|opt| opt.display_order);
            }
            Ok(questions)
        } else {
            Err(format!("문제 불러오기 실패: {}", response.text().await.unwrap_or_default()))
        }
    }

    pub async fn get_question_by_id(&self, id: &str) -> Result<Question, String> {
        let url = format!(
            "{}/rest/v1/questions?id=eq.{}&select=*,question_options(*)",
            SUPABASE_CONFIG.url, id
        );
        let response = self.request_builder("GET", &url).await?.send().await.map_err(|e| e.to_string())?;

        if response.ok() {
            let mut questions: Vec<Question> = response.json().await.map_err(|e| e.to_string())?;
            if let Some(mut q) = questions.pop() {
                 q.options.sort_by_key(|opt| opt.display_order);
                 Ok(q)
            } else {
                Err("문제를 찾을 수 없습니다.".to_string())
            }
        } else {
            Err(format!("문제 불러오기 실패: {}", response.text().await.unwrap_or_default()))
        }
    }
    
    async fn upsert_options(&self, auth_header: &str, options: &[QuestionOption]) -> Result<(), String> {
        if options.is_empty() { return Ok(()); }
        let url = format!("{}/rest/v1/question_options", SUPABASE_CONFIG.url);
        let response = Request::post(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(options).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("보기 생성/수정 실패: {}", response.text().await.unwrap_or_default()));
        }
        Ok(())
    }

    // ===== 수정된 부분 시작 =====
    pub async fn create_question(&self, question: &Question) -> Result<(), String> {
        let auth_header = self.get_auth_header_string().await?;
        let q_url = format!("{}/rest/v1/questions", SUPABASE_CONFIG.url);

        // questions 테이블에 있는 필드만으로 JSON을 만듭니다.
        let q_body = json!({
            "id": question.id,
            "certificate_id": question.certificate_id,
            "content": question.content,
            "explanation": question.explanation
        });

        let q_res = Request::post(&q_url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(&q_body).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;

        if !q_res.ok() {
            return Err(format!("문제 생성 실패: {}", q_res.text().await.unwrap_or_default()));
        }

        let options_with_id: Vec<QuestionOption> = question.options.iter().map(|opt| {
            let mut new_opt = opt.clone();
            new_opt.question_id = question.id.clone();
            new_opt
        }).collect();
        self.upsert_options(&auth_header, &options_with_id).await
    }
    
    pub async fn update_question(&self, question: &Question) -> Result<(), String> {
        let auth_header = self.get_auth_header_string().await?;

        // 1. 기존 보기들 먼저 삭제 (이 로직은 그대로 유지)
        let del_opt_url = format!("{}/rest/v1/question_options?question_id=eq.{}", SUPABASE_CONFIG.url, question.id);
        let del_res = Request::delete(&del_opt_url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send().await.map_err(|e| e.to_string())?;
        
        if !del_res.ok() {
            return Err(format!("기존 보기 삭제 실패: {}", del_res.text().await.unwrap_or_default()));
        }

        // 2. 문제 내용 업데이트 (테이블에 있는 필드만 전송)
        let q_url = format!("{}/rest/v1/questions?id=eq.{}", SUPABASE_CONFIG.url, question.id);
        let q_body = json!({
            "content": question.content,
            "explanation": question.explanation
        });
        let q_res = Request::patch(&q_url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(&q_body).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;

        if !q_res.ok() {
            return Err(format!("문제 업데이트 실패: {}", q_res.text().await.unwrap_or_default()));
        }

        // 3. 새 보기들 생성 (이 로직은 그대로 유지)
        let options_with_id: Vec<QuestionOption> = question.options.iter().map(|opt| {
            let mut new_opt = opt.clone();
            new_opt.question_id = question.id.clone();
            new_opt
        }).collect();
        self.upsert_options(&auth_header, &options_with_id).await
    }
    // ===== 수정된 부분 끝 =====

    // (update_question_stats, delete_question 함수는 변경 없음)
    pub async fn update_question_stats(&self, question: &Question) -> Result<(), String> {
        let url = format!("{}/rest/v1/questions?id=eq.{}", SUPABASE_CONFIG.url, question.id);
        let body = json!({
            "attempt_count": question.attempt_count,
            "correct_count": question.correct_count,
            "last_attempt": question.last_attempt
        });
        let response = self.request_builder("PATCH", &url).await?
            .header("Content-Type", "application/json")
            .json(&body).map_err(|e| e.to_string())?
            .send().await.map_err(|e| e.to_string())?;

        if response.ok() { Ok(()) } 
        else { Err(format!("문제 통계 업데이트 실패: {}", response.text().await.unwrap_or_default())) }
    }

    pub async fn delete_question(&self, id: &str) -> Result<(), String> {
        let url = format!("{}/rest/v1/questions?id=eq.{}", SUPABASE_CONFIG.url, id);
        let response = self.request_builder("DELETE", &url).await?.send().await.map_err(|e| e.to_string())?;

        if response.ok() { Ok(()) } 
        else { Err(format!("문제 삭제 실패: {}", response.text().await.unwrap_or_default())) }
    }
}