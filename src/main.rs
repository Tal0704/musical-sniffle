#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
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

fn main()  -> std::io::Result<()> {
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
