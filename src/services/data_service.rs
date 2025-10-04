// src/services/data_service.rs (새 파일)

use crate::models::{ExportedCertificate, Question};
use crate::services::{CertificateService, QuestionService};
use wasm_bindgen_futures::spawn_local;

pub struct DataService;

impl DataService {
    /// 모든 자격증과 관련 문제들을 JSON 문자열로 내보냅니다.
    pub async fn export_data() -> Result<String, String> {
        let certificates = CertificateService::get_all().await?;
        let mut export_data = Vec::new();

        for cert in certificates {
            let questions = QuestionService::get_by_certificate(&cert.id).await?;
            export_data.push(ExportedCertificate {
                certificate: cert,
                questions,
            });
        }

        serde_json::to_string_pretty(&export_data).map_err(|e| e.to_string())
    }

    /// JSON 파일로부터 데이터를 가져와 DB에 저장합니다.
    pub async fn import_data(json_str: &str) -> Result<String, String> {
        let imported_data: Vec<ExportedCertificate> =
            serde_json::from_str(json_str).map_err(|e| e.to_string())?;

        let total_certs = imported_data.len();
        let mut created_certs = 0;
        let mut created_questions = 0;

        for exported_cert in imported_data {
            // 새 자격증 생성
            let new_cert = CertificateService::create(
                exported_cert.certificate.name,
                exported_cert.certificate.description,
            )
            .await?;
            created_certs += 1;

            // 해당 자격증에 문제들 추가
            for var in exported_cert.questions {
                 let mut question = Question::new(new_cert.id.clone(), var.content.clone());
                 question.explanation = var.explanation;
                 question.options = var.options;
                
                 // 비동기로 각 문제를 생성
                spawn_local(async move {
                    let _ = QuestionService::create(question).await;
                });
                created_questions += 1;
            }
        }
        
        Ok(format!(
            "가져오기 완료! {}개의 자격증과 {}개의 문제가 생성되었습니다. 페이지가 새로고침됩니다.",
            created_certs, created_questions
        ))
    }
}