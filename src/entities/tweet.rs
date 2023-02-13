use chrono::{DateTime, Utc};

// Tweetエンティティ（DBのカラム、データ型と一致させる）
pub struct Tweet {
    #[allow(dead_code)]
    id: Option<i32>,
    pub message: String,
    pub posted_at: DateTime<Utc>,
    deleted: bool,
}

impl Tweet {
    // DBから取得したRowをTweetエンティティに変換するときに呼ぶ
    pub fn new(id: i32, message: String, posted_at: DateTime<Utc>) -> Tweet {
        Tweet {
            id: Some(id),
            message,
            posted_at,
            deleted: false,
        }
    }

    // /tweets/newへのPOSTリクエストで新規作成時に呼ぶ
    pub fn create(message: &str) -> Tweet {
        Tweet {
            id: None,
            message: message.into(),
            posted_at: Utc::now(),
            deleted: false,
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    pub fn delete(&mut self) {
        self.deleted = true;
    }
}
