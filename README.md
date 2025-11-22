# QuizNote: 웹어셈블리 기반 객관식 오답노트

QuizNote는 자격증 시험이나 학습을 위한 객관식 문제를 효율적으로 관리하고, 랜덤 퀴즈를 통해 반복 학습할 수 있는 웹 애플리케이션입니다. Rust와 Yew 프레임워크를 사용하여 빠르고 안정적인 WebAssembly 앱으로 구현되었습니다.

## ✨ 주요 기능

### 1. 🔐 인증 및 사용자 관리
- **이메일 매직 링크 로그인**: 비밀번호 없이 이메일로 전송된 링크를 통해 안전하고 간편하게 로그인합니다.
- **세션 관리**: 로그인 상태 유지 및 토큰 만료 시 자동 갱신 기능을 지원합니다.

### 2. 📝 문제 및 자격증 관리
- **자격증(카테고리) 생성**: 시험 과목이나 자격증별로 문제를 그룹화하여 관리할 수 있습니다.
- **마크다운(Markdown) 지원**:
  - 문제, 보기, 해설에 **굵게**, *기울임*, `코드 블록`, 표 등 다양한 서식을 적용할 수 있습니다.
  - 문제 작성 시 **실시간 미리보기(Split View)**를 제공하여 렌더링 결과를 바로 확인할 수 있습니다.
- **문제 검색**: 키워드로 등록된 문제와 보기를 실시간으로 검색할 수 있습니다.
- **스마트 페이지네이션**: 문제가 많아져도 10페이지 단위로 끊어서 보여주어 탐색이 편리합니다.

### 3. 🎯 퀴즈 풀이 시스템
- **랜덤 출제**: 자격증 내의 문제를 무작위 순서로 섞어서 출제하여 암기식 학습을 방지합니다.
- **즉시 피드백**: 정답 선택 시 즉시 정답/오답 여부와 상세 해설을 확인할 수 있습니다.
- **학습 통계**: 각 문제별 시도 횟수와 정답 횟수를 기록하여 취약한 문제를 파악할 수 있습니다.

### 4. 💾 데이터 백업 및 복원
- **JSON 내보내기/가져오기**: 작성한 모든 데이터를 JSON 파일로 백업하거나, 다른 기기에서 복원할 수 있습니다.

## 🛠 기술 스택

- **Frontend**: [Rust](https://www.rust-lang.org/), [Yew](https://yew.rs/) (0.21), WebAssembly
- **Build Tool**: [Trunk](https://trunkrs.dev/)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) (Typography Plugin 포함)
- **Backend**: [Supabase](https://supabase.com/) (PostgreSQL, Auth)
- **Libraries**:
  - `pulldown-cmark`: 마크다운 파싱 및 렌더링
  - `gloo-net`: 비동기 HTTP 통신
  - `serde`: 데이터 직렬화/역직렬화

## 🚀 시작하기

### 1. 필수 도구 설치
Rust와 Trunk가 설치되어 있어야 합니다.

```bash
# 1. Rust 설치 (이미 설치된 경우 생략)
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

# 2. WebAssembly 타겟 추가
rustup target add wasm32-unknown-unknown

# 3. Trunk 설치
cargo install trunk
````

### 2\. 프로젝트 실행

프로젝트 폴더에서 다음 명령어를 실행합니다.

```bash
# 로컬 개발 서버 실행 (자동 리로딩 지원)
trunk serve
```

실행 후 브라우저에서 `http://localhost:8080`으로 접속합니다.

### 3\. 배포 빌드

프로젝트를 배포용으로 빌드하려면 다음 명령어를 사용합니다.

```bash
trunk build --release
```

`dist/` 폴더에 생성된 정적 파일들을 GitHub Pages, Netlify, Vercel 등에 업로드하여 배포할 수 있습니다.

## ⚙️ 환경 설정

`src/config/supabase.rs` 파일에서 본인의 Supabase 프로젝트 설정으로 변경해야 합니다.

```rust
pub static SUPABASE_CONFIG: Lazy<SupabaseConfig> = Lazy::new(|| SupabaseConfig {
    url: "https://YOUR_PROJECT_ID.supabase.co",
    anon_key: "YOUR_ANON_KEY",
});
```

## 📂 프로젝트 구조

```
src/
├── components/         # UI 컴포넌트
│   ├── auth/           # 로그인 관련
│   ├── certificate/    # 자격증 목록/상세/폼
│   ├── question/       # 문제 목록/폼 (마크다운 에디터)
│   ├── quiz/           # 퀴즈 풀이 로직
│   ├── data/           # 데이터 내보내기/가져오기
│   └── markdown.rs     # 마크다운 렌더링 공통 컴포넌트
├── config/             # 환경 변수 및 설정
├── models/             # 데이터 구조체 (Structs)
├── services/           # 비즈니스 로직 및 API 통신
└── main.rs             # 앱 진입점 및 라우팅
```