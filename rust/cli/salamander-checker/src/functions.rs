use serde::Deserialize;
use reqwest::blocking::get;

#[derive(Deserialize)]
struct PlayerCount {
    players: u32,
}

pub fn get_player_count(link: &String) -> Result<u32, reqwest::Error> {
    let body = get(link).unwrap().json::<PlayerCount>().unwrap();
    
    Ok(body.players)
}
