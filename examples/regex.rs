use anyhow::{Result, anyhow};
use futures::StreamExt;
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
    let mut framed = Framed::new(file, LinesCodec::new());

    let mut writer =
        csv_async::AsyncSerializer::from_writer(File::create("assets/nginx_logs.csv").await?);

    while let Some(Ok(line)) = framed.next().await {
        println!("{}", line);
        let log = parse_nginx_log(&line)?;
        writer.serialize(log).await?;
    }
    writer.flush().await?;

    Ok(())
}

fn parse_nginx_log(s: &str) -> Result<NginxLog> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
        r#"^(?<ip>\S+)\s+\S+\s+\S+\s+\[(?<date>[^\]]+)\]\s+"(?<method>\S+)\s+(?<url>\S+)\s+(?<proto>[^"]+)"\s+(?<status>\d+)\s+(?<bytes>\d+)\s+"(?<referer>[^"]+)"\s+"(?<ua>[^"]+)"$"#,
    ).unwrap()
    });
    let cap = RE.captures(s).ok_or(anyhow!("parse error"))?;

    let addr = cap
        .name("ip")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse ip error"))?;

    let datetime = cap
        .name("date")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse date error"))?;

    let method = cap
        .name("method")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse method error"))?;

    let url = cap
        .name("url")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse url error"))?;

    let protocol = cap
        .name("proto")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse protocol error"))?;

    let status = cap
        .name("status")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse status error"))?;

    let body_bytes = cap
        .name("bytes")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse body_bytes error"))?;

    let referer = cap
        .name("referer")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse referer error"))?;

    let user_agent = cap
        .name("ua")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse user_agent error"))?;

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
