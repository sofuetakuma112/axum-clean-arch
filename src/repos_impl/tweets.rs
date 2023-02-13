use crate::database::ConnectionPool;
use crate::entities::Tweet;
use crate::repositories::Tweets;
use tokio_postgres::Row;

pub struct TweetsImpl<'a> {
    pub pool: &'a ConnectionPool,
}

// レポジトリ（定義）の実装
#[axum::async_trait]
impl<'a> Tweets for TweetsImpl<'a> {
    async fn find(&self, id: i32) -> Option<Tweet> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM tweets WHERE id = $1", &[&id])
            .await
            .unwrap();
        row.map(|r| r.into())
    }

    // Tweetエンティティのベクターを返す
    async fn list(&self) -> Vec<Tweet> {
        let conn = self.pool.get().await.unwrap();
        let rows = conn
            .query("SELECT * FROM tweets ORDER BY posted_at DESC", &[])
            .await
            .unwrap();
        // into_iter()はベクタの所有権を消費する
        rows.into_iter().map(|r| r.into()).collect() // TweetエンティティにFrom<Row>を実装している必要がある
    }

    async fn store(&self, entity: &Tweet) {
        let conn = self.pool.get().await.unwrap();
        if let Some(id) = entity.id() {
            if entity.is_deleted() {
                conn.execute("DELETE FROM tweets WHERE id = $1", &[&id])
                    .await
                    .ok();
            }
        } else {
            // 新規作成
            conn.execute(
                "INSERT INTO tweets (message, posted_at, posted_by) VALUES ($1, $2, $3)",
                &[&entity.message, &entity.posted_at, &entity.posted_by],
            )
            .await
            .ok();
        }
    }
}

impl From<Row> for Tweet {
    fn from(r: Row) -> Self {
        Tweet::new(
            r.get("id"),
            r.get("message"),
            r.get("posted_at"),
            r.get("posted_by"),
        )
    }
}
