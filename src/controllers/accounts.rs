use axum::{
    extract::{Extension, Form},
    http::Uri,
    response::{Headers, IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepositoryProvider;
use crate::services::{self, SessionToken};

pub fn accounts() -> Router {
    Router::new()
        .route("/new", routing::post(post))
        .route("/session", routing::post(new_session)) // ログイン処理用のエンドポイント
}

// アカウント新規作成
async fn post(
    form: Form<SignUpForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let account_repo = repository_provider.accounts(); // Accountsリポジトリの実装を取得
    services::create_account(
        &account_repo,
        &form.email,
        &form.password,
        &form.display_name,
    )
    .await;
    let session_token = services::create_session(&account_repo, &form.email, &form.password).await;
    redirect_with_session(session_token)
}

async fn new_session(
    form: Form<SignInForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let account_repo = repository_provider.accounts();
    let session_token = services::create_session(&account_repo, &form.email, &form.password).await;
    redirect_with_session(session_token)
}

fn redirect_with_session(
    session: Option<SessionToken>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(session_token) = session {
        let headers = Headers(vec![("Set-Cookie", session_token.cookie())]);
        let response = Redirect::to(Uri::from_static("/"));
        // <T: IntoResponse> (Headers, T) 型も IntoResponse を実装しています。
        Ok((headers, response)) // レスポンスヘッダ付きでリダイレクトさせるときはタプルで書く？
    } else {
        Err(Redirect::to(Uri::from_static("/login?error=invalid"))) // ログインページを再度表示
    }
}

#[derive(Deserialize)]
struct SignInForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct SignUpForm {
    email: String,
    password: String,
    display_name: String,
}
