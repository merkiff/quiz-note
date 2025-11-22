use crate::models::{Certificate, Question, QuestionOption};
use crate::routes::Route;
use crate::services::{CertificateService, QuestionService};
use crate::components::Markdown; // [추가됨]
use chrono::Utc;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuizPageProps {
    pub certificate_id: String,
}

#[derive(Clone, PartialEq)]
enum QuizState {
    Loading,
    NoQuestions,
    InProgress {
        current_index: usize,
        tried_incorrect_options: HashSet<String>,
        is_solved: bool,
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
    let correct_answer_count = use_state(|| 0);

    {
        let certificate = certificate.clone();
        let questions = questions.clone();
        let quiz_state = quiz_state.clone();
        let certificate_id = props.certificate_id.clone();

        use_effect_with(certificate_id.clone(), move |_| {
            spawn_local(async move {
                if let Ok(certs) = CertificateService::get_all().await {
                    if let Some(cert) = certs.into_iter().find(|c| c.id == certificate_id) {
                        certificate.set(Some(cert));
                    }
                }
                
                match QuestionService::get_by_certificate(&certificate_id).await {
                    Ok(mut quests) if !quests.is_empty() => {
                        let mut rng = thread_rng();
                        quests.shuffle(&mut rng);
                        questions.set(quests);
                        quiz_state.set(QuizState::InProgress {
                            current_index: 0,
                            tried_incorrect_options: HashSet::new(),
                            is_solved: false,
                        });
                    }
                    _ => quiz_state.set(QuizState::NoQuestions),
                }
            });
            || ()
        });
    }
    
    let on_option_click = {
        let quiz_state = quiz_state.clone();
        let questions = questions.clone();
        let correct_answer_count = correct_answer_count.clone();

        Callback::from(move |option: QuestionOption| {
            if let QuizState::InProgress { current_index, mut tried_incorrect_options, is_solved } = (*quiz_state).clone() {
                if is_solved { return; }

                if let Some(question) = questions.get(current_index) {
                    let mut updated_question = question.clone();
                    
                    if option.is_correct {
                        if tried_incorrect_options.is_empty() {
                            updated_question.correct_count += 1;
                            correct_answer_count.set(*correct_answer_count + 1);
                        }
                        updated_question.attempt_count += 1;
                        updated_question.last_attempt = Some(Utc::now());
                        
                        quiz_state.set(QuizState::InProgress {
                            current_index,
                            tried_incorrect_options,
                            is_solved: true,
                        });

                    } else {
                        if !tried_incorrect_options.contains(&option.id) {
                            tried_incorrect_options.insert(option.id);
                             if tried_incorrect_options.len() == 1 {
                                updated_question.attempt_count += 1;
                                updated_question.last_attempt = Some(Utc::now());
                            }
                            quiz_state.set(QuizState::InProgress {
                                current_index,
                                tried_incorrect_options,
                                is_solved: false,
                            });
                        }
                    }

                     spawn_local(async move {
                        let _ = QuestionService::update_stats(&updated_question).await;
                    });
                }
            }
        })
    };

    let on_next_question = {
        let quiz_state = quiz_state.clone();
        let questions = questions.clone();
        let correct_answer_count = correct_answer_count.clone();

        Callback::from(move |_| {
            if let QuizState::InProgress { current_index, .. } = &*quiz_state {
                let next_index = current_index + 1;
                if next_index >= questions.len() {
                    quiz_state.set(QuizState::Completed {
                        total_questions: questions.len(),
                        correct_answers: *correct_answer_count,
                    });
                } else {
                    quiz_state.set(QuizState::InProgress {
                        current_index: next_index,
                        tried_incorrect_options: HashSet::new(),
                        is_solved: false,
                    });
                }
            }
        })
    };

    let current_state = (*quiz_state).clone();
    let current_question = match &current_state {
        QuizState::InProgress { current_index, .. } => questions.get(*current_index).cloned(),
        _ => None,
    };

    html! {
        <div class="max-w-4xl mx-auto px-4 py-5 sm:p-6">
            {match current_state {
                QuizState::Loading => html! { <div class="text-center py-12"><p class="text-gray-500">{"문제를 불러오는 중..."}</p></div> },

                QuizState::NoQuestions => html! {
                    <div class="text-center py-12">
                        <p class="text-gray-500 mb-4">{"아직 등록된 문제가 없습니다."}</p>
                        <Link<Route> to={Route::NewQuestion}>
                            <button class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700">{"문제 만들기"}</button>
                        </Link<Route>>
                    </div>
                },

                QuizState::InProgress { current_index, tried_incorrect_options, is_solved } => {
                    if let Some(question) = current_question {
                        html! {
                            <div>
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
                                        <div class="bg-blue-600 h-2 rounded-full transition-all duration-300" style={format!("width: {}%", (current_index + 1) * 100 / questions.len())} />
                                    </div>
                                </div>

                                <div class="bg-white shadow rounded-lg p-6 mb-6">
                                    // [수정됨] 문제 내용 마크다운 적용
                                    <h3 class="text-lg font-medium text-gray-900 mb-4 prose max-w-none">
                                        <Markdown content={question.content.clone()} />
                                    </h3>

                                    <div class="space-y-3">
                                        {for question.options.iter().map(|option| {
                                            let on_option_click = on_option_click.clone();
                                            let option_clone = option.clone();
                                            
                                            let is_tried_incorrect = tried_incorrect_options.contains(&option.id);
                                            let show_correct = is_solved && option.is_correct;
                                            
                                            let mut classes = "p-4 rounded-lg border-2 transition-all".to_string();
                                            let is_clickable = !is_solved && !is_tried_incorrect;

                                            if show_correct {
                                                classes.push_str(" border-green-500 bg-green-50");
                                            } else if is_tried_incorrect {
                                                classes.push_str(" border-red-500 bg-red-50 cursor-not-allowed");
                                            } else if is_solved {
                                                classes.push_str(" border-gray-300 bg-gray-50 cursor-not-allowed");
                                            } else {
                                                classes.push_str(" border-gray-300 hover:border-blue-500 cursor-pointer");
                                            }

                                            html! {
                                                <div
                                                    key={option.id.clone()}
                                                    onclick={ if is_clickable { Some(Callback::from(move |_| on_option_click.emit(option_clone.clone()))) } else { None } }
                                                    class={classes}
                                                >
                                                    <div class="flex items-start">
                                                        <span class="mr-3 font-medium pt-1">{format!("{}.", option.display_order + 1)}</span>
                                                        <div class="flex-1">
                                                            // [수정됨] 보기 내용 마크다운 적용
                                                            <div class="prose prose-sm max-w-none">
                                                                <Markdown content={option.content.clone()} />
                                                            </div>
                                                            {if (is_solved || is_tried_incorrect) && !option.explanation.is_empty() {
                                                                html! {
                                                                    // [수정됨] 보기 해설 마크다운 적용
                                                                    <div class="mt-2 text-sm text-gray-600 prose prose-sm max-w-none bg-white p-2 rounded border border-gray-100">
                                                                        <Markdown content={option.explanation.clone()} />
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }}
                                                        </div>
                                                        {if show_correct {
                                                            html! { <span class="ml-2 text-green-600 font-bold">{"✓"}</span> }
                                                        } else if is_tried_incorrect {
                                                            html! { <span class="ml-2 text-red-600 font-bold">{"✗"}</span> }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </div>
                                                </div>
                                            }
                                        })}
                                    </div>

                                    {if is_solved && !question.explanation.is_empty() {
                                        html! {
                                            <div class="mt-6 p-4 bg-gray-50 rounded-lg border border-gray-200">
                                                <h4 class="font-medium text-gray-900 mb-2">{"전체 해설"}</h4>
                                                // [수정됨] 전체 해설 마크다운 적용
                                                <div class="text-gray-700 prose prose-sm max-w-none">
                                                    <Markdown content={question.explanation.clone()} />
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>

                                <div class="flex justify-between">
                                    <Link<Route> to={Route::CertificateDetail { id: props.certificate_id.clone() }}>
                                        <button class="text-gray-600 hover:text-gray-900">{"← 돌아가기"}</button>
                                    </Link<Route>>
                                    {if is_solved {
                                        html! {
                                            <button onclick={on_next_question} class="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700">
                                                {if current_index == questions.len() - 1 { "결과 보기" } else { "다음 문제" }}
                                            </button>
                                        }
                                    } else {
                                        html! { <div class="text-gray-500 italic">{"정답을 선택하면 다음으로 넘어갈 수 있습니다."}</div> }
                                    }}
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                },
                QuizState::Completed { total_questions, correct_answers } => html! {
                     <div class="bg-white shadow rounded-lg p-8 text-center">
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">{"퀴즈 완료!"}</h2>
                        <div class="mb-6">
                            <p class="text-4xl font-bold text-blue-600 mb-2">{format!("{}/{}", correct_answers, total_questions)}</p>
                            <p class="text-gray-600">
                                {if total_questions > 0 { format!("첫 시도 정답률: {}%", correct_answers * 100 / total_questions) } else { "정답률: 0%".to_string() }}
                            </p>
                        </div>
                        <div class="space-y-3">
                           <Link<Route> to={Route::Quiz { certificate_id: props.certificate_id.clone() }} classes="block">
                                <button class="w-full bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700">{"다시 풀기"}</button>
                            </Link<Route>>
                            <Link<Route> to={Route::CertificateDetail { id: props.certificate_id.clone() }} classes="block">
                                <button class="w-full bg-gray-200 text-gray-700 px-6 py-3 rounded-md hover:bg-gray-300">{"자격증 페이지로"}</button>
                            </Link<Route>>
                        </div>
                    </div>
                },
            }}
        </div>
    }
}