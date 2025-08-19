use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use crate::services::AuthService;

#[function_component(Home)]
pub fn home() -> Html {
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

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <Link<Route> to={Route::Certificates} classes="block bg-white overflow-hidden shadow rounded-lg hover:shadow-lg transition cursor-pointer">
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
                </Link<Route>>

                <Link<Route> to={Route::NewQuestion} classes="block bg-white overflow-hidden shadow rounded-lg hover:shadow-lg transition cursor-pointer">
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
                </Link<Route>>
            </div>
        </div>
    }
}