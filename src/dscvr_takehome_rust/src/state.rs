use crate::post::PostStore;
use crate::user::UserStore;
use candid::CandidType;
use serde::Deserialize;
use std::cell::RefCell;

#[derive(Default, CandidType, Deserialize)]
pub struct State {
    // NOTE: When adding new persistent fields here, ensure that these fields
    // are being persisted in the `replace` method below.
    pub post_store: PostStore,
    pub user_store: UserStore,
    pub global_index: u64,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
