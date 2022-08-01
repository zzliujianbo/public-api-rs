mod user;

use self::user::StatResult;
use anyhow::Result;
use serde::Deserialize;

/// 哔哩哔哩用户关注数、粉丝数
#[derive(Debug, Deserialize)]
pub struct Stat {
    /// 用户id
    mid: u32,
    /// 关注数
    following: u32,
    ///
    whisper: u32,
    ///
    black: u32,
    /// 粉丝数
    follower: u32,
}

impl From<StatResult> for Stat {
    fn from(result: StatResult) -> Self {
        Self {
            mid: result.data.mid,
            following: result.data.following,
            whisper: result.data.whisper,
            black: result.data.black,
            follower: result.data.follower,
        }
    }
}

/// 调用bilibili接口获取用户关注数、粉丝数
///
/// bilibili获取用户关注数、粉丝数接口地址: `https://api.bilibili.com/x/relation/stat?vmid=`
///
/// # Arguments
/// * `vmid` - 用户id，可以在哔哩哔哩用户主页查看获取，如：罗翔老师的个人主页：`https://space.bilibili.com/517327498`，其中`517327498`就是`vmid`
/// # Examples
/// ```rust
/// # tokio_test::block_on(async {
/// let result = public_api_rs::bilibili::user_stat("517327498").await;
/// assert!(result.is_ok());
/// # });
/// ```
///
pub async fn user_stat<T>(vmid: T) -> Result<Stat>
where
    T: Into<String>,
{
    user::get_stat(vmid).await.map(|r| r.into())
}
