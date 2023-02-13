use crate::constants::database_url;
use crate::repos_impl::{AccountsImpl, TweetsImpl};
use axum::extract::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> Extension<RepositoryProvider> {
    let manager = PostgresConnectionManager::new_from_stringlike(database_url(), NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    Extension(RepositoryProvider(pool)) // 全てのリクエストでコネクションプールを共有する
}

#[derive(Clone)] // スレッド間で共有できるようにするため？
pub struct RepositoryProvider(ConnectionPool); // ConnectionPoolは恐らくArcなのでCloneしてもポインタがコピーされるだけ？

impl RepositoryProvider {
    pub fn tweets(&self) -> TweetsImpl {
        TweetsImpl { pool: &self.0 }
    }

    pub fn accounts(&self) -> AccountsImpl {
        AccountsImpl { pool: &self.0 }
    }
}
