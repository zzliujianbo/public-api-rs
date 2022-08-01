use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct QQWeatherResult {
    pub data: QQWeatherResultData,
    pub message: String,
    pub status: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QQWeatherResultData {
    pub air: QQWeatherResultDataAir,
    pub observe: QQWeatherResultDataObserve,
}

///空气质量
#[derive(Debug, Deserialize)]
pub(crate) struct QQWeatherResultDataAir {
    pub aqi: u32,
    pub aqi_level: u32,
    pub aqi_name: String,
    pub update_time: String,
}

///当前天气
#[derive(Debug, Deserialize)]
pub(crate) struct QQWeatherResultDataObserve {
    pub degree: String,
    pub humidity: String,
    pub weather: String,
    pub weather_code: String,
    pub weather_short: String,
    pub wind_direction: String,
    pub wind_power: String,
    pub update_time: String,
}

pub(crate) async fn weather<T>(province: T, city: T) -> Result<QQWeatherResult>
where
    T: Into<String>,
{
    let result = reqwest::get(format!(
        "https://wis.qq.com/weather/common?source=pc&province={}&city={}&weather_type=observe|air",
        province.into(),
        city.into()
    ))
    .await?
    .json::<QQWeatherResult>()
    .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather() {
        tokio_test::block_on(async {
            let result = weather("北京市", "北京市").await;
            dbg!(&result);
            assert!(result.is_ok());
        });
    }
}
