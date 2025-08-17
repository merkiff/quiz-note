mod components;
mod config;
mod models;
mod routes;
mod services;
mod storage;

use routes::{switch, Route};
use services::AuthService;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let is_checking_auth = use_state(|| true);
    let force_render = use_force_update();

    // 인증 콜백 처리
    {
        let is_checking_auth = is_checking_auth.clone();
        let force_render = force_render.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                // 항상 콜백 처리 시도
                match AuthService::handle_auth_callback().await {
                    Ok(_) => {
                        web_sys::console::log_1(&"Auth callback processed".into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Auth callback error: {}", e).into());
                    }
                }

                // URL에 토큰이 있었다면 리렌더링
                let hash = window().unwrap().location().hash().unwrap_or_default();
                if hash.contains("access_token") {
                    force_render.force_update();
                }

                is_checking_auth.set(false);
            });
        });
    }

    if *is_checking_auth {
        return html! {
            <div class="min-h-screen flex items-center justify-center">
                <p class="text-gray-500">{"로딩 중..."}</p>
            </div>
        };
    }

    let is_authenticated = AuthService::is_authenticated();
    let current_user = AuthService::get_current_user();

    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-gray-50">
                {if is_authenticated {
                    html! {
                        <>
                            <nav class="bg-white shadow-sm">
                                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                                    <div class="flex justify-between h-16">
                                        <div class="flex">
                                            <div class="flex-shrink-0 flex items-center">
                                                <Link<Route> to={Route::Home}>
                                                    <h1 class="text-xl font-bold text-gray-800">{"QuizNote"}</h1>
                                                </Link<Route>>
                                            </div>
                                            <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                                                <Link<Route> to={Route::Certificates}
                                                    classes="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                                                    {"자격증 관리"}
                                                </Link<Route>>
                                                <Link<Route> to={Route::NewQuestion}
                                                    classes="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">
                                                    {"문제 작성"}
                                                </Link<Route>>
                                            </div>
                                        </div>
                                        <div class="flex items-center">
                                            <span class="text-sm text-gray-500 mr-4">
                                                {current_user.map(|u| u.email).unwrap_or_default()}
                                            </span>
                                            <button
                                                onclick={|_| {
                                                    spawn_local(async {
                                                        let _ = AuthService::sign_out().await;
                                                    });
                                                }}
                                                class="text-sm text-gray-500 hover:text-gray-700"
                                            >
                                                {"로그아웃"}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </nav>

                            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                                <Switch<Route> render={switch} />
                            </main>
                        </>
                    }
                } else {
                    html! {
                        <Switch<Route> render={switch} />
                    }
                }}
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
