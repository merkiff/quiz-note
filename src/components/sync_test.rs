use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::sync_service::SyncService;

#[function_component(SyncTest)]
pub fn sync_test() -> Html {
    let is_syncing = use_state(|| false);
    let sync_message = use_state(|| String::new());

    let on_sync = {
        let is_syncing = is_syncing.clone();
        let sync_message = sync_message.clone();
        
        Callback::from(move |_| {
            let is_syncing = is_syncing.clone();
            let sync_message = sync_message.clone();
            
            is_syncing.set(true);
            sync_message.set("동기화 중...".to_string());
            
            spawn_local(async move {
                match SyncService::push_to_cloud().await {
                    Ok(_) => sync_message.set("동기화 성공!".to_string()),
                    Err(e) => sync_message.set(format!("동기화 실패: {}", e)),
                }
                is_syncing.set(false);
            });
        })
    };

    html! {
        <div class="p-4">
            <button
                onclick={on_sync}
                disabled={*is_syncing}
                class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50"
            >
                {"클라우드 동기화 테스트"}
            </button>
            
            {if !sync_message.is_empty() {
                html! {
                    <p class="mt-2 text-sm">
                        {&*sync_message}
                    </p>
                }
            } else {
                html! {}
            }}
        </div>
    }
}