use yew::prelude::*;
use yew_router::prelude::*;
use crate::models::Certificate;
use crate::routes::Route;
use crate::services::CertificateService;
use crate::components::CertificateForm;  // 이 줄을 추가

#[function_component(CertificateList)]
pub fn certificate_list() -> Html {
    let certificates = use_state(Vec::<Certificate>::new);
    let show_form = use_state(|| false);
    
    {
        let certificates = certificates.clone();
        use_effect_with((), move |_| {
            if let Ok(certs) = CertificateService::get_all() {
                certificates.set(certs);
            }
        });
    }

    let on_delete = {
        let certificates = certificates.clone();
        Callback::from(move |id: String| {
            if let Ok(_) = CertificateService::delete(&id) {
                if let Ok(certs) = CertificateService::get_all() {
                    certificates.set(certs);
                }
            }
        })
    };

    html! {
        <div class="px-4 py-5 sm:p-6">
            <div class="flex justify-between items-center mb-6">
                <h2 class="text-2xl font-bold text-gray-900">{"자격증 관리"}</h2>
                <button 
                    onclick={let show_form = show_form.clone(); move |_| show_form.set(!*show_form)}
                    class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition"
                >
                    {"새 자격증 추가"}
                </button>
            </div>

            if *show_form {
                <div class="mb-6">
                    <CertificateForm 
                        on_submit={
                            let certificates = certificates.clone();
                            let show_form = show_form.clone();
                            move |_| {
                                if let Ok(certs) = CertificateService::get_all() {
                                    certificates.set(certs);
                                }
                                show_form.set(false);
                            }
                        }
                        on_cancel={let show_form = show_form.clone(); move |_| show_form.set(false)}
                    />
                </div>
            }

            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                {for certificates.iter().map(|cert| {
                    let cert_id = cert.id.clone();
                    let on_delete = on_delete.clone();
                    
                    html! {
                        <div class="bg-white overflow-hidden shadow rounded-lg">
                            <div class="px-4 py-5 sm:p-6">
                                <h3 class="text-lg font-medium text-gray-900">
                                    {&cert.name}
                                </h3>
                                <p class="mt-1 text-sm text-gray-600">
                                    {&cert.description}
                                </p>
                                <div class="mt-3 text-sm text-gray-500">
                                    {format!("문제 수: {}", cert.question_count)}
                                </div>
                                <div class="mt-4 flex space-x-2">
                                    <Link<Route> to={Route::CertificateDetail { id: cert.id.clone() }}>
                                        <button class="text-blue-600 hover:text-blue-900 text-sm font-medium">
                                            {"상세보기"}
                                        </button>
                                    </Link<Route>>
                                    <button 
                                        onclick={move |_| on_delete.emit(cert_id.clone())}
                                        class="text-red-600 hover:text-red-900 text-sm font-medium"
                                    >
                                        {"삭제"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>

            if certificates.is_empty() && !*show_form {
                <div class="text-center py-12">
                    <p class="text-gray-500">{"등록된 자격증이 없습니다."}</p>
                    <p class="text-gray-500 mt-2">{"새 자격증을 추가해보세요!"}</p>
                </div>
            }
        </div>
    }
}