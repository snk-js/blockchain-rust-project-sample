use candid::{CandidType, Principal};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default, CandidType, Deserialize)]
pub struct UserStore {
    users: HashMap<Principal, User>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    id: Principal,
    username: String,
}

impl UserStore {
    pub fn create_user(&mut self, id: Principal, username: String) -> User {
        let user = User::new(id, username);
        self.users.insert(id, user.clone());
        return user;
    }

    pub fn list(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }
}

impl User {
    pub fn new(id: Principal, username: String) -> User {
        return User { id, username };
    }
}
