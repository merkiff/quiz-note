use chrono::Utc;

use crate::models::{Question, QuestionOption};
use crate::storage::{load_from_storage, save_to_storage, QUESTIONS_KEY, CERTIFICATES_KEY};
use crate::services::CertificateService;
use std::collections::HashMap;

pub struct QuestionService;

impl QuestionService {
    pub fn get_all() -> Result<Vec<Question>, String> {
        let questions: HashMap<String, Question> = 
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();
        
        let mut quests: Vec<Question> = questions.into_values().collect();
        quests.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(quests)
    }

    pub fn get_by_certificate(certificate_id: &str) -> Result<Vec<Question>, String> {
        let questions: HashMap<String, Question> = 
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();
        
        let mut quests: Vec<Question> = questions
            .into_values()
            .filter(|q| q.certificate_id == certificate_id)
            .collect();
        quests.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(quests)
    }

    pub fn get_by_id(id: &str) -> Result<Question, String> {
        let questions: HashMap<String, Question> = 
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();
        
        questions.get(id)
            .cloned()
            .ok_or_else(|| "문제를 찾을 수 없습니다.".to_string())
    }

    pub fn create(question: Question) -> Result<Question, String> {
        // 최소 2개의 보기 필요
        if question.options.len() < 2 {
            return Err("최소 2개의 보기가 필요합니다.".to_string());
        }

        // 정답이 하나 이상 있는지 확인
        let correct_count = question.options.iter().filter(|o| o.is_correct).count();
        if correct_count == 0 {
            return Err("정답을 선택해주세요.".to_string());
        }
        if correct_count > 1 {
            return Err("정답은 하나만 선택해주세요.".to_string());
        }

        let mut questions: HashMap<String, Question> = 
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();
        
        questions.insert(question.id.clone(), question.clone());
        save_to_storage(QUESTIONS_KEY, &questions)?;

        // 자격증의 문제 수 업데이트
        let mut certificates: HashMap<String, crate::models::Certificate> = 
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
        
        if let Some(cert) = certificates.get_mut(&question.certificate_id) {
            cert.question_count += 1;
            save_to_storage(CERTIFICATES_KEY, &certificates)?;
        }

        Ok(question)
    }

    pub fn update(id: &str, mut updated_question: Question) -> Result<Question, String> {
        let mut questions: HashMap<String, Question> = 
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();
        
        if !questions.contains_key(id) {
            return Err("문제를 찾을 수 없습니다.".to_string());
        }

        // ID는 변경하지 않음
        updated_question.id = id.to_string();
        
        // 최근 시도 시간 업데이트
        updated_question.last_attempt = Some(Utc::now());
        
        questions.insert(id.to_string(), updated_question.clone());
        save_to_storage(QUESTIONS_KEY, &questions)?;
        
        Ok(updated_question)
    }

    pub fn delete(id: &str) -> Result<(), String> {
        let mut questions: HashMap<String, Question> = 
            load_from_storage(QUESTIONS_KEY).unwrap_or_default();
        
        if let Some(question) = questions.remove(id) {
            save_to_storage(QUESTIONS_KEY, &questions)?;

            // 자격증의 문제 수 업데이트
            let mut certificates: HashMap<String, crate::models::Certificate> = 
                load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
            
            if let Some(cert) = certificates.get_mut(&question.certificate_id) {
                cert.question_count = cert.question_count.saturating_sub(1);
                save_to_storage(CERTIFICATES_KEY, &certificates)?;
            }
        }
        
        Ok(())
    }
}