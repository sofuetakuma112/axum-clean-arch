use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use std::time::Duration;

use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};
use crate::entities::Account;
use crate::repositories::Accounts;

pub async fn create_account(repo: &impl Accounts, email: &str, password: &str, display_name: &str) {
    let new_account = Account::create(email, password, display_name);
    repo.store(&new_account).await;
}

/// 引数のemailを使ってDBからアカウントを取得して、引数のpasswordと照合し一致した場合、セッションをDBに格納してクッキー文字列を返す
pub async fn create_session(
    repo: &impl Accounts,
    email: &str,
    password: &str,
) -> Option<SessionToken> {
    let account = repo.find_by(email).await;
    if let Some(account) = account {
        if !account.matches_password(password) {
            return None;
        }

        let database_url = database_url();
        let store = PostgresSessionStore::new(&database_url).await.unwrap(); // セッションストアの作成

        // セッションを作成する
        let mut session = Session::new();
        session
            .insert(AXUM_SESSION_USER_ID_KEY, account.id().unwrap())
            .unwrap();
        session.expire_in(Duration::from_secs(604800));

        let cookie = store.store_session(session).await.unwrap().unwrap(); // セッションストアにセッションを保存する

        Some(SessionToken::new(&cookie)) // SessionToken構造体に詰め込んで返す
    } else {
        None
    }
}

pub fn clear_session() -> SessionToken {
    SessionToken::clear()
}

pub struct SessionToken {
    token: String,
    max_age: usize,
}

impl SessionToken {
    pub fn new(token: &str) -> SessionToken {
        SessionToken {
            token: token.to_string(),
            max_age: 604800,
        }
    }

    pub fn clear() -> SessionToken {
        SessionToken {
            token: "deleted".to_string(),
            max_age: 0,
        }
    }
}

impl SessionToken {
    pub fn cookie(&self) -> String {
        format!(
            "{}={}; Max-Age={}; Path=/; HttpOnly",
            AXUM_SESSION_COOKIE_NAME, self.token, self.max_age
        )
    }
}
