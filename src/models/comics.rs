use crate::db_connection::establish_connection;
use crate::db_connection::ConnDsl;
use crate::schema::characters;
use crate::schema::characters_stats;
use crate::schema::characters_to_comics;
use crate::schema::comics;
use actix::Handler;
use actix::Message;
use actix_web::*;
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use std::io;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Comics {
    pub id: i32,
    pub title: String,
    pub issuenumber: Option<f64>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
// pub struct ComicsList(pub Vec<Comics>);
pub struct ComicsList;

impl Message for ComicsList {
    // type Result = Result<ComicsListMsgs, Error>;
    type Result = io::Result<ComicsListMsgs>;
}

#[derive(Serialize, Deserialize)]
pub struct ComicsListMsgs {
    pub status: i32,
    pub message: String,
    pub comics_list: Vec<Comics>,
}

impl Handler<ComicsList> for ConnDsl {
    // type Result = Result<ComicsListMsgs, Error>;
    type Result = io::Result<ComicsListMsgs>;

    fn handle(&mut self, comics_list: ComicsList, _: &mut Self::Context) -> Self::Result {
        // let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let conn = &self.0.get().unwrap();
        use crate::schema::comics::dsl::*;

        let result = comics
            //.limit(10)
            .load::<Comics>(conn)
            .expect("Error loading comics");

        Ok(ComicsListMsgs {
            status: 200,
            message: "article_list result.".to_string(),
            comics_list: result,
        })
    }
}

//impl ComicsList {
//    pub fn list() -> Self {
//        use crate::schema::comics::dsl::*;
//        let connection = establish_connection();

//        let result = comics
//            //.limit(10)
//            .load::<Comics>(&connection)
//            .expect("Error loading comics");

//        ComicsList(result)
//    }
//}

impl Comics {
    pub fn find(id: &i32) -> Result<Comics, diesel::result::Error> {
        let connection = establish_connection();

        comics::table.find(id).first(&connection)
    }
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Characters {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct CharactersStats {
    pub id: i32,
    pub name: String,
    pub alignment: String,
    pub intelligence: i32,
    pub strengh: i32,
    pub durability: i32,
    pub power: i32,
    pub combat: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CharactersJoinedToCharactersStats {
    pub name: String,
    pub alignment: String,
    pub intelligence: i32,
    pub strengh: i32,
    pub speed: i32,
    pub durability: i32,
    pub power: i32,
    pub combat: i32,
    pub total: i32,
}

impl CharactersJoinedToCharactersStats {
    pub fn new(
        name: String,
        alignment: String,
        intelligence: i32,
        strengh: i32,
        speed: i32,
        durability: i32,
        power: i32,
        combat: i32,
        total: i32,
    ) -> CharactersJoinedToCharactersStats {
        CharactersJoinedToCharactersStats {
            name,
            alignment,
            intelligence,
            strengh,
            speed,
            durability,
            power,
            combat,
            total,
        }
    }
}

impl Characters {
    pub fn find() -> Result<Vec<CharactersJoinedToCharactersStats>, diesel::result::Error> {
        let connection = establish_connection();
        let mut result = vec![];
        let data = characters::table
            .inner_join(characters_stats::table.on(characters_stats::name.eq(characters::name)))
            .select((
                characters::name,
                characters_stats::alignment,
                characters_stats::intelligence,
                characters_stats::strengh,
                characters_stats::speed,
                characters_stats::durability,
                characters_stats::power,
                characters_stats::combat,
                characters_stats::total,
            ))
            .load(&connection);
        for (name, alignment, intelligence, strengh, speed, durability, power, combat, total) in
            data?
        {
            result.push(CharactersJoinedToCharactersStats::new(
                name,
                alignment,
                intelligence,
                strengh,
                speed,
                durability,
                power,
                combat,
                total,
            ));
        }
        Ok(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharactersList(pub Vec<Characters>);

impl CharactersList {
    pub fn list() -> Self {
        let connection = establish_connection();

        let result = characters::table
            //.limit(10)
            .load::<Characters>(&connection)
            .expect("Error loading comics");

        CharactersList(result)
    }
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct CharactersToComics {
    pub id: i32,
    pub comic_id: i32,
    pub characters_id: i32,
}

impl CharactersToComics {
    pub fn find() -> Result<Vec<(i32, i32)>, diesel::result::Error> {
        let connection = establish_connection();
        // let data = characters_to_comics::table
        //     .inner_join(comics::table.on(comics::id.eq(characters_to_comics::id)))
        //     .select((comics::id, characters_to_comics::id))
        //     .load(&connection);
        // return data;
        characters_to_comics::table
            .inner_join(comics::table.on(comics::id.eq(characters_to_comics::id)))
            .select((comics::id, characters_to_comics::id))
            .load(&connection)
    }
}
