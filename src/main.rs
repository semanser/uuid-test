use async_graphql::*;
use sqlx::types::Uuid;

#[derive(SimpleObject)]
pub struct UserModel {
    pub id: Uuid
}

fn main() {
    println!("Hello, world!");
}
