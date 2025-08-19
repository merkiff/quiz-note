use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::services::CertificateService;

#[derive(Properties, PartialEq)]
pub struct CertificateFormProps {
    pub on_submit: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(CertificateForm)]
pub fn certificate_form(props: &CertificateFormProps) -> Html {
    let name_ref = use_node_ref();
    let description_ref = use_node_ref();
    let error = use_state(|| None::<String>);
    let is_loading = use_state(|| false);

    let on_submit = {
        let name_ref = name_ref.clone();
        let description_ref = description_ref.clone();
        let on_submit_prop = props.on_submit.clone();
        let error = error.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name_ref.cast::<HtmlInputElement>().map(|i| i.value()).unwrap_or_default();
            let description = description_ref.cast::<HtmlInputElement>().map(|i| i.value()).unwrap_or_default();

            if name.trim().is_empty() {
                error.set(Some("자격증 이름을 입력해주세요.".to_string()));
                return;
            }

            let on_submit_prop = on_submit_prop.clone();
            let error = error.clone();
            let is_loading = is_loading.clone();
            spawn_local(async move {
                is_loading.set(true);
                error.set(None);
                match CertificateService::create(name, description).await {
                    Ok(_) => {
                        on_submit_prop.emit(());
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <form onsubmit={on_submit} class="bg-white shadow-sm rounded-lg p-6">
            <div class="space-y-4">
                <div>
                    <label for="name" class="block text-sm font-medium text-gray-700">
                        {"자격증 이름"}
                    </label>
                    <input
                        ref={name_ref}
                        type="text"
                        id="name"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                        placeholder="예: 정보처리기사"
                        disabled={*is_loading}
                    />
                </div>

                <div>
                    <label for="description" class="block text-sm font-medium text-gray-700">
                        {"설명"}
                    </label>
                    <input
                        ref={description_ref}
                        type="text"
                        id="description"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                        placeholder="자격증에 대한 간단한 설명"
                        disabled={*is_loading}
                    />
                </div>

                if let Some(err) = &*error {
                    <div class="text-red-600 text-sm">
                        {err}
                    </div>
                }

                <div class="flex justify-end space-x-3">
                    <button
                        type="button"
                        onclick={let on_cancel = props.on_cancel.clone(); move |_| on_cancel.emit(())}
                        class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                        disabled={*is_loading}
                    >
                        {"취소"}
                    </button>
                    <button
                        type="submit"
                        class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
                        disabled={*is_loading}
                    >
                        { if *is_loading { "저장 중..." } else { "저장" } }
                    </button>
                </div>
            </div>
        </form>
    }
}