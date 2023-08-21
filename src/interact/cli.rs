use crate::{
    config::constants::URL,
    network::send::send_request,
    utils::helpers::{parse_json, parse_quality_link},
};
use json::JsonValue;

pub async fn search_anime(anime_name: String) -> Option<JsonValue> {
    let list = send_request(
        URL.to_string(),
        format!("/info/?query={}&limit=20", anime_name),
    )
    .await;

    return parse_json(list);
}

pub fn show_list_of_animes(animes: Option<JsonValue>, reverse: bool) -> Option<Vec<String>> {
    if let None = animes {
        return None;
    }
    let animes = animes.unwrap();
    let mut slugs: Vec<String> = Vec::new();
    for index in 0..animes.len() {
        let title = &animes[index]["title"]["english"]
            .as_str()
            .unwrap_or_default();
        let episodes = &animes[index]["episodeNum"].as_i64().unwrap_or_default();
        slugs.push(
            animes[index]["slug"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        );
        println!("{}. {} (episodes: {})", index + 1, title, episodes);
    }
    return Some(slugs);
}

pub async fn get_episodes(slug: String) -> Option<JsonValue> {
    let episodes = send_request(URL.to_string(), format!("/episodes/?id={}&dub=false", slug)).await;
    return parse_json(episodes);
}

// pub fn show_list_episodes(episodes:Option<JsonValue>)
pub async fn get_server_path(slug: String, episode: usize) -> Option<String> {
    let server = send_request(
        URL.to_string(),
        format!("/watch/{}-episode-{}?server=", slug, episode),
    )
    .await;

    if let None = server {
        return None;
    }
    let server = server.unwrap();
    let server = parse_json(Some(server));
    if let None = server {
        return None;
    }
    let server = server.unwrap()["source"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    println!("{}", server);
    let server_path_index = server.rfind("/");
    if let None = server_path_index {
        return None;
    }
    let path = &server[server_path_index.unwrap()..];
    return Some(path.to_string());
}

pub async fn get_quality(server_path: String) -> Option<(u64, String, String)> {
    let res = send_request(URL.to_string(), server_path).await;
    if let None = res {
        return None;
    }
    let java_res = res.unwrap();
    let regex = regex::Regex::new(r"const source = '(.*)'").unwrap();
    if let Some(matches) = regex.captures(&java_res) {
        let source = matches.get(1);
        if let Some(data) = source {
            //
            let source = data.as_str().to_string();
            //in
            println!("{}", source);
            let server_path_index = source.rfind("/");
            if let None = server_path_index {
                return None;
            }
            let path = &source.clone()[..server_path_index.unwrap()];
            //out
            let quality_res = send_request(source, "".to_string()).await;
            if let None = quality_res {
                return None;
            }
            // println!("I am fine here");
            return parse_quality_link(quality_res.unwrap(), path.to_string());
            //
        } else {
            return None;
        }
    } else {
        return None;
    }
}
