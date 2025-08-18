use crate::config::SUPABASE_CONFIG;
use crate::services::AuthService;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone)]
pub struct SupabaseClient; // 필드 제거

impl SupabaseClient {
    pub fn new() -> Self {
        Self {} // 빈 구조체 반환
    }

    async fn get_auth_header(&self) -> Result<String, String> {
        // 토큰 만료 확인
        if AuthService::is_token_expired() {
            web_sys::console::log_1(&"Token expired, refreshing...".into());
            AuthService::refresh_token().await?;
        }

        AuthService::get_session()
            .map(|session| format!("Bearer {}", session.access_token))
            .ok_or("로그인이 필요합니다".to_string())
    }
    // 자격증 동기화 (클라우드로 업로드)
    pub async fn sync_certificates_to_cloud(
        &self,
        certificates: &std::collections::HashMap<String, crate::models::Certificate>,
    ) -> Result<(), String> {
        // 인증 확인
        let auth_header = self
            .get_auth_header().await?;
            //("로그인이 필요합니다".to_string())?;

        if !certificates.is_empty() {
            let url = format!("{}/rest/v1/certificates", SUPABASE_CONFIG.url);

            let records: Vec<serde_json::Value> = certificates
                .values()
                .map(|cert| {
                    json!({
                        "id": cert.id,
                        "name": cert.name,
                        "description": cert.description,
                        "question_count": cert.question_count,
                        "created_at": cert.created_at.to_rfc3339(),
                    })
                })
                .collect();

            // UPSERT 사용 (있으면 업데이트, 없으면 삽입)
            let response = Request::post(&url)
                .header("apikey", SUPABASE_CONFIG.anon_key)
                .header("Authorization", &auth_header)
                .header("Content-Type", "application/json")
                .header("Prefer", "resolution=merge-duplicates") // UPSERT 설정
                .json(&records)
                .map_err(|e| format!("요청 생성 실패: {}", e))?
                .send()
                .await
                .map_err(|e| format!("요청 실패: {}", e))?;

            if !response.ok() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(format!("자격증 동기화 실패: {}", error_text));
            }
        }

