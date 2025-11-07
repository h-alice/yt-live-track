use diesel::prelude::*;
use crate::schema::tcc_live;

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = tcc_live)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LiveEntry {
    pub id: i32,
    pub title: String,
    pub live_id: String,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = tcc_live)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LiveInfo {
    pub title: String,
    pub live_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = tcc_live)]
pub struct NewLiveEntry<'a> {
    pub title: &'a str,
    pub live_id: &'a str,
}