use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};
use async_session::SessionStore;
use async_sqlx_session::PostgresSessionStore;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::headers::Cookie;
use axum::http::Uri;
use axum::response::Redirect;
use serde::{Deserialize, Serialize};

/// ハンドラ関数の引数に指定するとミドルウェアとして振る舞う？
#[derive(Deserialize, Serialize)]
pub struct UserContext {
    pub user_id: i32,
}

// リクエストの検証を行いたい場合には FromRequestトレイトを実装します。
// FromRequestトレイトを実装した構造体をハンドラ関数の引数に渡すと、ミドルウェアとして動作する？
#[axum::async_trait]
impl<B> FromRequest<B> for UserContext
where
    B: Send,
{
    // Rejectionに from_request() が失敗した場合の型を宣言する(Err列挙子の中身の型？)
    type Rejection = Redirect;

    // リクエストのCookieヘッダからセッションキーを取得して、セッションストアからセッションを取得してUserContext構造体に詰め込んで返す
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let redirect = || Redirect::to(Uri::from_static("/login"));

        let database_url = database_url();
        // セッションストアを作成
        let store = PostgresSessionStore::new(&database_url)
            .await
            .map_err(|_| redirect())?;

        // from_requestでリクエストのCookieヘッダからクッキーの抽出を実行する？
        let cookies = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap()
            .ok_or(redirect())?; // ok_or()でOptionをResultに変換
        let session_str = cookies.get(AXUM_SESSION_COOKIE_NAME).ok_or(redirect())?; // store.store_session(session)の結果返されるクッキー文字列を取得

        // セッションストアからセッションを取り出す
        let session = store
            .load_session(session_str.to_string()) // Result<Option<>>を返す
            .await
            .map_err(|_| redirect())?; // 非同期処理によるResultのハンドリング
        let session = session.ok_or(redirect())?;

        let context = UserContext {
            user_id: session.get::<i32>(AXUM_SESSION_USER_ID_KEY).unwrap(),
        };
        Ok(context)
    }
}
