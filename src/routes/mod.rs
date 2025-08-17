use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{Home, CertificateList, CertificateDetail, QuestionForm, QuizPage};
use crate::components::auth::Login;
use crate::services::AuthService;

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
    let is_authenticated = AuthService::is_authenticated();
    
    match routes {
        Route::Login => html! { <Login /> },
        Route::Home => {
            if is_authenticated {
                html! { <Home /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
        Route::Certificates => {
            if is_authenticated {
                html! { <CertificateList /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
        Route::CertificateDetail { id } => {
            if is_authenticated {
                html! { <CertificateDetail {id} /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
        Route::NewQuestion => {
            if is_authenticated {
                html! { <QuestionForm /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
        Route::EditQuestion { id } => {
            if is_authenticated {
                html! { <h1>{format!("문제 수정: {}", id)}</h1> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
        Route::Quiz { certificate_id } => {
            if is_authenticated {
                html! { <QuizPage {certificate_id} /> }
            } else {
                html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
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