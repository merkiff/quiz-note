use yew::prelude::*;
use yew_router::prelude::*;
use crate::models::Certificate;
use crate::routes::Route;
use crate::services::CertificateService;
use crate::components::question::QuestionList;

#[derive(Properties, PartialEq)]
pub struct CertificateDetailProps {
    pub id: String,
}

#[function_component(CertificateDetail)]
pub fn certificate_detail(props: &CertificateDetailProps) -> Html {
    let certificate = use_state(|| None::<Certificate>);
    let navigator = use_navigator().unwrap();

    {
        let certificate = certificate.clone();
        let id = props.id.clone();
        use_effect_with(id, move |id| {
            match CertificateService::get_by_id(id) {
                Ok(cert) => certificate.set(Some(cert)),
                Err(_) => certificate.set(None),
            }
        });
    }

    let on_delete = {
        let id = props.id.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Ok(_) = CertificateService::delete(&id) {
                navigator.push(&Route::Certificates);
            }
        })
    };

    match &*certificate {
        Some(cert) => html! {
            <div class="px-4 py-5 sm:p-6">
                <div class="bg-white shadow overflow-hidden sm:rounded-lg mb-6">
                    <div class="px-4 py-5 sm:px-6">
                        <h3 class="text-lg leading-6 font-medium text-gray-900">
                            {&cert.name}
                        </h3>
                        <p class="mt-1 max-w-2xl text-sm text-gray-500">
                            {&cert.description}
                        </p>
                    </div>
                    <div class="border-t border-gray-200">
                        <dl>
                            <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">
                                    {"문제 수"}
                                </dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {cert.question_count}
                                </dd>
                            </div>
                            <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">
                                    {"생성일"}
                                </dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                    {cert.created_at.format("%Y-%m-%d %H:%M").to_string()}
                                </dd>
                            </div>
                        </dl>
                    </div>
                    <div class="px-4 py-3 bg-gray-50 text-right sm:px-6">
                        <Link<Route> to={Route::Quiz { certificate_id: cert.id.clone() }}>
                            <button class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 mr-3">
                                {"문제 풀기"}
                            </button>
                        </Link<Route>>
                        <Link<Route> to={Route::NewQuestion}>
                            <button class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 mr-3">
                                {"문제 추가"}
                            </button>
                        </Link<Route>>
                        <button 
                            onclick={on_delete}
                            class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                        >
                            {"삭제"}
                        </button>
                    </div>
                </div>

                // 문제 목록 추가
                <div class="bg-white shadow sm:rounded-lg p-6">
                    <QuestionList certificate_id={cert.id.clone()} />
                </div>
            </div>
        },
        None => html! {
            <div class="text-center py-12">
                <p class="text-gray-500">{"자격증을 찾을 수 없습니다."}</p>
                <Link<Route> to={Route::Certificates}>
                    <button class="mt-4 text-blue-600 hover:text-blue-900">
                        {"목록으로 돌아가기"}
                    </button>
                </Link<Route>>
            </div>
        }
    }
}