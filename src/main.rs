#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::{DirEntry, File};
use std::io::Read;
use audiotags::{Tag, Picture, MimeType, Album};
use reqwest::blocking::Client;
use yt_dlp::Youtube;
mod Song;

fn fetchUrl(url: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    eprintln!("Fetching {url:?}...");

    let client = Client::builder().build()?;

    let mut res = client.get(url).send()?;

    let mut buffer = Vec::new();
    res.copy_to(&mut buffer).expect("unable to copy contents");

    Ok(buffer)
}

fn test()  -> std::io::Result<()> {
    let url = "https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=2069&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D";
    let res = fetchUrl(String::from(url)).expect("Could'nt fetch url");
    std::fs::write("image.png", res).expect("Couldn't write to file");
    
    let path = std::env::current_dir()?;
    println!("Current dir: {}", path.display());
    let mut tag = Tag::new().read_from_path("LOST IN THE ECHO.mp3").unwrap();
    
    tag.set_title("foo fighters");
    tag.set_album_title("What?! 21 cars?!");

    let mut file = File::open("image.png").expect("Failed to open image");
    let mut image_data = Vec::new();
    file.read_to_end(&mut image_data).expect("Failed to read image data");

    println!("{}", image_data.len());
    let cover = Picture {
        mime_type: MimeType::Png,
        data: &image_data,
    };

    tag.set_album_cover(cover);

    tag.write_to_path("LOST IN THE ECHO.mp3").expect("Fail to save");
    Ok(())
}

fn parseName(line: String) -> Option<String> {
    if line.len() == 0 || line.chars().nth(0).unwrap() != '[' {
        return None;
    }
    let mut i: usize = 0;
    while line.chars().nth(i).unwrap() != ']' && i < line.len() {
        i += 1;
    }

    return Some(
        line.chars()
            .into_iter()
            .skip(1)
            .take(i - 1)
            .collect()
        )
}

fn parseThumbnail(line: String) -> Option<String> {
    if line.len() == 0 || !line.starts_with("## [") {
        return None;
    }

    let mut i: usize = 0;
    while line.chars().nth(i).unwrap() != ']' && i < line.len() {
        i += 1;
    }

    while line.chars().nth(i).unwrap() != '(' && i < line.len() {
        i += 1;
    }

    let mut j: usize = i;
    while line.chars().nth(j).unwrap() != ')' && j < line.len() {
        j += 1;
    }

    println!("{} | {}", line.len(), j);
    return Some(
        line.chars()
        .into_iter()
        .skip(i + 1)
        .take(j - i - 1)
        .collect()
    )
}

fn parseAlbum(line: String) -> Option<String> {
    if line.len() == 0 || !line.starts_with("## ") {
        return None;
    }

    let mut i: usize = 4;
    while line.chars().nth(i).unwrap() != ']' && i < line.len() {
        i += 1;
    }

    return Some(
        line.chars()
        .skip(4)
        .take(i - 4)
        .collect()
    )
}

fn parseLink(line: String) -> Option<String> {
    if line.len() == 0 || !line.starts_with("[") {
        return None;
    }

    let mut i: usize = 0;
    while line.chars().nth(i).unwrap() != ']' && i < line.len() {
        i += 1;
    }
    while line.chars().nth(i).unwrap() != '(' && i < line.len() {
        i += 1;
    }

    let mut j: usize = i;
    while line.chars().nth(j).unwrap() != ')' && j < line.len() {
        j += 1;
    }

    return Some(
        line.chars()
        .skip(i + 1)
        .take(j - i - 1)
        .collect()
    )
}

fn parseArtist(line: String) -> Option<String> {
    if line.len() == 0 || !line.starts_with("# ") {
        return None;
    }

    return Some(
        line.chars()
        .skip(2)
        .take(line.len() - 2)
        .collect()
    )
}

fn loadDownloaded(path: Vec<DirEntry>) -> Vec<String> {
    let mut songs: Vec<String> = vec![];
    for entry in path {
        songs.push(entry.file_name().into_string().unwrap_or(String::from("")));
    }
    return songs;
}

fn loadLibrary(mut file: std::fs::File) -> std::io::Result<Vec<Song::Album>> {
    let albums: Vec<Song::Album> = vec![];
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split('\n').collect();

    for line in lines {
        
    }
    
    return Ok(albums);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = std::fs::read_dir(args[1].clone())?
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    let songs = loadDownloaded(path);
    for i in 0..songs.len() {
        println!("{i}: {}", songs[i]);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testParseLink() {
        let s = String::from("[Battery](https://youtu.be/vA1nlwTbCvg?si=_RvX49cOHer9jrSt)");
        let parsed = parseLink(s);
        println!("{}", parsed.clone().unwrap());
        assert_eq!(parsed, Some(String::from("https://youtu.be/vA1nlwTbCvg?si=_RvX49cOHer9jrSt")));
    }

    #[test]
    fn testParseName() {
        let s = String::from("[Battery](https://youtu.be/vA1nlwTbCvg?si=_RvX49cOHer9jrSt)");
        let parsed = parseName(s);
        println!("{}", parsed.clone().unwrap());
        assert_eq!(parsed, Some(String::from("Battery")));
    }

    #[test]
    fn testParseThumbnail() {
        let s = String::from("## [Master of Puppets](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fv13.net%2Fwp-content%2Fuploads%2Fmetallica_-_master_of_puppets.jpg&f=1&nofb=1&ipt=104229cca003b142a34f1f135ba6e3eef8430307945a1074f59334ae39f0086f&ipo=images)");
        let parsed = parseThumbnail(s);
        println!("{}", parsed.clone().unwrap());
        assert_eq!(parsed, Some(String::from("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fv13.net%2Fwp-content%2Fuploads%2Fmetallica_-_master_of_puppets.jpg&f=1&nofb=1&ipt=104229cca003b142a34f1f135ba6e3eef8430307945a1074f59334ae39f0086f&ipo=images")));
    }

    #[test]
    fn testParseAlbum() {
        let s = String::from("## [Master of Puppets](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fv13.net%2Fwp-content%2Fuploads%2Fmetallica_-_master_of_puppets.jpg&f=1&nofb=1&ipt=104229cca003b142a34f1f135ba6e3eef8430307945a1074f59334ae39f0086f&ipo=images)");
        let parsed = parseAlbum(s);
        println!("{}", parsed.clone().unwrap());
        assert_eq!(parsed, Some(String::from("Master of Puppets")));
    }

    #[test]
    fn testParseArtist() {
        let s = String::from("# Metallica");
        let parsed = parseArtist(s);
        println!("{}", parsed.clone().unwrap());
        assert_eq!(parsed, Some(String::from("Metallica")));
    }
}
