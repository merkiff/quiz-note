use crate::services::SupabaseClient;
use crate::storage::{load_from_storage, save_to_storage, CERTIFICATES_KEY, QUESTIONS_KEY};
use std::collections::HashMap;

pub struct SyncService;

impl SyncService {
    // 클라우드로 업로드
    pub async fn push_to_cloud() -> Result<(), String> {
        let client = SupabaseClient::new(); // 수정됨

        // LocalStorage에서 자격증 가져오기
        let certificates: HashMap<String, crate::models::Certificate> =
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();

        // Supabase에 동기화
        client.sync_certificates_to_cloud(&certificates).await?;

        // 문제도 동기화
        let questions: HashMap<String, crate::models::Question> =
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();

        client.sync_questions_to_cloud(&questions).await?;

        Ok(())
    }

    // 클라우드에서 다운로드
    pub async fn pull_from_cloud() -> Result<(), String> {
        let client = SupabaseClient::new(); // 수정됨

        // Supabase에서 자격증 가져오기
        let certificates = client.sync_certificates_from_cloud().await?;

        // LocalStorage에 저장
        save_to_storage(CERTIFICATES_KEY, &certificates)?;

        // Supabase에서 문제 가져오기
        let questions = client.sync_questions_from_cloud().await?;
        save_to_storage(QUESTIONS_KEY, &questions)?;
        Ok(())
    }

    // 양방향 동기화
    pub async fn sync() -> Result<(), String> {
        Self::push_to_cloud().await?;
        Self::pull_from_cloud().await?;
        Ok(())
    }
}
