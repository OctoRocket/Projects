use reqwest;
use serde;

#[derive(serde::Deserialize)]
struct PlayerCount {
    players: u32,
}

pub fn get_player_count(link: &String) -> Result<u32, reqwest::Error> {
    let body = reqwest::blocking::get(link).unwrap().json::<PlayerCount>().unwrap();
    
    Ok(body.players)
}
