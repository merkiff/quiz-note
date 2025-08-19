use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::Question;
use crate::services::QuestionService;

#[derive(Properties, PartialEq)]
pub struct QuestionListProps {
    pub certificate_id: String,
}

#[function_component(QuestionList)]
pub fn question_list(props: &QuestionListProps) -> Html {
    let questions = use_state(Vec::<Question>::new);
    let error = use_state(|| None::<String>);
    let is_loading = use_state(|| true);

    {
        let questions = questions.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();
        let certificate_id = props.certificate_id.clone();
        use_effect_with(certificate_id, move |certificate_id| {
            let certificate_id = certificate_id.clone();
            spawn_local(async move {
                is_loading.set(true);
                match QuestionService::get_by_certificate(&certificate_id).await {
                    Ok(quests) => questions.set(quests),
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
            spawn_local(async move {
                if let Err(e) = QuestionService::delete(&id).await {
                    error.set(Some(e));
                } else {
                    questions.set(questions.iter().filter(|q| q.id != id).cloned().collect());
                }
            });
        })
    };

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
                        <div class="space-y-4">
                            {for questions.iter().enumerate().map(|(idx, question)| {
                                let question_id = question.id.clone();
                                let on_delete = on_delete.clone();

                                html! {
                                    <div class="bg-gray-50 shadow-sm rounded-lg p-4">
                                        <div class="flex justify-between items-start">
                                            <div class="flex-1">
                                                <p class="font-medium text-gray-900">
                                                    {format!("문제 {}: {}", idx + 1, &question.content)}
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
                                            <button
                                                onclick={move |_| on_delete.emit(question_id.clone())}
                                                class="ml-4 text-red-600 hover:text-red-900 text-sm"
                                            >
                                                {"삭제"}
                                            </button>
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