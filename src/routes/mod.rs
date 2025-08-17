use crate::components::auth::Login;
use crate::components::{CertificateDetail, CertificateList, Home, QuestionForm, QuizPage};
use crate::services::AuthService;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::window;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("/certificates")]
    Certificates,
    #[at("/certificates/:id")]
    CertificateDetail { id: String },
    #[at("/questions/new")]
    NewQuestion,
    #[at("/questions/:id/edit")]
    EditQuestion { id: String },
    #[at("/quiz/:certificate_id")]
    Quiz { certificate_id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    // 로그인 체크
    // 해시에 access_token이 있으면 처리 중이므로 로딩 표시
    let hash = window().unwrap().location().hash().unwrap_or_default();
    if hash.contains("access_token") {
        return html! {
            <div class="min-h-screen flex items-center justify-center">
                <div class="text-center">
                    <p class="text-gray-500 mb-2">{"로그인 처리 중..."}</p>
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
                </div>
            </div>
        };
    }
    let is_authenticated = AuthService::is_authenticated();

    match routes {
        Route::Login => html! { <Login /> },
        Route::Home => {
            if is_authenticated {
                html! { <Home /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
        Route::Certificates => {
            if is_authenticated {
                html! { <CertificateList /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
        Route::CertificateDetail { id } => {
            if is_authenticated {
                html! { <CertificateDetail {id} /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
        Route::NewQuestion => {
            if is_authenticated {
                html! { <QuestionForm /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
        Route::EditQuestion { id } => {
            if is_authenticated {
                html! { <h1>{format!("문제 수정: {}", id)}</h1> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
        Route::Quiz { certificate_id } => {
            if is_authenticated {
                html! { <QuizPage {certificate_id} /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        }
        Route::NotFound => html! {
            <div class="text-center py-12">
                <h1 class="text-2xl font-bold text-gray-900">{"404 - 페이지를 찾을 수 없습니다"}</h1>
                <Link<Route> to={Route::Home}>
                    <button class="mt-4 text-blue-600 hover:text-blue-900">
                        {"홈으로 돌아가기"}
                    </button>
                </Link<Route>>
            </div>
        },
    }
}
