use super::Playlist;

pub struct User{
    pub csrf_token: String,
    pub name: String,
    pub notification_count: i32,
    pub subscriptions: Vec<String>,
}
impl User {
    pub fn get_playlists(&self) -> Vec<Playlist::Playlist> {
        todo!()
    }
}