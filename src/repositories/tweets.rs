use crate::entities::Tweet;

// レポジトリの定義（インターフェース）
#[axum::async_trait]
pub trait Tweets {
    async fn find(&self, id: i32) -> Option<Tweet>;
    async fn list(&self) -> Vec<Tweet>;
    async fn store(&self, entity: &Tweet);
}
