use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::models::Question;
use crate::routes::Route;
use crate::services::QuestionService;
use web_sys::{window, HtmlInputElement}; // HtmlInputElement 추가

#[derive(Properties, PartialEq)]
pub struct QuestionListProps {
    pub certificate_id: String,
}

const QUESTIONS_PER_PAGE: usize = 10;
const PAGES_PER_VIEW: usize = 10; // 한 번에 보여줄 페이지 번호 개수

#[function_component(QuestionList)]
pub fn question_list(props: &QuestionListProps) -> Html {
    let questions = use_state(Vec::<Question>::new);
    let error = use_state(|| None::<String>);
    let is_loading = use_state(|| true);
    let current_page = use_state(|| 1);
    let search_term = use_state(String::new); // 검색어 상태 추가

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

    // 검색어 변경 핸들러
    let on_search = {
        let search_term = search_term.clone();
        let current_page = current_page.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_term.set(input.value());
            current_page.set(1); // 검색 시 1페이지로 초기화
        })
    };

    // 검색 필터링 로직
    let filtered_questions: Vec<&Question> = questions.iter()
        .filter(|q| {
            let term = search_term.to_lowercase();
            if term.is_empty() {
                return true;
            }
            // 문제 내용이나 보기에 검색어가 포함되어 있는지 확인
            q.content.to_lowercase().contains(&term) ||
            q.options.iter().any(|opt| opt.content.to_lowercase().contains(&term))
        })
        .collect();
    
    // 페이지네이션 계산
    let total_items = filtered_questions.len();
    let total_pages = (total_items as f64 / QUESTIONS_PER_PAGE as f64).ceil() as usize;

    // 현재 페이지가 속한 그룹 계산 (예: 1~10페이지는 0그룹, 11~20페이지는 1그룹)
    let current_page_group = (*current_page - 1) / PAGES_PER_VIEW;
    let start_page = current_page_group * PAGES_PER_VIEW + 1;
    let end_page = (start_page + PAGES_PER_VIEW - 1).min(total_pages);

    let start_index = (*current_page - 1) * QUESTIONS_PER_PAGE;
    let current_view_questions = filtered_questions.into_iter().skip(start_index).take(QUESTIONS_PER_PAGE);

    html! {
        <div class="space-y-4">
            <div class="flex justify-between items-center">
                <h3 class="text-lg font-medium text-gray-900">{"문제 목록"}</h3>
                // 검색 입력창 추가
                <div class="relative rounded-md shadow-sm">
                    <input
                        type="text"
                        class="block w-full rounded-md border-gray-300 pl-3 pr-10 focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                        placeholder="문제 검색..."
                        value={(*search_term).clone()}
                        oninput={on_search}
                    />
                </div>
            </div>

            {
                if *is_loading {
                    html!{ <div class="text-center py-8 text-gray-500">{"문제를 불러오는 중..."}</div> }
                } else if let Some(err) = &*error {
                    html! { <div class="text-red-600 text-center py-8">{format!("오류: {}", err)}</div> }
                } else if total_items == 0 {
                    html! {
                        <div class="text-center py-8 text-gray-500">
                            {if search_term.is_empty() {
                                "등록된 문제가 없습니다."
                            } else {
                                "검색 결과가 없습니다."
                            }}
                        </div>
                    }
                } else {
                    html! {
                        <>
                            <div class="space-y-4">
                                {for current_view_questions.enumerate().map(|(idx_on_page, question)| {
                                    let question_id = question.id.clone();
                                    let on_delete = on_delete.clone();
                                    // 전체 목록에서의 실제 인덱스 (검색 결과 기준 아님, 현재 페이지 기준 번호 표시)
                                    let display_number = total_items - (start_index + idx_on_page) + 1; // 역순 번호 표시 예시 (선택 사항)
                                    // 또는 기존 방식대로:
                                    // let display_number = start_index + idx_on_page + 1;

                                    html! {
                                        <div key={question_id.clone()} class="bg-gray-50 shadow-sm rounded-lg p-4">
                                            <div class="flex justify-between items-start">
                                                <div class="flex-1">
                                                    <p class="font-medium text-gray-900">
                                                        // 검색어 하이라이팅은 복잡하므로 단순 텍스트 표시
                                                        {format!("문제: {}", &question.content)}
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

                            // 개선된 페이지네이션 UI
                            if total_pages > 1 {
                                <div class="flex justify-center mt-6 space-x-2">
                                    // 이전 페이지 버튼
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
                                    
                                    // start_page 부터 end_page 까지만 렌더링
                                    { for (start_page..=end_page).map(|page| {
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

                                    // 다음 페이지 버튼
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