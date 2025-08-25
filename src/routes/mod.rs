use crate::components::auth::Login;
use crate::components::{CertificateDetail, CertificateList, Home, QuestionForm, QuizPage};
use crate::services::AuthService;
use yew::prelude::*;
use yew_router::prelude::*;

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

// 제네릭 T를 제거하고 Redirect<Route>로 명시적으로 변경
fn render_protected_route(component: Html) -> Html {
    if AuthService::is_authenticated() {
        component
    } else {
        html! { <Redirect<Route> to={Route::Login} /> }
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        // 헬퍼 함수 호출 시 타입 인자 제거
        Route::Home => render_protected_route(html! { <Home /> }),
        Route::Certificates => render_protected_route(html! { <CertificateList /> }),
        Route::CertificateDetail { id } => {
            render_protected_route(html! { <CertificateDetail {id} /> })
        }
        Route::NewQuestion => render_protected_route(html! { <QuestionForm /> }),
        Route::EditQuestion { id } => {
            render_protected_route(html! { <QuestionForm id={Some(id)} /> })
        }
        Route::Quiz { certificate_id } => {
            render_protected_route(html! { <QuizPage {certificate_id} /> })
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