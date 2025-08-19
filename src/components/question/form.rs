use crate::models::{Certificate, Question, QuestionOption};
use crate::routes::Route;
use crate::services::{CertificateService, QuestionService};
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[function_component(QuestionForm)]
pub fn question_form() -> Html {
    let navigator = use_navigator().unwrap();
    let certificates = use_state(Vec::<Certificate>::new);
    let selected_certificate = use_state(String::new);
    let question_content = use_state(String::new);
    let explanation = use_state(String::new);
    let options = use_state(|| {
        vec![
            (String::new(), false, String::new()),
            (String::new(), false, String::new()),
            (String::new(), false, String::new()),
            (String::new(), false, String::new()),
        ]
    });
    let error = use_state(|| None::<String>);
    let is_loading = use_state(|| true); // 자격증 로딩을 위해 true로 시작

    {
        let certificates = certificates.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match CertificateService::get_all().await {
                    Ok(certs) => certificates.set(certs),
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false); // 로딩 완료
            });
            || ()
        });
    }

    let on_certificate_change = {
        let selected_certificate = selected_certificate.clone();
        Callback::from(move |e: Event| {
            let value = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
            selected_certificate.set(value);
        })
    };

    let on_question_change = {
        let question_content = question_content.clone();
        Callback::from(move |e: Event| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            question_content.set(target.value());
        })
    };

    let on_explanation_change = {
        let explanation = explanation.clone();
        Callback::from(move |e: Event| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            explanation.set(target.value());
        })
    };

    let on_option_change = {
        let options = options.clone();
        Callback::from(move |(idx, value): (usize, String)| {
            let mut opts = (*options).clone();
            if let Some(opt) = opts.get_mut(idx) {
                opt.0 = value;
            }
            options.set(opts);
        })
    };

    let on_correct_change = {
        let options = options.clone();
        Callback::from(move |idx: usize| {
            let mut opts = (*options).clone();
            for (i, opt) in opts.iter_mut().enumerate() {
                opt.1 = i == idx;
            }
            options.set(opts);
        })
    };

    let on_option_explanation_change = {
        let options = options.clone();
        Callback::from(move |(idx, value): (usize, String)| {
            let mut opts = (*options).clone();
            if let Some(opt) = opts.get_mut(idx) {
                opt.2 = value;
            }
            options.set(opts);
        })
    };

    let add_option = {
        let options = options.clone();
        Callback::from(move |_| {
            let mut opts = (*options).clone();
            opts.push((String::new(), false, String::new()));
            options.set(opts);
        })
    };

    let remove_option = {
        let options = options.clone();
        Callback::from(move |idx: usize| {
            let mut opts = (*options).clone();
            if opts.len() > 2 {
                opts.remove(idx);
                options.set(opts);
            }
        })
    };

    let on_submit = {
        let navigator = navigator.clone();
        let selected_certificate = selected_certificate.clone();
        let question_content = question_content.clone();
        let explanation = explanation.clone();
        let options = options.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(None);

            if selected_certificate.is_empty() {
                error.set(Some("자격증을 선택해주세요.".to_string()));
                return;
            }
            if question_content.trim().is_empty() {
                error.set(Some("문제를 입력해주세요.".to_string()));
                return;
            }

            let mut question = Question::new((*selected_certificate).clone(), (*question_content).clone());
            question.explanation = (*explanation).clone();
            for (content, is_correct, expl) in (*options).iter() {
                if !content.trim().is_empty() {
                    let mut option = QuestionOption::new(content.clone(), *is_correct);
                    option.explanation = expl.clone();
                    question.options.push(option);
                }
            }
            
            // 유효성 검사 추가
            if question.options.iter().filter(|o| !o.content.trim().is_empty()).count() < 2 {
                 error.set(Some("최소 2개 이상의 보기를 입력해주세요.".to_string()));
                 return;
            }
             if !question.options.iter().any(|o| o.is_correct) {
                 error.set(Some("정답을 선택해주세요.".to_string()));
                 return;
            }

            let navigator = navigator.clone();
            let error = error.clone();
            let is_loading = is_loading.clone();
            spawn_local(async move {
                is_loading.set(true);
                match QuestionService::create(question).await {
                    Ok(_) => navigator.push(&Route::Certificates),
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div class="max-w-4xl mx-auto px-4 py-5 sm:p-6">
            <h2 class="text-2xl font-bold text-gray-900 mb-6">{"새 문제 작성"}</h2>

            <form onsubmit={on_submit} class="space-y-6">
                 <div>
                    <label for="certificate-select" class="block text-sm font-medium text-gray-700">{"자격증 선택"}</label>
                    <select
                        id="certificate-select"
                        class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md border"
                        onchange={on_certificate_change}
                        disabled={*is_loading}
                    >
                        <option value="" selected={selected_certificate.is_empty()}>{"자격증을 선택하세요"}</option>
                        { for certificates.iter().map(|cert| {
                            html! { <option value={cert.id.clone()} selected={*selected_certificate == cert.id}>{&cert.name}</option> }
                        })}
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        {"문제"}
                    </label>
                    <textarea
                        value={(*question_content).clone()}
                        onchange={on_question_change}
                        rows="3"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                        placeholder="문제를 입력하세요"
                        disabled={*is_loading}
                    />
                </div>

                <div>
                    <div class="flex justify-between items-center mb-2">
                        <label class="block text-sm font-medium text-gray-700">
                            {"보기"}
                        </label>
                        <button
                            type="button"
                            onclick={add_option}
                            class="text-sm text-blue-600 hover:text-blue-900"
                            disabled={*is_loading}
                        >
                            {"+ 보기 추가"}
                        </button>
                    </div>

                    <div class="space-y-4">
                        {for options.iter().enumerate().map(|(idx, (content, is_correct, expl))| {
                            let on_option_change = on_option_change.clone();
                            let on_correct_change = on_correct_change.clone();
                            let on_option_explanation_change = on_option_explanation_change.clone();
                            let remove_option = remove_option.clone();
                            let content_value = content.clone();
                            let expl_value = expl.clone();

                            html! {
                                <div key={idx.to_string()} class="mb-4 p-4 border rounded-lg bg-gray-50">
                                    <div class="flex items-start space-x-3">
                                        <input
                                            type="radio"
                                            name="correct_answer"
                                            checked={*is_correct}
                                            onchange={move |_| on_correct_change.emit(idx)}
                                            class="mt-1"
                                            disabled={*is_loading}
                                        />
                                        <div class="flex-1">
                                            <input
                                                type="text"
                                                value={content_value}
                                                onchange={move |e: Event| {
                                                    let target: HtmlInputElement = e.target_unchecked_into();
                                                    on_option_change.emit((idx, target.value()));
                                                }}
                                                placeholder={format!("보기 {}", idx + 1)}
                                                class="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                                                disabled={*is_loading}
                                            />
                                            <textarea
                                                value={expl_value}
                                                onchange={move |e: Event| {
                                                    let target: HtmlTextAreaElement = e.target_unchecked_into();
                                                    on_option_explanation_change.emit((idx, target.value()));
                                                }}
                                                placeholder="이 보기에 대한 해설 (선택사항)"
                                                rows="2"
                                                class="mt-2 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                                                disabled={*is_loading}
                                            />
                                        </div>
                                        {if options.len() > 2 {
                                            html! {
                                                <button
                                                    type="button"
                                                    onclick={move |_| remove_option.emit(idx)}
                                                    class="text-red-600 hover:text-red-900 disabled:opacity-50"
                                                    disabled={*is_loading}
                                                >
                                                    {"삭제"}
                                                </button>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        {"전체 해설 (선택사항)"}
                    </label>
                    <textarea
                        value={(*explanation).clone()}
                        onchange={on_explanation_change}
                        rows="3"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                        placeholder="문제에 대한 전체 해설을 입력하세요"
                        disabled={*is_loading}
                    />
                </div>

                {if let Some(err) = &*error {
                    html! {
                        <div class="text-red-600 text-sm">
                            {err}
                        </div>
                    }
                } else {
                    html! {}
                }}

                <div class="flex justify-end space-x-3">
                    <Link<Route> to={Route::Certificates}>
                        <button
                            type="button"
                            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                        >
                            {"취소"}
                        </button>
                    </Link<Route>>
                    <button
                        type="submit"
                        class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
                        disabled={*is_loading}
                    >
                        { if *is_loading { "저장 중..." } else { "문제 저장" } }
                    </button>
                </div>
            </form>
        </div>
    }
}