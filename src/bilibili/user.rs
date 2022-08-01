use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct StatResult {
    pub code: u32,
    pub message: String,
    pub ttl: u32,
    pub data: StatResultData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StatResultData {
    pub mid: u32,
    pub following: u32,
    pub whisper: u32,
    pub black: u32,
    pub follower: u32,
}

pub async fn get_stat<T>(vmid: T) -> Result<StatResult>
where
    T: Into<String>,
{
    let result = reqwest::get(format!(
        "https://api.bilibili.com/x/relation/stat?vmid={}",
        vmid.into()
    ))
    .await?
    .json::<StatResult>()
    .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stat() {
        tokio_test::block_on(async {
            let result = get_stat("517327498").await;
            dbg!(&result);
            assert!(result.is_ok());
        });
    }
}
