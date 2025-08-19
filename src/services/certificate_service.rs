use crate::models::Certificate;
use crate::services::SupabaseClient;

pub struct CertificateService;

impl CertificateService {
    pub async fn get_all() -> Result<Vec<Certificate>, String> {
        let client = SupabaseClient::new();
        let mut certs = client.get_all_certificates().await?;
        // 최신순으로 정렬
        certs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(certs)
    }

    pub async fn create(name: String, description: String) -> Result<Certificate, String> {
        let client = SupabaseClient::new();
        let certificate = Certificate::new(name, description);
        client.create_certificate(&certificate).await?;
        Ok(certificate)
    }

    pub async fn delete(id: &str) -> Result<(), String> {
        let client = SupabaseClient::new();
        client.delete_certificate(id).await
    }
    
    // get_by_id, update는 현재 앱에서 직접 사용하지 않으므로 단순화를 위해 삭제합니다.
    // 필요하다면 위와 같은 방식으로 추가할 수 있습니다.
}