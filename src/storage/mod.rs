use gloo::storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

pub const CERTIFICATES_KEY: &str = "quiz_note_certificates";
pub const QUESTIONS_KEY: &str = "quiz_note_questions";

pub fn save_to_storage<T: Serialize>(key: &str, data: &T) -> Result<(), String> {
    LocalStorage::set(key, data).map_err(|e| e.to_string())
}

pub fn load_from_storage<T: DeserializeOwned>(key: &str) -> Result<T, String> {
    LocalStorage::get(key).map_err(|e| e.to_string())
}

pub fn delete_from_storage(key: &str) -> Result<(), String> {
    LocalStorage::delete(key);
    Ok(())
}