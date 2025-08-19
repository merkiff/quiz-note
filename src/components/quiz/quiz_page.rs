use crate::models::{Certificate, Question};
use crate::routes::Route;
use crate::services::{CertificateService, QuestionService};
use chrono::Utc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuizPageProps {
    pub certificate_id: String,
}

// ... (QuizState enum remains the same)
#[derive(Clone, PartialEq)]
enum QuizState {
    Loading,
    NoQuestions,
    InProgress {
        current_index: usize,
        selected_option: Option<String>,
        show_result: bool,
        is_correct: bool,
    },
    Completed {
        total_questions: usize,
        correct_answers: usize,
    },
}


#[function_component(QuizPage)]
pub fn quiz_page(props: &QuizPageProps) -> Html {
    let certificate = use_state(|| None::<Certificate>);
    let questions = use_state(Vec::<Question>::new);
    let quiz_state = use_state(|| QuizState::Loading);
    let attempt_results = use_state(Vec::<bool>::new);

    {
        let certificate = certificate.clone();
        let questions = questions.clone();
        let quiz_state = quiz_state.clone();
        let certificate_id = props.certificate_id.clone();

        use_effect_with(certificate_id.clone(), move |_| {
            spawn_local(async move {
                // 자격증 정보 로드
                if let Ok(certs) = CertificateService::get_all().await {
                    if let Some(cert) = certs.into_iter().find(|c| c.id == certificate_id) {
                        certificate.set(Some(cert));
                    }
                }
                
                // 문제 로드
                match QuestionService::get_by_certificate(&certificate_id).await {
                    Ok(quests) if !quests.is_empty() => {
                        questions.set(quests);
                        quiz_state.set(QuizState::InProgress {
                            current_index: 0,
                            selected_option: None,
                            show_result: false,
                            is_correct: false,
                        });
                    }
                    _ => quiz_state.set(QuizState::NoQuestions),
                }
            });
            || ()
        });
    }
    
    // ... (on_option_select callback remains the same)
    let on_option_select = {
        let quiz_state = quiz_state.clone();
        Callback::from(move |option_id: String| {
            if let QuizState::InProgress {
                current_index,
                show_result,
                ..
            } = &*quiz_state
            {
                if !show_result {
                    quiz_state.set(QuizState::InProgress {
                        current_index: *current_index,
                        selected_option: Some(option_id),
                        show_result: false,
                        is_correct: false,
                    });
                }
            }
        })
    };


    let on_submit_answer = {
        let quiz_state = quiz_state.clone();
        let questions = questions.clone();
        let attempt_results = attempt_results.clone();

        Callback::from(move |_| {
            if let QuizState::InProgress { current_index, selected_option, .. } = &*quiz_state {
                if let (Some(selected_id), Some(question)) = (selected_option, questions.get(*current_index)) {
                    let is_correct = question.options.iter().any(|opt| opt.id == *selected_id && opt.is_correct);

                    if is_correct {
                        let mut results = (*attempt_results).clone();
                        results.push(true);
                        attempt_results.set(results);
                    }

                    let mut updated_question = question.clone();
                    updated_question.attempt_count += 1;
                    if is_correct {
                        updated_question.correct_count += 1;
                    }
                    updated_question.last_attempt = Some(Utc::now());

                    // 비동기로 통계 업데이트
                    spawn_local(async move {
                        let _ = QuestionService::update_stats(&updated_question).await;
                    });

                    quiz_state.set(QuizState::InProgress {
                        current_index: *current_index,
                        selected_option: selected_option.clone(),
                        show_result: true,
                        is_correct,
                    });
                }
            }
        })
    };
    
    // ... (on_retry and on_next_question callbacks remain the same)
    // ...

    // ... (UI Rendering part remains mostly the same)
    // ...

    // The rest of the file can remain as it is, since the logic inside is mostly state manipulation
    // which does not need to be async. The data loading and saving parts are now async.
    // Make sure the entire file content is replaced with this structure.
    let current_question = match &*quiz_state {
        QuizState::InProgress { current_index, .. } => questions.get(*current_index).cloned(),
        _ => None,
    };
    let on_retry = {
        let quiz_state = quiz_state.clone();
        Callback::from(move |_| {
            if let QuizState::InProgress { current_index, .. } = &*quiz_state {
                quiz_state.set(QuizState::InProgress {
                    current_index: *current_index,
                    selected_option: None,
                    show_result: false,
                    is_correct: false,
                });
            }
        })
    };

    let on_next_question = {
        let quiz_state = quiz_state.clone();
        let questions = questions.clone();
        let attempt_results = attempt_results.clone();

        Callback::from(move |_| {
            if let QuizState::InProgress {
                current_index,
                is_correct,
                ..
            } = &*quiz_state
            {
                if !is_correct {
                    return;
                }

                let next_index = current_index + 1;

                if next_index >= questions.len() {
                    let correct_count = attempt_results.iter().filter(|&&x| x).count();
                    quiz_state.set(QuizState::Completed {
                        total_questions: questions.len(),
                        correct_answers: correct_count,
                    });
                } else {
                    quiz_state.set(QuizState::InProgress {
                        current_index: next_index,
                        selected_option: None,
                        show_result: false,
                        is_correct: false,
                    });
                }
            }
        })
    };
    let current_state = (*quiz_state).clone();
    let show_result = match &current_state {
        QuizState::InProgress { show_result, .. } => *show_result,
        _ => false,
    };
    let is_correct = match &current_state {
        QuizState::InProgress { is_correct, .. } => *is_correct,
        _ => false,
    };
    let selected_option = match &current_state {
        QuizState::InProgress {
            selected_option, ..
        } => selected_option.clone(),
        _ => None,
    };
    let current_index = match &current_state {
        QuizState::InProgress { current_index, .. } => *current_index,
        _ => 0,
    };
    html! {
        <div class="max-w-4xl mx-auto px-4 py-5 sm:p-6">
            {match &current_state {
                QuizState::Loading => html! {
                    <div class="text-center py-12">
                        <p class="text-gray-500">{"문제를 불러오는 중..."}</p>
                    </div>
                },

                QuizState::NoQuestions => html! {
                    <div class="text-center py-12">
                        <p class="text-gray-500 mb-4">{"아직 등록된 문제가 없습니다."}</p>
                        <Link<Route> to={Route::NewQuestion}>
                            <button class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700">
                                {"문제 만들기"}
                            </button>
                        </Link<Route>>
                    </div>
                },

                QuizState::InProgress { .. } => {
                    if let Some(question) = current_question {
                        html! {
                            <div>
                                // 진행 상황
                                <div class="mb-6">
                                    <div class="flex justify-between text-sm text-gray-600 mb-2">
                                        <span>{format!("문제 {} / {}", current_index + 1, questions.len())}</span>
                                        {if let Some(cert) = &*certificate {
                                            html! { <span>{&cert.name}</span> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                    <div class="w-full bg-gray-200 rounded-full h-2">
                                        <div
                                            class="bg-blue-600 h-2 rounded-full transition-all duration-300"
                                            style={format!("width: {}%", (current_index + 1) * 100 / questions.len())}
                                        />
                                    </div>
                                </div>

                                // 문제
                                <div class="bg-white shadow rounded-lg p-6 mb-6">
                                    <h3 class="text-lg font-medium text-gray-900 mb-4">
                                        {&question.content}
                                    </h3>

                                    // 보기들
                                    <div class="space-y-3">
                                        {for question.options.iter().enumerate().map(|(idx, option)| {
                                            let option_id = option.id.clone();
                                            let on_option_select = on_option_select.clone();
                                            let is_selected = selected_option.as_ref() == Some(&option.id);
                                            let show_correct = show_result && option.is_correct;
                                            let show_incorrect = show_result && is_selected && !option.is_correct;

                                            html! {
                                                <div
                                                    key={option.id.clone()}
                                                    onclick={move |_| {
                                                        if !show_result {
                                                            on_option_select.emit(option_id.clone())
                                                        }
                                                    }}
                                                    class={format!(
                                                        "p-4 rounded-lg border-2 transition-all {}",
                                                        if show_correct {
                                                            "border-green-500 bg-green-50"
                                                        } else if show_incorrect {
                                                            "border-red-500 bg-red-50"
                                                        } else if is_selected {
                                                            "border-blue-500 bg-blue-50"
                                                        } else if !show_result {
                                                            "border-gray-300 hover:border-gray-400 cursor-pointer"
                                                        } else {
                                                            "border-gray-300"
                                                        }
                                                    )}
                                                >
                                                    <div class="flex items-start">
                                                        <span class="mr-3 font-medium">{format!("{}.", idx + 1)}</span>
                                                        <div class="flex-1">
                                                            <p>{&option.content}</p>

                                                            // 해설 표시
                                                            {if show_result && !option.explanation.is_empty() {
                                                                html! {
                                                                    <p class="mt-2 text-sm text-gray-600">
                                                                        {&option.explanation}
                                                                    </p>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }}
                                                        </div>

                                                        // 체크/엑스 마크
                                                        {if show_result {
                                                            if option.is_correct {
                                                                html! { <span class="ml-2 text-green-600">{"✓"}</span> }
                                                            } else if is_selected && !option.is_correct {
                                                                html! { <span class="ml-2 text-red-600">{"✗"}</span> }
                                                            } else {
                                                                html! {}
                                                            }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </div>
                                                </div>
                                            }
                                        })}
                                    </div>

                                    // 전체 해설
                                    {if show_result && !question.explanation.is_empty() {
                                        html! {
                                            <div class="mt-6 p-4 bg-gray-50 rounded-lg">
                                                <h4 class="font-medium text-gray-900 mb-2">{"해설"}</h4>
                                                <p class="text-gray-700">{&question.explanation}</p>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>

                                // 버튼 영역
                                <div class="flex justify-between">
                                    <Link<Route> to={Route::CertificateDetail { id: props.certificate_id.clone() }}>
                                        <button class="text-gray-600 hover:text-gray-900">
                                            {"← 돌아가기"}
                                        </button>
                                    </Link<Route>>

                                    <div>
                                        {if show_result {
                                            if is_correct {
                                                html! {
                                                    <button
                                                        onclick={on_next_question}
                                                        class="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700"
                                                    >
                                                        {if current_index == questions.len() - 1 { "결과 보기" } else { "다음 문제" }}
                                                    </button>
                                                }
                                            } else {
                                                html! {
                                                    <button
                                                        onclick={on_retry}
                                                        class="bg-orange-600 text-white px-6 py-2 rounded-md hover:bg-orange-700"
                                                    >
                                                        {"다시 시도"}
                                                    </button>
                                                }
                                            }
                                        } else {
                                            html! {
                                                <button
                                                    onclick={on_submit_answer}
                                                    disabled={selected_option.is_none()}
                                                    class={format!(
                                                        "px-6 py-2 rounded-md {}",
                                                        if selected_option.is_some() { "bg-blue-600 text-white hover:bg-blue-700" }
                                                        else { "bg-gray-300 text-gray-500 cursor-not-allowed" }
                                                    )}
                                                >
                                                    {"답안 제출"}
                                                </button>
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {} // Should be covered by Loading state
                    }
                },

                QuizState::Completed { total_questions, correct_answers } => html! {
                    <div class="bg-white shadow rounded-lg p-8 text-center">
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">{"퀴즈 완료!"}</h2>
                        <div class="mb-6">
                            <p class="text-4xl font-bold text-blue-600 mb-2">
                                {format!("{}/{}", correct_answers, total_questions)}
                            </p>
                            <p class="text-gray-600">
                                {
                                    if *total_questions > 0 {
                                        format!("정답률: {}%", correct_answers * 100 / total_questions)
                                    } else {
                                        "정답률: 0%".to_string()
                                    }
                                }
                            </p>
                        </div>
                        <div class="space-y-3">
                           <Link<Route> to={Route::Quiz { certificate_id: props.certificate_id.clone() }} classes="block">
                                <button class="w-full bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700">
                                    {"다시 풀기"}
                                </button>
                            </Link<Route>>

                            <Link<Route> to={Route::CertificateDetail { id: props.certificate_id.clone() }} classes="block">
                                <button class="w-full bg-gray-200 text-gray-700 px-6 py-3 rounded-md hover:bg-gray-300">
                                    {"자격증 페이지로"}
                                </button>
                            </Link<Route>>
                        </div>
                    </div>
                },
            }}
        </div>
    }
}