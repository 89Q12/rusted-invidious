use crate::config::Preferences;

use super::piped::Playlist;

#[derive(Clone)]
pub struct User{
    pub csrf_token: String,
    pub name: String,
    pub notification_count: i32,
    pub subscriptions: Vec<String>,
    pub preferences: Preferences,
}
impl User {
    pub fn get_playlists(&self) -> Vec<Playlist> {
        todo!()
    }
}