use candid::{CandidType, Principal};
use serde::Deserialize;
use std::collections::HashMap;
use std::slice::SliceIndex;


#[derive(Default, CandidType, Deserialize)]
pub struct PostStore {
    posts: HashMap<u64, Post>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Post {
    id: u64,
    upvotes: u64,
    voted_by: Vec<Option<String>>,
    body: String,
    owner_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct PostQuery {
    pub sort: PostSort,
    pub page: u64,
    pub page_size: u64,
}


#[derive(CandidType, Deserialize)]
pub enum PostSort {
    Top,
    New,
}

impl PostStore {
    pub fn create_post(&mut self, id: u64, body: String, owner_id: Principal, voted_by: Option<Vec<Option<String>>>) -> Post {
        let post = Post::new(id, body, owner_id, voted_by);
        self.posts.insert(id, post.clone());
        return post;
    }

    pub fn list(&mut self, PostQuery { sort, page, page_size }: PostQuery) -> Vec<Post> {
        let mut posts: Vec<Post> = self.posts.values().cloned().collect::<Vec<Post>>();
    

        posts.sort_by(|a, b| {
            match sort {
                PostSort::Top => b.upvotes.cmp(&a.upvotes),
                PostSort::New => a.id.cmp(&b.id),
            }
        });

        return posts
    }

    pub fn posts_by_user(&mut self, user_id: Principal) -> Vec<Post> {
        let posts = self.posts.values().cloned().collect::<Vec<Post>>();
        return posts.into_iter().filter(|p| p.owner_id == user_id).collect::<Vec<Post>>();
    }

    pub fn upvote_post(&mut self, id: u64) -> Option<Post> {
        let post = self.posts.get(&id).cloned();
        if let Some(mut post) = post {
            if post.voted_by.contains(&Some(ic_cdk::caller().to_string())) {
                // inform!(post, "You already voted for this post.");
                return None
            }
            post.upvotes += 1;
            post.voted_by.push(Some(ic_cdk::caller().to_string()));
            return Some(post);
        }
        // inform!(None, "No post with id {} found.", id);  
        return None
    }
}

impl Post {
    pub fn new(id: u64, body: String, owner_id: Principal, _voted_by:Option<Vec<Option<String>>>) -> Post {
        return Post {
            id,
            body,
            voted_by: vec![],   
            owner_id,
            upvotes: 0,
        };
    }
}
