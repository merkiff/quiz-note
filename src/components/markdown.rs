use yew::prelude::*;
use web_sys::Element;
use pulldown_cmark::{Parser, html, Options};

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub content: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    // 1. div 요소 생성
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    // 2. 마크다운 옵션 설정
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    // 3. HTML 변환
    let parser = Parser::new_ext(&props.content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 4. HTML 주입 및 Tailwind Typography 클래스 적용
    div.set_inner_html(&html_output);
    // 'prose' 클래스가 마크다운 스타일을 자동으로 적용해줍니다.
    let _ = div.set_class_name("prose prose-sm max-w-none break-words");

    // 5. VRef로 반환
    Html::VRef(div.into())
}