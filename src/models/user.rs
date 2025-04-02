pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub repeat_password: Option<String>,
    pub profile_pic: Option<String>,
    pub banner: Option<String>,
    pub events: Vec<NEvent>,
}