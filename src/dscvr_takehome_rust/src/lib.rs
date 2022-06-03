use std::vec;

use crate::state::STATE;
use ic_cdk::storage;
use ic_cdk_macros::*;
use post::{Post, PostSort, PostQuery};
use state::State;
use user::User;
use candid::{Principal};

mod post;
mod state;
mod user;

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
pub fn create_user(username: String) -> User {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().user_store.create_user(caller, username))
}

#[query]
pub fn list_users() -> Vec<User> {
    STATE.with(|s| s.borrow().user_store.list())
}

#[update]
pub fn create_post(body: String) -> Post {
    return STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.global_index += 1;

        let idx = state.global_index.clone();
        return state.post_store.create_post(idx, body, ic_cdk::caller(), Some(vec![Some("string".to_string())]));
    });
}

#[update]
pub fn upvote_post(id: u64) -> Option<Post> {
    // let caller = ic_cdk::caller();
    return STATE.with(|s| s.borrow_mut().post_store.upvote_post(id));
}

#[query]
pub fn list_posts() -> Vec<Post> {
    return STATE.with(|s| s.borrow_mut().post_store.list( PostQuery { sort: PostSort::Top, page: 0, page_size: 0 }));
}
#[query]
pub fn list_posts_by_user(id: Principal) -> Vec<Post> {
    return STATE.with(|s| s.borrow_mut().post_store.posts_by_user(id));
}

#[init]
fn main() {}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| storage::stable_save((s,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (old_state,): (State,) = storage::stable_restore().unwrap();
    STATE.with(|s| *s.borrow_mut() = old_state);
}
