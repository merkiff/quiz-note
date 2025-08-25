use crate::models::Certificate;
use crate::services::SupabaseClient;

pub struct CertificateService;

impl CertificateService {
    pub async fn get_all() -> Result<Vec<Certificate>, String> {
        let client = SupabaseClient::new();
        let mut certs = client.get_all_certificates().await?;
        certs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(certs)
    }

    pub async fn get_by_id(id: &str) -> Result<Certificate, String> {
        let client = SupabaseClient::new();
        client.get_certificate_by_id(id).await
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
}