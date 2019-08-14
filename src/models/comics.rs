use crate::db_connection::establish_connection;
use crate::schema::comics;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Comics {
    pub id: i32,
    pub title: String,
    pub issuenumber: Option<f64>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ComicsList(pub Vec<Comics>);

impl ComicsList {
    pub fn list() -> Self {
        use crate::schema::comics::dsl::*;
        let connection = establish_connection();

        let result = comics
            //.limit(10)
            .load::<Comics>(&connection)
            .expect("Error loading comics");

        ComicsList(result)
    }
}

impl Comics {
    pub fn find(id: &i32) -> Result<Comics, diesel::result::Error> {
        let connection = establish_connection();

        comics::table.find(id).first(&connection)
    }
}
