use candid::{CandidType, Principal};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default, CandidType, Deserialize)]
pub struct PostStore {
    posts: HashMap<u64, Post>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Post {
    id: u64,
    upvotes: u64,
    body: String,
    owner_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct PostQuery {
    sort: PostSort,
    page: u64,
    page_size: u64,
}


#[derive(CandidType, Deserialize)]
pub enum PostSort {
    Top,
    New,
}

impl PostStore {
    pub fn create_post(&mut self, id: u64, body: String, owner_id: Principal) -> Post {
        let post = Post::new(id, body, owner_id);
        self.posts.insert(id, post.clone());
        return post;
    }

    pub fn list(&mut self, /* query: PostQuery */) -> Vec<Post> {
        let posts = self.posts.values().cloned().collect::<Vec<Post>>();
        // posts.sort_by(|a, b| {
        //     match query.sort {
        //         PostSort::Top => b.upvotes.cmp(&a.upvotes),
        //         PostSort::New => a.id.cmp(&b.id),
        //     }
        // });
        return posts;   
    }

    pub fn upvote_post(&mut self, id: u64) -> Option<Post> {
        let post = self.posts.get(&id).cloned();
        if let Some(mut post) = post {
            post.upvotes += 1;
            self.posts.insert(id, post.clone());
            return Some(post);
        }
        return None;
    }
}

impl Post {
    pub fn new(id: u64, body: String, owner_id: Principal) -> Post {
        return Post {
            id,
            body,
            owner_id,
            upvotes: 0,
        };
    }
}
