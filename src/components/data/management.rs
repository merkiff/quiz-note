use crate::services::DataService;
use gloo_file::{futures::read_as_text, Blob, File, ObjectUrl};
use wasm_bindgen::JsCast; // JsCast 트레이트를 가져옵니다.
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[function_component(DataManagement)]
pub fn data_management() -> Html {
    let message = use_state(|| None::<String>);
    let is_loading = use_state(|| false);
    let file_input_ref = use_node_ref();

    let on_export = {
        let message = message.clone();
        let is_loading = is_loading.clone();
        Callback::from(move |_| {
            let message = message.clone();
            let is_loading = is_loading.clone();
            is_loading.set(true);
            message.set(Some("데이터를 내보내는 중...".to_string()));

            spawn_local(async move {
                match DataService::export_data().await {
                    Ok(json_str) => {
                        let blob = Blob::new_with_options(
                            json_str.as_bytes(),
                            Some("application/json"),
                        );
                        let url = ObjectUrl::from(blob);

                        let a: HtmlElement = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .create_element("a")
                            .unwrap()
                            .dyn_into()
                            .unwrap();
                        
                        let now = chrono::Local::now();
                        let filename = format!("quiznote_backup_{}.json", now.format("%Y%m%d_%H%M%S"));

                        a.set_attribute("href", &url).unwrap();
                        a.set_attribute("download", &filename).unwrap();
                        a.click();
                        message.set(Some(format!("'{}' 파일로 내보내기 성공!", filename)));
                    }
                    Err(e) => message.set(Some(format!("내보내기 실패: {}", e))),
                }
                is_loading.set(false);
            });
        })
    };

    let on_import_change = {
        let message = message.clone();
        let is_loading = is_loading.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file = File::from(file);
                    let message = message.clone();
                    let is_loading = is_loading.clone();
                    is_loading.set(true);
                    message.set(Some("파일을 읽고 데이터를 가져오는 중...".to_string()));

                    spawn_local(async move {
                        let content = read_as_text(&file).await.unwrap_or_default();
                        match DataService::import_data(&content).await {
                            Ok(msg) => {
                                message.set(Some(msg));
                                // 3초 후 페이지 새로고침
                                gloo::timers::callback::Timeout::new(3000, || {
                                    web_sys::window().unwrap().location().reload().unwrap();
                                }).forget();
                            }
                            Err(e) => message.set(Some(format!("가져오기 실패: {}", e))),
                        }
                        is_loading.set(false);
                    });
                }
            }
        })
    };

    let on_import_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };


    html! {
        <div class="px-4 py-5 sm:p-6">
            <h2 class="text-2xl font-bold text-gray-900 mb-6">{"데이터 관리"}</h2>

            <div class="bg-white shadow rounded-lg p-6 space-y-6">
                <div>
                    <h3 class="text-lg font-medium text-gray-900">{"데이터 내보내기"}</h3>
                    <p class="mt-1 text-sm text-gray-600">
                        {"모든 자격증과 문제 데이터를 JSON 파일로 백업합니다."}
                    </p>
                    <button
                        onclick={on_export}
                        disabled={*is_loading}
                        class="mt-3 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50"
                    >
                        {"내보내기"}
                    </button>
                </div>

                <div class="border-t border-gray-200"></div>

                <div>
                    <h3 class="text-lg font-medium text-gray-900">{"데이터 가져오기"}</h3>
                    <p class="mt-1 text-sm text-gray-600">
                        {"백업한 JSON 파일을 불러와 데이터를 복원합니다. 기존 데이터에 추가됩니다."}
                    </p>
                    <input type="file" ref={file_input_ref} onchange={on_import_change} accept=".json" class="hidden" />
                    <button
                        onclick={on_import_click}
                        disabled={*is_loading}
                        class="mt-3 inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50"
                    >
                        {"파일 선택 및 가져오기"}
                    </button>
                </div>

                 {if let Some(msg) = &*message {
                    html! {
                        <div class="mt-4 p-4 bg-gray-50 rounded-lg text-center">
                            if *is_loading {
                                <div class="flex items-center justify-center">
                                    <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-600 mr-3"></div>
                                    <span>{msg}</span>
                                </div>
                            } else {
                                <p class="text-gray-700">{msg}</p>
                            }
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}