use axum::{
    extract::{Extension, Query},
    response::{Headers, IntoResponse},
    routing, Router,
};
use serde::Deserialize;

use crate::controllers::{accounts, tweets};
use crate::database::{self, RepositoryProvider};
use crate::request::UserContext;
use crate::response;
use crate::services;
use crate::views::{SignIn, SignUp};

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .route("/login", routing::get(login))
        .route("/register", routing::get(register))
        .nest("/tweets", tweets::tweets()) // .nest()は第二引数にRouterを受け取る
        .nest("/accounts", accounts::accounts())
        .layer(database_layer) // 受信するすべてのリクエストのExtensionにオブジェクトを挿入するミドルウェアを追加
}

// ハンドラ関数の戻り値はIntoResponseトレイトを実装している必要がある
async fn get(
    _: UserContext, // 認証が必要なリクエストのハンドラー関数の引数にUserContext型の引数を追加するだけで、自動的にミドルウェアが割り当てられる？
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let tweet_repo = repository_provider.tweets(); // Tweetsリポジトリの実装を取得する
    let account_repo = repository_provider.accounts(); // Accountsリポジトリの実装を取得する
    let home = services::list_tweets(&tweet_repo, &account_repo).await; // リポジトリの実装を受け取ってビューを返す
    response::from_template(home)
}

// ログインページを返す
async fn login(query: Query<LoginQuery>) -> impl IntoResponse {
    let empty_session_token = services::clear_session();
    let headers = Headers(vec![("Set-Cookie", empty_session_token.cookie())]);
    let response = response::from_template(SignIn {
        error: query.error.is_some(),
    });
    (headers, response)
}

// サインアップページを返す
async fn register() -> impl IntoResponse {
    response::from_template(SignUp)
}

#[derive(Deserialize)]
struct LoginQuery {
    error: Option<String>,
}
