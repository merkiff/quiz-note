use crate::models::Certificate;
use crate::storage::{load_from_storage, save_to_storage, CERTIFICATES_KEY};
use std::collections::HashMap;

pub struct CertificateService;

impl CertificateService {
    pub fn get_all() -> Result<Vec<Certificate>, String> {
        let certificates: HashMap<String, Certificate> = 
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
        
        let mut certs: Vec<Certificate> = certificates.into_values().collect();
        certs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(certs)
    }

    pub fn get_by_id(id: &str) -> Result<Certificate, String> {
        let certificates: HashMap<String, Certificate> = 
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
        
        certificates.get(id)
            .cloned()
            .ok_or_else(|| "자격증을 찾을 수 없습니다.".to_string())
    }

    pub fn create(name: String, description: String) -> Result<Certificate, String> {
        let mut certificates: HashMap<String, Certificate> = 
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
        
        let certificate = Certificate::new(name, description);
        certificates.insert(certificate.id.clone(), certificate.clone());
        
        save_to_storage(CERTIFICATES_KEY, &certificates)?;
        Ok(certificate)
    }

    pub fn update(id: &str, name: String, description: String) -> Result<Certificate, String> {
        let mut certificates: HashMap<String, Certificate> = 
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
        
        let certificate = certificates.get_mut(id)
            .ok_or_else(|| "자격증을 찾을 수 없습니다.".to_string())?;
        
        certificate.name = name;
        certificate.description = description;
        let updated = certificate.clone();
        
        save_to_storage(CERTIFICATES_KEY, &certificates)?;
        Ok(updated)
    }

    pub fn delete(id: &str) -> Result<(), String> {
        let mut certificates: HashMap<String, Certificate> = 
            load_from_storage(CERTIFICATES_KEY).unwrap_or_default();
        
        certificates.remove(id);
        save_to_storage(CERTIFICATES_KEY, &certificates)?;
        Ok(())
    }
}