use anyhow::{Result, anyhow};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use tokio::fs::File;
use tokio_util::codec::{Framed, LinesCodec};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct NginxLog {
    addr: String,
    datetime: String,
    method: String,
    url: String,
    protocol: String,
    status: String,
    body_bytes: String,
    referer: String,
    user_agent: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file = File::open("assets/nginx_logs.txt").await?;
    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    let mut framed = Framed::new(file, LinesCodec::new());

    let mut writer =
        csv_async::AsyncSerializer::from_writer(File::create("assets/nginx_logs.csv").await?);

    // 初始化进度条
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );
        
    while let Some(Ok(line)) = framed.next().await {
        let log = parse_nginx_log(&line)?;
        writer.serialize(log).await?;
        // 更新进度条
        pb.inc(line.len() as u64);
    }
    writer.flush().await?;
    pb.finish_with_message("Processing complete");

    Ok(())
}

fn parse_nginx_log(s: &str) -> Result<NginxLog> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
        r#"^(?<ip>\S+)\s+\S+\s+\S+\s+\[(?<date>[^\]]+)\]\s+"(?<method>\S+)\s+(?<url>\S+)\s+(?<proto>[^"]+)"\s+(?<status>\d+)\s+(?<bytes>\d+)\s+"(?<referer>[^"]+)"\s+"(?<ua>[^"]+)"$"#,
    ).expect("Failed to compile regex")
    });

    fn extract_field<'a>(cap: &'a regex::Captures, name: &str) -> Result<String> {
        cap.name(name)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow!("parse {} error", name))
    }

    let cap = RE.captures(s).ok_or_else(|| anyhow!("parse error"))?;

    let addr = extract_field(&cap, "ip")?;
    let datetime = extract_field(&cap, "date")?;
    let method = extract_field(&cap, "method")?;
    let url = extract_field(&cap, "url")?;
    let protocol = extract_field(&cap, "proto")?;
    let status = extract_field(&cap, "status")?;
    let body_bytes = extract_field(&cap, "bytes")?;
    let referer = extract_field(&cap, "referer")?;
    let user_agent = extract_field(&cap, "ua")?;

    Ok(NginxLog {
        addr,
        datetime,
        method,
        url,
        protocol,
        status,
        body_bytes,
        referer,
        user_agent,
    })
}
