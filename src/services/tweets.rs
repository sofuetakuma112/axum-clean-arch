// サービスでは、リポジトリの実装が提供するメソッドを使ってビューを構築する
use crate::entities::Tweet;
use crate::repositories::{Accounts, Tweets};
use crate::request::UserContext;
use crate::views::Home;
use std::collections::HashSet;

pub async fn list_tweets(repo: &impl Tweets, account_repo: &impl Accounts) -> Home {
    // ツイート一覧を取得する
    let tweets = repo.list().await; 
    // ツイート一覧にある posted_by を重複無しの一覧にする
    let posted_account_ids = tweets.iter().map(|x| x.posted_by).collect::<HashSet<i32>>();
    // posted_by のidでアカウント一覧を取得する
    let accounts = account_repo.find(posted_account_ids).await;

    let tweets = tweets
        .into_iter()
        .map(|x| {
            let account = accounts.get(&x.posted_by).unwrap();
            // .into() を呼び出し、Tweetビューに変換している
            // Fromトレイトの実装を利用している
            (x, account).into()
        })
        .collect();

    Home { tweets }
}

pub async fn create_tweet(repo: &impl Tweets, user_context: &UserContext, message: &str) {
    let new_tweet = Tweet::create(message, user_context.user_id); // Tweetエンティティの作成
    repo.store(&new_tweet).await; // DBに保存する
}

pub async fn delete_tweet(repo: &impl Tweets, id: i32) {
    let tweet = repo.find(id).await;
    if let Some(mut tweet) = tweet {
        tweet.delete(); // deletedフラグをtrueにする
        repo.store(&tweet).await;
    }
}
