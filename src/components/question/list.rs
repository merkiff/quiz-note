use yew::prelude::*;
use crate::models::Question;
use crate::services::QuestionService;

#[derive(Properties, PartialEq)]
pub struct QuestionListProps {
    pub certificate_id: String,
}

#[function_component(QuestionList)]
pub fn question_list(props: &QuestionListProps) -> Html {
    let questions = use_state(Vec::<Question>::new);

    {
        let questions = questions.clone();
        let certificate_id = props.certificate_id.clone();
        use_effect_with(certificate_id, move |certificate_id| {
            if let Ok(quests) = QuestionService::get_by_certificate(certificate_id) {
                questions.set(quests);
            }
        });
    }

    let on_delete = {
        let questions = questions.clone();
        let certificate_id = props.certificate_id.clone();
        Callback::from(move |id: String| {
            if let Ok(_) = QuestionService::delete(&id) {
                if let Ok(quests) = QuestionService::get_by_certificate(&certificate_id) {
                    questions.set(quests);
                }
            }
        })
    };

    html! {
        <div class="space-y-4">
            <h3 class="text-lg font-medium text-gray-900">{"문제 목록"}</h3>
            
            {if questions.is_empty() {
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
                                <div class="bg-white shadow rounded-lg p-4">
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
            }}
        </div>
    }
}