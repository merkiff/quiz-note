use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub certificate_id: String,
    pub content: String,
    pub options: Vec<QuestionOption>,  // Option을 QuestionOption으로 변경
    pub explanation: String,
    pub created_at: DateTime<Utc>,
    pub last_attempt: Option<DateTime<Utc>>,  // 여기는 표준 Option 사용
    pub attempt_count: u32,
    pub correct_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionOption {  // Option을 QuestionOption으로 변경
    pub id: String,
    pub content: String,
    pub is_correct: bool,
    pub explanation: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub question_count: u32,
    pub created_at: DateTime<Utc>,
}

impl Question {
    pub fn new(certificate_id: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            certificate_id,
            content,
            options: Vec::new(),
            explanation: String::new(),
            created_at: Utc::now(),
            last_attempt: None,
            attempt_count: 0,
            correct_count: 0,
        }
    }
}

impl QuestionOption {  // Option을 QuestionOption으로 변경
    pub fn new(content: String, is_correct: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            is_correct,
            explanation: String::new(),
        }
    }
}

impl Certificate {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            question_count: 0,
            created_at: Utc::now(),
        }
    }
}