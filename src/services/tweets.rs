use crate::entities::Tweet;
use crate::repositories::Tweets;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        // TweetエンティティからTweetビューにFromトレイトの実装を利用して型変換する
        tweets: tweets.into_iter().map(|x| x.into()).collect(), // ここで .into() を呼び出し、エンティティをビューに変換している
    }
}

pub async fn create_tweet(repo: &impl Tweets, message: &str) {
    let new_tweet = Tweet::create(message); // Tweetエンティティの作成
    repo.store(&new_tweet).await; // DBに保存する
}

pub async fn delete_tweet(repo: &impl Tweets, id: i32) {
    let tweet = repo.find(id).await;
    if let Some(mut tweet) = tweet {
        tweet.delete(); // deletedフラグをtrueにする
        repo.store(&tweet).await;
    }
}
