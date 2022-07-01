pub struct User{
    pub csrf_token: String,
    pub name: String,
    pub notification_count: i32,
}
impl User {
    pub fn get_playlists(&self) -> Vec<String> {
        todo!()
    }
}