use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub url_icon: String,
}

#[derive(Queryable)]
pub struct Message {
    pub id: String,
    pub id_server: String,
    pub id_channel: String,
    pub id_author: String,
    pub time: NaiveDateTime,
}
