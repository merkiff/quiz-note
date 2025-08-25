use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::Question;
use crate::routes::Route;
use crate::services::QuestionService;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct QuestionListProps {
    pub certificate_id: String,
}

const QUESTIONS_PER_PAGE: usize = 10;

#[function_component(QuestionList)]
pub fn question_list(props: &QuestionListProps) -> Html {
    let questions = use_state(Vec::<Question>::new);
    let error = use_state(|| None::<String>);
    let is_loading = use_state(|| true);
    let current_page = use_state(|| 1);

    {
        let questions = questions.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();
        let certificate_id = props.certificate_id.clone();
        use_effect_with(certificate_id, move |certificate_id| {
            let certificate_id = certificate_id.clone();
            spawn_local(async move {
                is_loading.set(true);
                error.set(None);
                match QuestionService::get_by_certificate(&certificate_id).await {
                    Ok(mut quests) => {
                        quests.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        questions.set(quests);
                    },
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
            || ()
        });
    }

    let on_delete = {
        let questions = questions.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let questions = questions.clone();
            let error = error.clone();
            if window().unwrap().confirm_with_message("정말로 이 문제를 삭제하시겠습니까?").unwrap_or(false) {
                spawn_local(async move {
                    if let Err(e) = QuestionService::delete(&id).await {
                        error.set(Some(e));
                    } else {
                        questions.set(questions.iter().filter(|q| q.id != id).cloned().collect());
                    }
                });
            }
        })
    };
    
    let total_pages = (questions.len() as f64 / QUESTIONS_PER_PAGE as f64).ceil() as usize;

    let start_index = (*current_page - 1) * QUESTIONS_PER_PAGE;
    let current_questions = questions.iter().skip(start_index).take(QUESTIONS_PER_PAGE);

    html! {
        <div class="space-y-4">
            <h3 class="text-lg font-medium text-gray-900">{"문제 목록"}</h3>

            {
                if *is_loading {
                    html!{ <div class="text-center py-8 text-gray-500">{"문제를 불러오는 중..."}</div> }
                } else if let Some(err) = &*error {
                    html! { <div class="text-red-600 text-center py-8">{format!("오류: {}", err)}</div> }
                } else if questions.is_empty() {
                    html! {
                        <div class="text-center py-8 text-gray-500">
                            {"등록된 문제가 없습니다."}
                        </div>
                    }
                } else {
                    html! {
                        <>
                            <div class="space-y-4">
                                {for current_questions.enumerate().map(|(idx_on_page, question)| {
                                    let question_id = question.id.clone();
                                    let on_delete = on_delete.clone();

                                    html! {
                                        <div key={question_id.clone()} class="bg-gray-50 shadow-sm rounded-lg p-4">
                                            <div class="flex justify-between items-start">
                                                <div class="flex-1">
                                                    <p class="font-medium text-gray-900">
                                                        {format!("문제 {}: {}", start_index + idx_on_page + 1, &question.content)}
                                                    </p>
                                                    <div class="mt-2 space-y-1">
                                                        {for question.options.iter().enumerate().map(|(opt_idx, option)| {
                                                            html! {
                                                                <div class={if option.is_correct { "text-green-600 font-medium" } else { "text-gray-600" }}>
                                                                    {format!("{}. {}", opt_idx + 1, &option.content)}
                                                                    {if option.is_correct { " ✓" } else { "" }}
                                                                </div>
                                                            }
                                                        })}
                                                    </div>
                                                    <div class="mt-2 text-sm text-gray-500">
                                                        {format!("시도: {}회, 정답: {}회", question.attempt_count, question.correct_count)}
                                                    </div>
                                                </div>
                                                <div class="flex space-x-4">
                                                    <Link<Route> to={Route::EditQuestion { id: question.id.clone() }}>
                                                        <button class="text-blue-600 hover:text-blue-900 text-sm">
                                                            {"수정"}
                                                        </button>
                                                    </Link<Route>>
                                                    <button
                                                        onclick={move |_| on_delete.emit(question_id.clone())}
                                                        class="text-red-600 hover:text-red-900 text-sm"
                                                    >
                                                        {"삭제"}
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>

                            if total_pages > 1 {
                                <div class="flex justify-center mt-6 space-x-2">
                                    <button
                                        onclick={{
                                            let current_page = current_page.clone();
                                            move |_| current_page.set((*current_page - 1).max(1))
                                        }}
                                        disabled={*current_page == 1}
                                        class="px-3 py-1 rounded-md border border-gray-300 bg-white text-gray-600 hover:bg-gray-100 disabled:opacity-50"
                                    >
                                        {"이전"}
                                    </button>
                                    
                                    { for (1..=total_pages).map(|page| {
                                        let current_page = current_page.clone();
                                        let page_onclick = {
                                            let current_page = current_page.clone();
                                            Callback::from(move |_| current_page.set(page))
                                        };
                                        html! {
                                            <button
                                                key={page}
                                                onclick={page_onclick}
                                                class={if *current_page == page {
                                                    "px-3 py-1 rounded-md border border-blue-500 bg-blue-500 text-white"
                                                } else {
                                                    "px-3 py-1 rounded-md border border-gray-300 bg-white text-gray-600 hover:bg-gray-100"
                                                }}
                                            >
                                                {page}
                                            </button>
                                        }
                                    }) }

                                    <button
                                        onclick={{
                                            let current_page = current_page.clone();
                                            move |_| current_page.set((*current_page + 1).min(total_pages))
                                        }}
                                        disabled={*current_page == total_pages}
                                        class="px-3 py-1 rounded-md border border-gray-300 bg-white text-gray-600 hover:bg-gray-100 disabled:opacity-50"
                                    >
                                        {"다음"}
                                    </button>
                                </div>
                            }
                        </>
                    }
                }
            }
        </div>
    }
}