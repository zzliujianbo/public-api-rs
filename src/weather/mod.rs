mod qq;

use self::qq::QQWeatherResult;
use anyhow::Result;
use serde::Deserialize;

///天气数据
#[derive(Debug, Deserialize)]
pub struct Weather {
    ///当前温度
    degree: String,

    ///当前湿度
    humidity: String,

    ///天气中文名字
    weather: String,

    ///天气代码
    weather_code: String,

    ///风向
    wind_direction: String,

    ///风力
    wind_power: String,

    ///空气质量指数
    aqi: u32,

    ///空气质量等级
    aqi_level: u32,

    ///空气质量名字
    aqi_name: String,

    ///当天最高温度，暂未实现
    max_degree: Option<String>,

    ///当天最低温度，暂未实现
    min_degree: Option<String>,
}

impl From<QQWeatherResult> for Weather {
    fn from(result: QQWeatherResult) -> Self {
        let observe = result.data.observe;
        let air = result.data.air;
        Self {
            degree: observe.degree,
            humidity: observe.humidity,
            weather: observe.weather,
            weather_code: observe.weather_code,
            wind_direction: observe.wind_direction,
            wind_power: observe.wind_power,
            aqi: air.aqi,
            aqi_level: air.aqi_level,
            aqi_name: air.aqi_name,
            max_degree: None,
            min_degree: None,
        }
    }
}

/// 调用腾讯天气接口获取天气信息
///
/// 腾讯天气接口地址: `https://wis.qq.com/weather/common`
///
/// # Arguments
/// * `province` - 省份名称
/// * `city` - 城市名称
///
/// # Examples
/// ```rust
/// # tokio_test::block_on(async {
/// let result = public_api_rs::weather::qq_weather("北京市", "北京市").await;
/// assert!(result.is_ok());
/// # });
/// ```
pub async fn qq_weather<T>(province: T, city: T) -> Result<Weather>
where
    T: Into<String>,
{
    qq::weather(province, city).await.map(|r| r.into())
}
