use lazy_static::lazy_static;
use regex::Regex;

macro_rules! impl_regex {
    ($re_name:ident,$re_str:expr) => {
        /// 正则表达式验证
        ///
        #[doc = concat!("正则表达式：`", stringify!($re_str), "`")]
        pub fn $re_name(text: &str) -> bool {
            lazy_static! {
                static ref RE: Regex = Regex::new($re_str).unwrap();
            }
            RE.is_match(text)
        }
    };
}

impl_regex!(is_url, r"[a-zA-z]+://[^\s]*");

impl_regex!(
    is_mail,
    r"[\w!#$%&'*+/=?^_`{|}~-]+(?:\.[\w!#$%&'*+/=?^_`{|}~-]+)*@(?:[\w](?:[\w-]*[\w])?\.)+[\w](?:[\w-]*[\w])?"
);

impl_regex!(is_chinese, r"[\u4e00-\u9fa5]");

impl_regex!(
    is_phone,
    r"^1(3\d|4[5-9]|5[0-35-9]|6[567]|7[0-8]|8\d|9[0-35-9])\d{8}$"
);

impl_regex!(
    is_id_number,
    r"^(\d{6})(\d{4})(\d{2})(\d{2})(\d{3})([0-9]|X)$"
);

impl_regex!(is_ipv4, r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}");

impl_regex!(is_blank_line, r"^[\s&&[^\n]]*\n$");

impl_regex!(is_blank, r"^\s*$");

impl_regex!(is_number, r"\d+");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        assert!(is_url("http://www.qq.com"));
        assert!(is_mail("123@qq.com"));
        assert!(is_chinese("中国"));
        assert!(is_phone("13838383838"));
        assert!(is_id_number("370283199001011234"));
        assert!(is_ipv4("10.1.1.1"));
        assert!(is_blank_line("   \r\n"));
        assert!(is_blank("  \t  \n"));
        assert!(is_number("123"));
    }
}
