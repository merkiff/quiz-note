use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedCertificate {
    #[serde(flatten)]
    pub certificate: Certificate,
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub certificate_id: String,
    pub content: String,
    pub explanation: String,

    // API 통신 및 내보내기/가져오기 모두에서 `options` 필드가 필요합니다.
    #[serde(rename = "question_options", default)]
    pub options: Vec<QuestionOption>,

    // 통계 정보는 내보내기 파일에 포함하지 않아도 되므로 skip_serializing을 유지합니다.
    #[serde(skip_serializing, default)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing, default)]
    pub last_attempt: Option<DateTime<Utc>>,
    #[serde(skip_serializing, default)]
    pub attempt_count: u32,
    #[serde(skip_serializing, default)]
    pub correct_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionOption {
    // ID 필드들은 DB에 저장될 때 반드시 필요하므로 직렬화(serialize) 되어야 합니다.
    pub id: String,
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

    // question_count는 DB 트리거로 관리되므로 직렬화할 필요가 없습니다.
    #[serde(skip_serializing, default)]
    pub question_count: u32,
    #[serde(skip_serializing, default)]
    pub created_at: DateTime<Utc>,
}

// impl 블록들은 그대로 유지합니다.
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
            question_id: String::new(),
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