mod config;
mod interact;
mod network;
mod utils;
use crate::config::constants::DEST;
use interact::cli::{self, get_server_path, show_list_of_animes};
use network::downloder::run;
use std::io::Write;

#[tokio::main]
async fn main() {
    let anime_name = get_input("Enter the anime: ");
    let animes = cli::search_anime(anime_name).await;
    if let None = animes {
        return;
    }
    let slugs = show_list_of_animes(animes, false);
    if let None = slugs {
        return;
    }

    let anime: usize = get_input("Choose the anime: ")
        .trim()
        .parse()
        .unwrap_or_default();
    let slugs = slugs.unwrap();
    let slug = slugs.get(anime - 1).unwrap();

    let episode: usize = get_input("Choose the episode: ")
        .trim()
        .parse()
        .unwrap_or_default();
    // println!("{}", slug);
    let server_path = get_server_path(slug.clone(), episode).await.unwrap();
    // println!("server_path: {}", server_path);
    std::fs::create_dir(format!("{}{}", DEST, slug));
    let filename = format!("{}{}/{}-episode-{}.mp4", DEST, slug, slug, episode);
    run(server_path, filename, 40).await.unwrap();
}

fn get_input(msg: &str) -> String {
    let mut str = String::new();
    while str.len() < 2 {
        str.clear();
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut str).unwrap();
    } //while
    return str;
}
