use std::process::Command;

use crate::config::constants::PATH;
use crate::network::send::send_request;
use crate::{interact::cli::get_quality, utils::helpers::parse_part};
use anyhow::Ok;
use futures::stream::TryStreamExt;
use futures::StreamExt;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
type Result<Output> = anyhow::Result<Output>;

pub async fn run(server: String, destination: String, thread: usize) -> Result<()> {
    let quality = get_quality(server).await;
    if let None = quality {
        Ok(());
    }
    let (quality, url, base_url) = quality.unwrap();

    println!("Downloading in{}p quality...", quality);
    let res = send_request(url, "".to_string()).await;
    if let None = res {
        return Ok(());
    }
    let parts = parse_part(res.unwrap());
    if let None = parts {
        return Ok(());
        // return;
    }

    if let Err(err) = std::fs::create_dir(PATH) {
        println!("{}", err);
    }
    let parts = parts.unwrap();
    let tmp_writer = tokio::fs::File::create(format!("{}{}", PATH, "parts.txt")).await;
    if let Err(err) = tmp_writer {
        println!("{}", err);
        return Ok(());
        // return;
    }
    let mut tmp_writer = tmp_writer.unwrap();
    for each in parts.iter() {
        tmp_writer
            .write(format!("file {}\n", each).as_bytes())
            .await;
    }
    tmp_writer.flush().await;

    let client = reqwest::Client::new();

    futures::stream::iter(parts)
        .enumerate()
        .map(|(index, part)| {
            let client = client.clone();
            let part_url = format!("{}/{}", base_url, part);
            let part_file_name = format!("{}/{}", PATH, part);

            async move {
                let status =
                    tokio::spawn(
                        async move { download(index, client, part_url, part_file_name).await },
                    )
                    .await;
                return Ok(()) as Result<()>;
            }
        })
        .buffer_unordered(thread)
        .try_collect()
        .await?;
    // .await;
    merge_video(destination).await;
    return Ok(());
}

pub async fn download(index: usize, client: Client, url: String, name: String) {
    let res = client.get(url).send().await;
    if let Err(err) = res {
        println!("{}", err);
        return;
    }

    let res = res.unwrap();
    // if !res.status().is_success() {
    //     println!("Something went wrong in server");
    //     return;
    // }
    let part_path = format!("{}.part", name);

    if std::path::Path::new(&name).exists() {
        println!("Few parts of a file is already downlaoded");
        return;
    }
    let out = res.bytes().await;
    if let Err(err) = out {
        println!("{}", err);
        return;
    }
    let out = out.unwrap();

    let part = tokio::fs::File::create(&part_path).await;
    if let Err(err) = part {
        println!("{}", err);
        return;
    }
    let mut part = part.unwrap();
    if let Err(err) = part.write_all(&out).await {
        println!("{}", err);
        return;
    }

    if let Err(err) = tokio::fs::rename(part_path, name).await {
        println!("{}", err);
    }
    println!("part {} is downloaded", index);
    //
}

async fn merge_video(dest: String) {
    let ffmpeg = tokio::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-i")
        .arg(format!("{}parts.txt", PATH))
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg("-c")
        .arg("copy")
        .arg(dest)
        .current_dir(PATH)
        .spawn();
    if let Err(err) = ffmpeg {
        println!("{}", err);
        return;
    }

    let ffmpeg = ffmpeg.unwrap().wait().await;
    if let Err(err) = ffmpeg {
        println!("{}", err);
        return;
    }

    if let Err(err) = std::fs::remove_dir_all(PATH) {
        println!("{}", err);
    }
}
