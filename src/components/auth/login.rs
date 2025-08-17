use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::services::AuthService;

#[function_component(Login)]
pub fn login() -> Html {
    let email_ref = use_node_ref();
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success = use_state(|| false);

    let on_submit = {
        let email_ref = email_ref.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email = email_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();

            if email.trim().is_empty() {
                error.set(Some("이메일을 입력해주세요.".to_string()));
                return;
            }

            let loading = loading.clone();
            let error = error.clone();
            let success = success.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match AuthService::sign_in_with_email(&email).await {
                    Ok(_) => {
                        success.set(true);
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        success.set(false);
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h1 class="text-center text-3xl font-extrabold text-gray-900">
                        {"QuizNote"}
                    </h1>
                    <h2 class="mt-6 text-center text-xl text-gray-600">
                        {"로그인"}
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        {"이메일로 로그인하면 모든 기기에서 데이터를 동기화할 수 있습니다"}
                    </p>
                </div>

                {if *success {
                    html! {
                        <div class="rounded-md bg-green-50 p-4">
                            <div class="flex">
                                <div class="flex-shrink-0">
                                    <svg class="h-5 w-5 text-green-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                    </svg>
                                </div>
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-green-800">
                                        {"이메일을 확인해주세요!"}
                                    </h3>
                                    <div class="mt-2 text-sm text-green-700">
                                        <p>
                                            {"로그인 링크를 이메일로 보냈습니다. 이메일의 링크를 클릭하면 로그인됩니다."}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <form class="mt-8 space-y-6" onsubmit={on_submit}>
                            <div>
                                <label for="email" class="block text-sm font-medium text-gray-700">
                                    {"이메일"}
                                </label>
                                <div class="mt-1">
                                    <input
                                        ref={email_ref}
                                        id="email"
                                        name="email"
                                        type="email"
                                        autocomplete="email"
                                        required=true
                                        class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                                        placeholder="your@email.com"
                                        disabled={*loading}
                                    />
                                </div>
                            </div>

                            {if let Some(err) = &*error {
                                html! {
                                    <div class="rounded-md bg-red-50 p-4">
                                        <div class="text-sm text-red-800">
                                            {err}
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}

                            <div>
                                <button
                                    type="submit"
                                    disabled={*loading}
                                    class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {if *loading {
                                        "로그인 링크 전송 중..."
                                    } else {
                                        "이메일로 로그인"
                                    }}
                                </button>
                            </div>

                            <div class="text-sm text-center text-gray-600">
                                {"비밀번호 없이 이메일로 전송된 링크를 클릭하여 로그인합니다"}
                            </div>
                        </form>
                    }
                }}
            </div>
        </div>
    }
}