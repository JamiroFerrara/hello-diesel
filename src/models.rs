// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
#[derive(Queryable, Debug)]
pub struct Artist {
    pub id: i32,
    pub artistName: String,
    pub name: String,
    pub surname: String,
}

#[derive(Queryable, Debug)]
pub struct Genre {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub waveData: Option<String>,
    pub artworkUrl: String,
    pub bannerUrl: String,
    pub price: f64,
    pub genreId: Option<String>,
    pub mp3: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct TracksOnArtist {
    pub id: String,
    pub createdAt: NaiveDateTime,
    pub trackId: String,
    pub artistId: i32,
}

#[derive(Queryable, Debug)]
pub struct TracksOnVinyl {
    pub id: String,
    pub createdAt: NaiveDateTime,
    pub trackId: String,
    pub vinylId: String,
}

#[derive(Queryable, Debug)]
pub struct Vinyl {
    pub id: String,
    pub createdAt: NaiveDateTime,
    pub title: String,
    pub artistId: i32,
    pub description: Option<String>,
    pub url: String,
    pub artworkUrl: String,
    pub bannerUrl: Option<String>,
    pub price: i32,
    pub stock: i32,
}

