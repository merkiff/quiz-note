# QuizNote: 객관식 문제 오답노트 웹 애플리케이션

QuizNote는 자격증 시험이나 학습을 위한 객관식 문제를 효율적으로 관리하고 풀어볼 수 있는 웹 애플리케이션입니다. Rust와 Yew를 사용해 개발되었으며, 백엔드로는 Supabase를 활용하여 데이터를 관리합니다.

## ✨ 주요 기능

- **이메일 기반 로그인**: 비밀번호 없이 이메일로 전송된 매직 링크를 통해 간편하게 로그인합니다.
- **자격증 관리**: 학습하려는 자격증 목록을 생성하고 관리할 수 있습니다.
- **문제 관리**: 자격증별로 객관식 문제를 생성, 수정, 삭제할 수 있습니다. 문제와 함께 해설 및 보기를 등록합니다.
- **페이지네이션**: 문제 목록이 길어질 경우, 페이지네이션을 통해 한 페이지에 적정 수의 문제만 표시하여 가독성을 높입니다.
- **랜덤 퀴즈**: 문제를 풀 때마다 순서가 무작위로 섞여, 문제의 위치를 외우는 것을 방지하고 학습 효과를 높입니다.
- **풀이 통계**: 문제별로 시도 횟수 및 정답 횟수를 기록하여 취약한 문제를 쉽게 파악할 수 있습니다.

## 🛠 기술 스택

- **Frontend**: Rust, Yew, WebAssembly
- **Build Tool**: Trunk
- **Backend**: Supabase (Auth, PostgreSQL DB)
- **Styling**: Tailwind CSS (via CDN)
- **Deployment**: Netlify, GitHub Pages

## 🚀 시작하기

### 1. 환경 설정

먼저 Rust와 `trunk`를 설치해야 합니다.

```bash
# Rust 설치 (이미 설치되어 있다면 생략)
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
source $HOME/.cargo/env

# wasm 타겟 추가
rustup target add wasm32-unknown-unknown

# Trunk 설치
cargo install trunk