use once_cell::sync::Lazy;

pub struct SupabaseConfig {
    pub url: &'static str,
    pub anon_key: &'static str,
}

// 실제 값으로 변경해주세요!
pub static SUPABASE_CONFIG: Lazy<SupabaseConfig> = Lazy::new(|| SupabaseConfig {
    url: "https://xmkblxxpnioyacnlomcn.supabase.co",  // 여기에 실제 URL
    anon_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Inhta2JseHhwbmlveWFjbmxvbWNuIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTU0MDI3ODksImV4cCI6MjA3MDk3ODc4OX0.a36nuDqUsz05LMTIY7VpvWFfFggaoegjVs_8lwBAVzs",          // 여기에 실제 anon key
});