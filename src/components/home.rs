use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use crate::services::{AuthService, sync_service::SyncService};
use wasm_bindgen_futures::spawn_local;
use gloo::storage::{LocalStorage, Storage};

#[function_component(Home)]
pub fn home() -> Html {
    let is_syncing = use_state(|| false);
    let sync_status = use_state(|| String::new());
    let last_sync = use_state(|| {
        gloo::storage::LocalStorage::get::<String>("last_sync_time").unwrap_or_default()
    });

    let on_sync = {
        let is_syncing = is_syncing.clone();
        let sync_status = sync_status.clone();
        let last_sync = last_sync.clone();
        
        Callback::from(move |_: MouseEvent| {
            let is_syncing = is_syncing.clone();
            let sync_status = sync_status.clone();
            let last_sync = last_sync.clone();
            
            spawn_local(async move {
                is_syncing.set(true);
                sync_status.set("동기화 중...".to_string());
                
                match SyncService::sync().await {
                    Ok(_) => {
                        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string();
                        let _ = gloo::storage::LocalStorage::set("last_sync_time", &now);
                        last_sync.set(now);
                        sync_status.set("동기화 완료!".to_string());
                    }
                    Err(e) => {
                        sync_status.set(format!("동기화 실패: {}", e));
                    }
                }
                
                is_syncing.set(false);
            });
        })
    };

    // 로그인 시 자동 동기화
    {
    let is_syncing = is_syncing.clone();
    let sync_status = sync_status.clone();
    let last_sync = last_sync.clone();
    
    use_effect_with((), move |_| {
        spawn_local(async move {
            is_syncing.set(true);
            sync_status.set("동기화 중...".to_string());
            
            match SyncService::sync().await {
                Ok(_) => {
                    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string();
                    let _ = LocalStorage::set("last_sync_time", &now);
                    last_sync.set(now);
                    sync_status.set("동기화 완료!".to_string());
                }
                Err(e) => {
                    sync_status.set(format!("동기화 실패: {}", e));
                }
            }
            
            is_syncing.set(false);
        });
    });
}

    let user = AuthService::get_current_user();

    html! {
        <div class="px-4 py-5 sm:p-6">
            <div class="text-center mb-8">
                <h2 class="text-3xl font-bold text-gray-900">
                    {"QuizNote에 오신 것을 환영합니다"}
                </h2>
                {if let Some(user) = user {
                    html! {
                        <p class="mt-2 text-lg text-gray-600">
                            {format!("{} 님", user.email)}
                        </p>
                    }
                } else {
                    html! {}
                }}
            </div>

            // 동기화 상태 카드
            <div class="bg-white shadow rounded-lg p-6 mb-8">
                <div class="flex items-center justify-between">
                    <div>
                        <h3 class="text-lg font-medium text-gray-900">{"클라우드 동기화"}</h3>
                        <p class="text-sm text-gray-500 mt-1">
                            {if !last_sync.is_empty() {
                                format!("마지막 동기화: {}", *last_sync)
                            } else {
                                "아직 동기화하지 않았습니다".to_string()
                            }}
                        </p>
                        {if !sync_status.is_empty() {
                            html! {
                                <p class="text-sm mt-2">
                                    {&*sync_status}
                                </p>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                    <button
                        onclick={on_sync}
                        disabled={*is_syncing}
                        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {if *is_syncing {
                            "동기화 중..."
                        } else {
                            "지금 동기화"
                        }}
                    </button>
                </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <Link<Route> to={Route::Certificates}>
                    <div class="bg-white overflow-hidden shadow rounded-lg hover:shadow-lg transition cursor-pointer">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <svg class="h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                    </svg>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-lg font-medium text-gray-900">
                                            {"자격증 관리"}
                                        </dt>
                                        <dd class="text-gray-500 text-sm">
                                            {"자격증을 추가하고 관리하세요"}
                                        </dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                    </div>
                </Link<Route>>
                
                <Link<Route> to={Route::NewQuestion}>
                    <div class="bg-white overflow-hidden shadow rounded-lg hover:shadow-lg transition cursor-pointer">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <svg class="h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                                    </svg>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-lg font-medium text-gray-900">
                                            {"새 문제 작성"}
                                        </dt>
                                        <dd class="text-gray-500 text-sm">
                                            {"객관식 문제를 추가하세요"}
                                        </dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                    </div>
                </Link<Route>>
            </div>
        </div>
    }
}