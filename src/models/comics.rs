#[derive(Queryable, Serialize, Deserialize)]
pub struct Comic {
    pub id: i32,
    pub title: String,
    pub issuenumber: Option<f64>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ComicList(pub Vec<Comic>);

impl ComicList {
    pub fn list() -> Self {
        use crate::db_connection::establish_connection;
        use crate::schema::comics::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let connection = establish_connection();

        let result = comics
            //.limit(10)
            .load::<Comic>(&connection)
            .expect("Error loading comics");

        ComicList(result)
    }
}
