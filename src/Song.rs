#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::mem;
use reqwest::blocking::Client;

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    InBoth,
    InLibrary,
    Downloaded,
}

#[derive(Clone, PartialEq)]
pub struct Song {
    pub status: Status,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub URL: String,
    pub year: i64,
    pub trackNumber: usize,
    pub imageUrl: String,
}

fn toUri(str: String) -> String {
    return str.replace(" ", "%20");
}

fn fetchUrl(url: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    eprintln!("Fetching {url:?}...");

    let client = Client::builder()
        .user_agent("Music Downloaded <0.1>")
        .build()?;

    let res = client.get(url)
        .send()?
        .json::<serde_json::Value>()?;

    Ok(res)
}

impl Song {
    fn download(&mut self, _path: String) {

    }

    fn new() -> Song {
        return Song{status: Status::InBoth,
            name: String::from(""),
            artist: String::from(""),
            album: String::from(""),
            URL: String::from(""),
            imageUrl: String::from(""),
            year: 0,
            trackNumber: 0
        };
    }

    fn getMetadata(&mut self) {
        let url = format!("https://musicbrainz.org/ws/2/recording?query=title:{}%20AND%20artist:{}%20AND%20release:{}&fmt=json", toUri(self.name.clone()), toUri(self.artist.clone()), toUri(self.album.clone()));
        let res = fetchUrl(url).expect("Couldn't fetch URL");

        let recordings = res.get("recordings").and_then(|recording| recording.as_array()).unwrap();
        let mut minYear = std::i64::MAX;
        for recording in recordings {
            if recording["score"].as_i64() > Some(85) {
                let yearRaw = recording["first-release-date"].as_str().unwrap_or_else(|| "0000");
                let year = yearRaw[0..4].parse::<i64>().expect("Couldn't convert to i64");
                if year > 0 {
                    if year < minYear && minYear > 0 {
                        minYear = year as i64;
                        if self.trackNumber == 0 {
                            if let Some(offset) = recording["releases"][0]["media"][0]["track-offset"].as_i64() {
                                self.trackNumber = offset as usize + 1;
                            }
                        }
                    }
                }
            }
        }
        println!("{}", self.trackNumber);
        self.year = minYear;
    }
}

fn combineSongs(library: &Vec<Song>, downloaded: &Vec<Song>) -> Vec<Song> {
     let mut songs: Vec<Song> = vec![];

     for i in 0..library.len() {
        let mut song = library[i].clone();
        song.status = Status::InLibrary;
        songs.push(song);
     }

     for i in 0..downloaded.len() {
        let mut found = songs.iter_mut().filter(|s| s.name == downloaded[i].name).peekable();
        if found.peek().is_some() {
            found.next().unwrap().status = Status::InBoth;
        }
        else {
            let mut song = downloaded[i].clone();
            song.status = Status::Downloaded;
            songs.push(song);
        }
     }

     return songs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testCombineSongs() {
        // let mut lib: Vec<Song> = vec![];
        // let mut download: Vec<Song> = vec![];
        // let s = Song {status: Status::Downloaded, name: String::from("value"), URL: String::from("")};
        // download.push(s);
        // let s = Song {status: Status::InLibrary, name: String::from("vlue"), URL: String::from("")};
        // lib.push(s);

        // let songs = combineSongs(&lib, &download);
        // assert!(songs.len() != 0);
    }

    #[test]
    fn testConvertToUri() {
        let mut songs: Vec<Song> = vec![Song::new(); 8];
        
        songs[0].name = String::from("Battery");
        songs[1].name = String::from("Master of Puppets");
        songs[2].name = String::from("The Thing That Should Not Be");
        songs[3].name = String::from("Welcome Home (Sanitarium)");
        songs[4].name = String::from("Disposable Heroes");
        songs[5].name = String::from("Leper Messiah");
        songs[6].name = String::from("Orion");
        songs[7].name = String::from("Damage, Inc.");

        for i in 0..songs.len() {
            songs[i].artist = String::from("Metallica");
            songs[i].album = String::from("Master of Puppets");
            songs[i].trackNumber = i + 1;
        }

        let songsBefore = songs.clone();

        for i in 0..songs.len() {
            songs[i].getMetadata();
            println!("{}", songs[i].name);
            assert_eq!(songs[i].trackNumber, songsBefore[i].trackNumber)
        }
    }
}
