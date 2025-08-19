use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::Certificate;
use crate::routes::Route;
use crate::services::CertificateService;
use crate::components::CertificateForm;

#[function_component(CertificateList)]
pub fn certificate_list() -> Html {
    let certificates = use_state(Vec::<Certificate>::new);
    let show_form = use_state(|| false);
    let error = use_state(|| None::<String>);
    let is_loading = use_state(|| true);

    // 데이터를 비동기로 불러오기
    {
        let certificates = certificates.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                is_loading.set(true);
                match CertificateService::get_all().await {
                    Ok(certs) => certificates.set(certs),
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
            || ()
        });
    }

    let on_delete = {
        let certificates = certificates.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            // Callback 내부에서 사용할 상태 핸들 복제
            let certificates = certificates.clone();
            let error = error.clone();
            spawn_local(async move {
                if let Err(e) = CertificateService::delete(&id).await {
                    error.set(Some(e));
                } else {
                    certificates.set(certificates.iter().filter(|c| c.id != id).cloned().collect());
                }
            });
        })
    };

    let on_form_submit = {
        let show_form = show_form.clone();
        let certificates = certificates.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |_| {
            show_form.set(false);
            // Callback 내부에서 사용할 상태 핸들 복제 (오류 수정 지점)
            let certificates = certificates.clone();
            let error = error.clone();
            let is_loading = is_loading.clone();
            spawn_local(async move {
                is_loading.set(true);
                match CertificateService::get_all().await {
                    Ok(certs) => {
                        certificates.set(certs);
                        error.set(None); // 성공 시 에러 메시지 초기화
                    },
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
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
                    { if *show_form { "취소" } else { "새 자격증 추가" } }
                </button>
            </div>

            if *show_form {
                <div class="mb-6">
                    <CertificateForm
                        on_submit={on_form_submit}
                        on_cancel={let show_form = show_form.clone(); move |_| show_form.set(false)}
                    />
                </div>
            }

            {
                if *is_loading {
                    html! { <div class="text-center py-12 text-gray-500">{"목록을 불러오는 중..."}</div> }
                } else if let Some(err) = &*error {
                    html! { <div class="text-red-600 text-center py-12">{format!("오류가 발생했습니다: {}", err)}</div> }
                } else if certificates.is_empty() {
                    html! {
                        <div class="text-center py-12">
                            <p class="text-gray-500">{"등록된 자격증이 없습니다."}</p>
                            <p class="text-gray-500 mt-2">{"새 자격증을 추가해보세요!"}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                            {for certificates.iter().map(|cert| {
                                let cert_id_for_delete = cert.id.clone();
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
                                                    onclick={move |_| on_delete.emit(cert_id_for_delete.clone())}
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
                    }
                }
            }
        </div>
    }
}