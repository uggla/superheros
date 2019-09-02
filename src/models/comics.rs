use crate::db_connection::establish_connection;
use crate::db_connection::DbExecutor;
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
pub struct ComicsList;

impl Message for ComicsList {
    type Result = io::Result<ComicsListMsgs>;
}

#[derive(Serialize, Deserialize)]
pub struct ComicsListMsgs {
    pub status: i32,
    pub message: String,
    pub comics_list: Vec<Comics>,
}

impl Handler<ComicsList> for DbExecutor {
    type Result = io::Result<ComicsListMsgs>;

    fn handle(&mut self, _comics_list: ComicsList, _: &mut Self::Context) -> Self::Result {
        use crate::schema::comics::dsl::*;
        let conn = &self.0.get().expect("Could not get a Db executor");

        let result = comics
            //.limit(10)
            .load::<Comics>(conn)
            .expect("Error loading comics");

        Ok(ComicsListMsgs {
            status: 200,
            message: "comics_list result.".to_string(),
            comics_list: result,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComicsId {
    pub comics_id: i32,
}

impl Message for ComicsId {
    type Result = io::Result<ComicsIdMsgs>;
}

#[derive(Serialize, Deserialize)]
pub struct ComicsIdMsgs {
    pub status: i32,
    pub message: String,
    pub comics_id: Comics,
}

impl Handler<ComicsId> for DbExecutor {
    type Result = io::Result<ComicsIdMsgs>;

    fn handle(&mut self, comics: ComicsId, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().expect("Could not get a Db executor");

        let result = comics::table.find(comics.comics_id).first(conn);
        match result {
            Ok(result) => Ok(ComicsIdMsgs {
                status: 200,
                message: "comics_id result.".to_string(),
                comics_id: result,
            }),
            Err(e) => Err(io::Error::new(io::ErrorKind::NotFound, e)),
        }
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
pub struct CharactersList;

impl Message for CharactersList {
    type Result = io::Result<CharactersListMsgs>;
}

#[derive(Serialize, Deserialize)]
pub struct CharactersListMsgs {
    pub status: i32,
    pub message: String,
    pub characters_list: Vec<Characters>,
}

//impl CharactersList {
//    pub fn list() -> Self {
//        let connection = establish_connection();

//        let result = characters::table
//            //.limit(10)
//            .load::<Characters>(&connection)
//            .expect("Error loading comics");

//        CharactersList(result)
//    }
//}
impl Handler<CharactersList> for DbExecutor {
    type Result = io::Result<CharactersListMsgs>;

    fn handle(&mut self, _characters_list: CharactersList, _: &mut Self::Context) -> Self::Result {
        use crate::schema::characters::dsl::*;
        let conn = &self.0.get().expect("Could not get a Db executor");

        let result = characters
            //.limit(10)
            .load::<Characters>(conn)
            .expect("Error loading characters");

        Ok(CharactersListMsgs {
            status: 200,
            message: "characters_list result.".to_string(),
            characters_list: result,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharactersJoinedToCharactersStats;

#[derive(Serialize, Deserialize)]
pub struct CharactersJoinedToCharactersStatsResult {
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

impl CharactersJoinedToCharactersStatsResult {
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
    ) -> CharactersJoinedToCharactersStatsResult {
        CharactersJoinedToCharactersStatsResult {
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

impl Message for CharactersJoinedToCharactersStats {
    type Result = io::Result<CharactersJoinedToCharactersStatsMsgs>;
}

#[derive(Serialize, Deserialize)]
pub struct CharactersJoinedToCharactersStatsMsgs {
    pub status: i32,
    pub message: String,
    pub characters_list: Vec<CharactersJoinedToCharactersStatsResult>,
}

impl Handler<CharactersJoinedToCharactersStats> for DbExecutor {
    type Result = io::Result<CharactersJoinedToCharactersStatsMsgs>;

    fn handle(
        &mut self,
        _characters_list: CharactersJoinedToCharactersStats,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = &self.0.get().expect("Could not get a Db executor");

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
            .load(conn);
        for (name, alignment, intelligence, strengh, speed, durability, power, combat, total) in
            data.unwrap()
        {
            result.push(CharactersJoinedToCharactersStatsResult::new(
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

        Ok(CharactersJoinedToCharactersStatsMsgs {
            status: 200,
            message: "CharactersJoinedToCharactersStats result.".to_string(),
            characters_list: result,
        })
    }
}

// impl Characters {
//     pub fn find() -> Result<Vec<CharactersJoinedToCharactersStats>, diesel::result::Error> {
//         let connection = establish_connection();
//         let mut result = vec![];
//         let data = characters::table
//             .inner_join(characters_stats::table.on(characters_stats::name.eq(characters::name)))
//             .select((
//                 characters::name,
//                 characters_stats::alignment,
//                 characters_stats::intelligence,
//                 characters_stats::strengh,
//                 characters_stats::speed,
//                 characters_stats::durability,
//                 characters_stats::power,
//                 characters_stats::combat,
//                 characters_stats::total,
//             ))
//             .load(&connection);
//         for (name, alignment, intelligence, strengh, speed, durability, power, combat, total) in
//             data?
//         {
//             result.push(CharactersJoinedToCharactersStats::new(
//                 name,
//                 alignment,
//                 intelligence,
//                 strengh,
//                 speed,
//                 durability,
//                 power,
//                 combat,
//                 total,
//             ));
//         }
//         Ok(result)
//     }
// }

#[derive(Queryable, Serialize, Deserialize)]
pub struct CharactersToComics {
    pub id: i32,
    pub comic_id: i32,
    pub characters_id: i32,
}

impl CharactersToComics {
    pub fn find() -> Result<Vec<(i32, i32)>, diesel::result::Error> {
        let connection = establish_connection();
        characters_to_comics::table
            .inner_join(comics::table.on(comics::id.eq(characters_to_comics::id)))
            .select((comics::id, characters_to_comics::id))
            .load(&connection)
    }
}
