// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub artistid: String,
    pub name: String,
    pub first_letter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub albumid: String,
    pub artistid: String,
    pub name: String,
    pub first_letter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub rusicid: String,
    pub albumid: String,
    pub title: String,
    pub imgurl: String,
    pub playpath: String,
    pub fullpath: String,
    pub extension: String,
    pub idx: String,
    pub page: String,
    pub fsizeresults: String,
    pub first_letter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumImage {
    pub albumid: String,
    pub width: String,
    pub height: String,
    pub filesize: String,
    pub fullpath: String,
    pub thumbpath: String,
    pub idx: String,
    pub page: String,
    pub httpthumbpath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayList {
    pub rusicid: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub artistcount: i64,
    pub albumcount: i64,
    pub songcount: i64,
    pub imagecount: i64,
}
