// LeetCode/355 - Design Twitter

use std::collections::{BinaryHeap, HashMap, HashSet};

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
struct Twitter {
    users: Vec<User>,
    user_id_to_idx: HashMap<i32, usize>,
    time: usize,
}

struct User {
    follows: HashSet<i32>,
    tweets: Vec<(usize, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        return Twitter {
            users: vec![],
            user_id_to_idx: HashMap::new(),
            time: 0,
        };
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.create_user(user_id);
        let user = &mut self.users[self.user_id_to_idx[&user_id]];
        user.tweets.push((self.time, tweet_id));
        self.time += 1;
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        self.create_user(user_id);
        let user = &self.users[self.user_id_to_idx[&user_id]];
        let mut max_heap = BinaryHeap::new();
        for followee_id in &user.follows {
            let followee = &self.users[self.user_id_to_idx[&followee_id]];
            if followee.tweets.len() == 0 {
                continue;
            }
            let idx = followee.tweets.len() - 1;
            let (time, tweet_id) = followee.tweets[idx];
            max_heap.push((time, tweet_id, followee_id, idx));
        }
        let mut ret = vec![];
        while ret.len() < 10 && max_heap.len() > 0 {
            let (_time, tweet_id, followee_id, idx) = max_heap.pop().unwrap();
            ret.push(tweet_id);
            let followee = &self.users[self.user_id_to_idx[&followee_id]];
            if idx == 0 {
                continue;
            }
            let (time, tweet_id) = followee.tweets[idx - 1];
            max_heap.push((time, tweet_id, followee_id, idx - 1));
        }
        return ret;
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.create_user(follower_id);
        self.create_user(followee_id);
        let follower = &mut self.users[self.user_id_to_idx[&follower_id]];
        follower.follows.insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.create_user(follower_id);
        self.create_user(followee_id);
        let follower = &mut self.users[self.user_id_to_idx[&follower_id]];
        follower.follows.remove(&followee_id);
    }

    fn create_user(&mut self, user_id: i32) {
        if self.user_id_to_idx.contains_key(&user_id) {
            return;
        }
        self.user_id_to_idx.insert(user_id, self.users.len());
        let mut user = User::new();
        user.follows.insert(user_id);
        self.users.push(user);
    }
}

impl User {
    fn new() -> Self {
        return User {
            follows: HashSet::new(),
            tweets: vec![],
        };
    }
}
