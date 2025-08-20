use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub certificate_id: String,
    pub content: String,
    #[serde(rename = "question_options", skip_serializing, default)]
    pub options: Vec<QuestionOption>,
    pub explanation: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_attempt: Option<DateTime<Utc>>,
    pub attempt_count: u32,
    pub correct_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: String,
    // 이 필드를 추가합니다.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub question_id: String,
    pub content: String,
    pub is_correct: bool,
    pub explanation: String,
    #[serde(default)]
    pub display_order: i32,
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

impl QuestionOption {
    pub fn new(content: String, is_correct: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            question_id: String::new(), // 필드를 초기화합니다.
            content,
            is_correct,
            explanation: String::new(),
            display_order: 0,
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

// 이 함수는 더 이상 사용되지 않으므로 삭제하거나 주석 처리합니다.
// fn is_zero(num: &i32) -> bool {
//     *num == 0
// }

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == T::default()
}