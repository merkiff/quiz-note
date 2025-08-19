use crate::models::Question;
use crate::services::SupabaseClient;

pub struct QuestionService;

impl QuestionService {
    pub async fn get_by_certificate(certificate_id: &str) -> Result<Vec<Question>, String> {
        let client = SupabaseClient::new();
        let mut quests = client.get_questions_by_certificate(certificate_id).await?;
        quests.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(quests)
    }

    pub async fn create(mut question: Question) -> Result<Question, String> {
        let client = SupabaseClient::new();

        // 보기에 순서(display_order) 부여
        for (index, option) in question.options.iter_mut().enumerate() {
            option.display_order = index as i32;
        }

        // 유효성 검사
        if question.options.len() < 2 {
            return Err("최소 2개의 보기가 필요합니다.".to_string());
        }
        if !question.options.iter().any(|o| o.is_correct) {
            return Err("정답을 선택해주세요.".to_string());
        }

        client.create_question(&question).await?;
        
        // question_count 업데이트는 Supabase DB 트리거로 처리하는 것이 더 효율적입니다.
        // 일단 클라이언트에서는 생략합니다.

        Ok(question)
    }
    
    pub async fn update_stats(question: &Question) -> Result<(), String> {
        let client = SupabaseClient::new();
        client.update_question_stats(question).await
    }

    pub async fn delete(id: &str) -> Result<(), String> {
        let client = SupabaseClient::new();
        client.delete_question(id).await
    }
}