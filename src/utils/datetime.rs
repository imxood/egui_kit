//! 跨平台时间格式化工具
//!
//! 支持 Native 和 WASM 平台的时间格式化

/// 格式化毫秒时间戳为 YYYY-MM-DD HH:mm:ss
pub fn format_datetime(timestamp_millis: i64) -> String {
    format_datetime_impl(timestamp_millis)
}

/// 格式化毫秒时间戳为日期 YYYY-MM-DD
pub fn format_date(timestamp_millis: i64) -> String {
    format_date_impl(timestamp_millis)
}

/// 格式化毫秒时间戳为时间 HH:mm:ss
pub fn format_time(timestamp_millis: i64) -> String {
    format_time_impl(timestamp_millis)
}

/// 格式化相对时间 (如 "3分钟前", "2小时前")
pub fn format_relative(timestamp_millis: i64) -> String {
    format_relative_impl(timestamp_millis)
}

// ===== Native 实现 =====

#[cfg(not(target_arch = "wasm32"))]
fn format_datetime_impl(timestamp_millis: i64) -> String {
    use chrono::{Local, TimeZone};
    Local
        .timestamp_millis_opt(timestamp_millis)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "无效时间".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn format_date_impl(timestamp_millis: i64) -> String {
    use chrono::{Local, TimeZone};
    Local
        .timestamp_millis_opt(timestamp_millis)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "无效日期".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn format_time_impl(timestamp_millis: i64) -> String {
    use chrono::{Local, TimeZone};
    Local
        .timestamp_millis_opt(timestamp_millis)
        .single()
        .map(|dt| dt.format("%H:%M:%S").to_string())
        .unwrap_or_else(|| "无效时间".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn format_relative_impl(timestamp_millis: i64) -> String {
    use chrono::{Local, TimeZone};

    let now = Local::now();
    let Some(dt) = Local.timestamp_millis_opt(timestamp_millis).single() else {
        return "无效时间".to_string();
    };

    let duration = now.signed_duration_since(dt);
    let seconds = duration.num_seconds();

    if seconds < 0 {
        return "未来".to_string();
    }

    format_duration_chinese(seconds)
}

// ===== WASM 实现 =====

#[cfg(target_arch = "wasm32")]
fn format_datetime_impl(timestamp_millis: i64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp_millis as f64));
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date(),
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}

#[cfg(target_arch = "wasm32")]
fn format_date_impl(timestamp_millis: i64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp_millis as f64));
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

#[cfg(target_arch = "wasm32")]
fn format_time_impl(timestamp_millis: i64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp_millis as f64));
    format!(
        "{:02}:{:02}:{:02}",
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}

#[cfg(target_arch = "wasm32")]
fn format_relative_impl(timestamp_millis: i64) -> String {
    let now = js_sys::Date::new_0();
    let now_millis = now.get_time() as i64;
    let diff_seconds = (now_millis - timestamp_millis) / 1000;

    if diff_seconds < 0 {
        return "未来".to_string();
    }

    format_duration_chinese(diff_seconds)
}

// ===== 通用辅助函数 =====

/// 格式化时长为中文描述
fn format_duration_chinese(seconds: i64) -> String {
    if seconds < 60 {
        return "刚刚".to_string();
    }

    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{}分钟前", minutes);
    }

    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}小时前", hours);
    }

    let days = hours / 24;
    if days < 30 {
        return format!("{}天前", days);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{}个月前", months);
    }

    let years = months / 12;
    format!("{}年前", years)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_chinese() {
        assert_eq!(format_duration_chinese(30), "刚刚");
        assert_eq!(format_duration_chinese(120), "2分钟前");
        assert_eq!(format_duration_chinese(3600), "1小时前");
        assert_eq!(format_duration_chinese(86400), "1天前");
        assert_eq!(format_duration_chinese(86400 * 45), "1个月前");
        assert_eq!(format_duration_chinese(86400 * 400), "1年前");
    }
}