        Ok(())
    }

    // 자격증 클라우드에서 가져오기
    pub async fn sync_certificates_from_cloud(
        &self,
    ) -> Result<std::collections::HashMap<String, crate::models::Certificate>, String> {
        // 인증 확인
        let auth_header = self
            .get_auth_header().await?;
            //.ok_or("로그인이 필요합니다".to_string())?;

        let url = format!("{}/rest/v1/certificates", SUPABASE_CONFIG.url);

        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send()
            .await
            .map_err(|e| format!("요청 실패: {}", e))?;

        if response.ok() {
            let records: Vec<serde_json::Value> = response
                .json()
                .await
                .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

            let mut certificates = std::collections::HashMap::new();

            for record in records {
                // JSON을 Certificate로 변환
                let cert = crate::models::Certificate {
                    id: record["id"].as_str().unwrap_or_default().to_string(),
                    name: record["name"].as_str().unwrap_or_default().to_string(),
                    description: record["description"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    question_count: record["question_count"].as_u64().unwrap_or(0) as u32,
                    created_at: chrono::DateTime::parse_from_rfc3339(
                        record["created_at"]
                            .as_str()
                            .unwrap_or("2024-01-01T00:00:00Z"),
                    )
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                };

                certificates.insert(cert.id.clone(), cert);
            }

            Ok(certificates)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("자격증 불러오기 실패: {}", error_text))
        }
    }

    // 문제 동기화 (TODO: 구현 필요)
    pub async fn sync_questions_to_cloud(
        &self,
        questions: &std::collections::HashMap<String, crate::models::Question>,
    ) -> Result<(), String> {
        // 인증 확인
        let auth_header = self
            .get_auth_header().await?;
            //.ok_or("로그인이 필요합니다".to_string())?;

        // 먼저 이 사용자의 모든 보기 삭제 (CASCADE로 인해 문제 삭제 시 자동 삭제되지만 명시적으로)
        let delete_options_url = format!("{}/rest/v1/question_options", SUPABASE_CONFIG.url);
        let _ = Request::delete(&delete_options_url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send()
            .await;

        if !questions.is_empty() {
            let url = format!("{}/rest/v1/questions", SUPABASE_CONFIG.url);

            let records: Vec<serde_json::Value> = questions
                .values()
                .map(|q| {
                    json!({
                        "id": q.id,
                        "certificate_id": q.certificate_id,
                        "content": q.content,
                        "explanation": q.explanation,
                        "attempt_count": q.attempt_count,
                        "correct_count": q.correct_count,
                        "created_at": q.created_at.to_rfc3339(),
                        "last_attempt": q.last_attempt.map(|dt| dt.to_rfc3339()),
                    })
                })
                .collect();

            // UPSERT 사용
            let response = Request::post(&url)
                .header("apikey", SUPABASE_CONFIG.anon_key)
                .header("Authorization", &auth_header)
                .header("Content-Type", "application/json")
                .header("Prefer", "resolution=merge-duplicates") // UPSERT 설정
                .json(&records)
                .map_err(|e| format!("요청 생성 실패: {}", e))?
                .send()
                .await
                .map_err(|e| format!("요청 실패: {}", e))?;

            if !response.ok() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(format!("문제 동기화 실패: {}", error_text));
            }

            // 보기(options)도 동기화
            for question in questions.values() {
                self.sync_options_for_question(&question.id, &question.options)
                    .await?;
            }
        }

        Ok(())
    }

    // 보기 동기화
    async fn sync_options_for_question(
        &self,
        question_id: &str,
        options: &[crate::models::QuestionOption],
    ) -> Result<(), String> {
        let auth_header = self
            .get_auth_header().await?;
            //.ok_or("로그인이 필요합니다".to_string())?;

        let url = format!("{}/rest/v1/question_options", SUPABASE_CONFIG.url);

        let records: Vec<serde_json::Value> = options
            .iter()
            .enumerate()
            .map(|(idx, opt)| {
                json!({
                    "id": opt.id,
                    "question_id": question_id,
                    "content": opt.content,
                    "is_correct": opt.is_correct,
                    "explanation": opt.explanation,
                    "display_order": idx,
                })
            })
            .collect();

        let response = Request::post(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates") // UPSERT 설정
            .json(&records)
            .map_err(|e| format!("요청 생성 실패: {}", e))?
            .send()
            .await
            .map_err(|e| format!("요청 실패: {}", e))?;

        if !response.ok() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("보기 동기화 실패: {}", error_text));
        }

        Ok(())
    }

    // 문제 클라우드에서 가져오기
    pub async fn sync_questions_from_cloud(
        &self,
    ) -> Result<std::collections::HashMap<String, crate::models::Question>, String> {
        let auth_header = self
            .get_auth_header().await?;
            //.ok_or("로그인이 필요합니다".to_string())?;

        let url = format!("{}/rest/v1/questions", SUPABASE_CONFIG.url);

        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send()
            .await
            .map_err(|e| format!("요청 실패: {}", e))?;

        if response.ok() {
            let records: Vec<serde_json::Value> = response
                .json()
                .await
                .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

            let mut questions = std::collections::HashMap::new();

            for record in records {
                // 보기들 가져오기
                let options = self
                    .get_options_for_question(record["id"].as_str().unwrap_or_default())
                    .await?;

                let question = crate::models::Question {
                    id: record["id"].as_str().unwrap_or_default().to_string(),
                    certificate_id: record["certificate_id"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    content: record["content"].as_str().unwrap_or_default().to_string(),
                    options,
                    explanation: record["explanation"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    created_at: chrono::DateTime::parse_from_rfc3339(
                        record["created_at"]
                            .as_str()
                            .unwrap_or("2024-01-01T00:00:00Z"),
                    )
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                    last_attempt: record["last_attempt"]
                        .as_str()
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc)),
                    attempt_count: record["attempt_count"].as_u64().unwrap_or(0) as u32,
                    correct_count: record["correct_count"].as_u64().unwrap_or(0) as u32,
                };

                questions.insert(question.id.clone(), question);
            }

            Ok(questions)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("문제 불러오기 실패: {}", error_text))
        }
    }

    // 특정 문제의 보기들 가져오기
    async fn get_options_for_question(
        &self,
        question_id: &str,
    ) -> Result<Vec<crate::models::QuestionOption>, String> {
        let auth_header = self
            .get_auth_header().await?;
            //.ok_or("로그인이 필요합니다".to_string())?;

        let url = format!(
            "{}/rest/v1/question_options?question_id=eq.{}&order=display_order",
            SUPABASE_CONFIG.url, question_id
        );

        let response = Request::get(&url)
            .header("apikey", SUPABASE_CONFIG.anon_key)
            .header("Authorization", &auth_header)
            .send()
            .await
            .map_err(|e| format!("요청 실패: {}", e))?;

        if response.ok() {
            let records: Vec<serde_json::Value> = response
                .json()
                .await
                .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

            let options = records
                .into_iter()
                .map(|record| crate::models::QuestionOption {
                    id: record["id"].as_str().unwrap_or_default().to_string(),
                    content: record["content"].as_str().unwrap_or_default().to_string(),
                    is_correct: record["is_correct"].as_bool().unwrap_or(false),
                    explanation: record["explanation"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                })
                .collect();

            Ok(options)
        } else {
            Ok(Vec::new())
        }
    }
}
