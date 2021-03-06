use super::schema::dialogs;
use super::schema::users;
use super::schema::users_subscriptions;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub created_at: String,
}

#[derive(Debug, Queryable, Default)]
pub struct Subscription {
    pub id: i32,
    pub user_id: String,
    pub subreddit: String,
    pub last_sent_at: Option<String>,
    pub send_on: i32,
    pub send_at: i32,
}

#[derive(Insertable)]
#[table_name = "users_subscriptions"]
pub struct NewSubscription<'a> {
    pub user_id: &'a str,
    pub subreddit: &'a str,
    pub send_on: i32,
    pub send_at: i32,
    pub last_sent_at: Option<String>,
}

#[derive(Debug, Queryable, Insertable, Clone, PartialEq)]
#[table_name = "dialogs"]
pub struct DialogEntity {
    pub user_id: String,
    pub command: String,
    pub step: String,
    pub data: String,
}
