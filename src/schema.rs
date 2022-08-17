table! {
    Artist (id) {
        id -> Integer,
        artistName -> Varchar,
        name -> Varchar,
        surname -> Varchar,
    }
}

table! {
    Genre (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    Track (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Varchar,
        url -> Varchar,
        waveData -> Nullable<Varchar>,
        artworkUrl -> Varchar,
        bannerUrl -> Varchar,
        price -> Decimal,
        genreId -> Nullable<Varchar>,
        mp3 -> Nullable<Varchar>,
    }
}

table! {
    TracksOnArtists (id) {
        id -> Varchar,
        createdAt -> Datetime,
        trackId -> Varchar,
        artistId -> Integer,
    }
}

table! {
    TracksOnVinyl (id) {
        id -> Varchar,
        createdAt -> Datetime,
        trackId -> Varchar,
        vinylId -> Varchar,
    }
}

table! {
    Vinyl (id) {
        id -> Varchar,
        createdAt -> Datetime,
        title -> Varchar,
        artistId -> Integer,
        description -> Nullable<Varchar>,
        url -> Varchar,
        artworkUrl -> Varchar,
        bannerUrl -> Nullable<Varchar>,
        price -> Integer,
        stock -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    Artist,
    Genre,
    Track,
    TracksOnArtists,
    TracksOnVinyl,
    Vinyl,
);
