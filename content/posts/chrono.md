---
title: Chrono Code
cover: /images/chrono.webp
date: 2023-10-31T10:59:31+08:00
tags:
  - RUST
draft: true
summary: Rust类型转换代码示例
---

- DateTime<Utc> - Utc的时间（2022-12-06 05:22:19.808711 UTC）
- DateTime<Local> - 本地的带Timezone的时间（2022-12-06 14:22:19.808810 +09:00）
- DateTime<FixedOffset> - 从Utc时间有N个小时偏移的时间（2022-12-06T12:00:00+09:00）
- NaiveDateTime - 不带Timezone的时间（2022-12-06T14:25:43.600317）

```rust
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};

fn main() {
    let utc_datetime = Utc::now();
    let utc_date = Utc::now().date_naive();

    println!("utc_datetime is : {}", utc_datetime); // 2022-12-06 05:25:43.599886 UTC
    println!("utc_date is : {}", utc_date); // 2022-12-06

    let local_datetime = Local::now();
    let local_date = Local::now().date_naive();

    println!("local_datetime is : {}", local_datetime); // 2022-12-06 14:25:43.600005 +09:00
    println!("local_date is : {}", local_date); // 2022-12-06

    022-12-06T12:00:00+09:00:00" x
    let dt = DateTime::parse_from_rfc3339("2022-12-06T12:00:00+09:00");
    println!("after parse dt is : {:?}", dt); // Ok(2022-12-06T12:00:00+09:00)

    ateTime::parse_from_str("2022/12/06 12:00:00 +0900", "%Y/%m/%d %H:%M:%S") x
    let dt_from_str = DateTime::parse_from_str("2022/12/06 12:00:00 +0900", "%Y/%m/%d %H:%M:%S %z");
    println!("after parse dt_from_str is : {:?}", dt_from_str); // 2022-12-06T12:00:00+09:00

    let dt_utc = Utc.datetime_from_str("2022-12-06 14:01:00", "%Y-%m-%d %H:%M:%S");
    println!("after parse dt_utc is : {:?}", dt_utc); // 2022-12-06T14:01:00Z

    let dt_local = Local.datetime_from_str("2022-12-06 14:01:00", "%Y-%m-%d %H:%M:%S");
    println!("after parse dt_local is : {:?}", dt_local); // 2022-12-06T14:01:00+09:00

    // 不带偏移量的时间
    let dt_navie = NaiveDateTime::parse_from_str("2022-12-06 14:01:00", "%Y-%m-%d %H:%M:%S");
    println!("after parse dt_navie is : {:?}", dt_navie); // 2022-12-06T14:01:00

    // format
    let text = Utc::now().format("%Y年%m月%d日").to_string();
    println!("after format text is : {:?}", text); // 2022年12月06日

    // Tz -> no Tz
    let utc = Utc::now();
    let navie_local = utc.naive_local();
    println!("navie_local is : {:?}", navie_local); // 2022-12-06T05:13:14.821945

    let navie_utc = utc.naive_utc();
    println!("navie_utc is : {:?}", navie_utc); // 2022-12-06T05:13:58.176283

    let local = Local::now();
    let local_navie = local.naive_local();
    println!("local_navie is : {:?}", local_navie); // 2022-12-06T14:16:16.096215

    let local_utc = local.naive_utc();
    println!("local_utc is : {:?}", local_utc); // 2022-12-06T05:16:16.096215

    // no Tz -> Tz
    let dt: NaiveDateTime =
        NaiveDateTime::parse_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S").unwrap();
    let utc = Utc.from_local_datetime(&dt).unwrap();
    println!("utc is : {:?}", utc); // 2018-12-07T19:31:28Z

    let local = Local.from_local_datetime(&dt).unwrap();
    println!("local is : {:?}", local); // 2018-12-07T19:31:28+09:00

    let offset = FixedOffset::east_opt(8 * 3600)
        .unwrap()
        .from_local_datetime(&dt)
        .unwrap();
    println!("offset is : {:?}", offset); // 2018-12-07T19:31:28+08:00

    // 日期的计算
    let dt1 = NaiveDateTime::parse_from_str("2022-12-06 14:01:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let dt2 = NaiveDateTime::parse_from_str("2021-12-06 14:01:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let duration = dt1 - dt2;
    println!("duration seconds is : {:?}", duration.num_seconds()); // 31536000
    println!("duration minutes is : {:?}", duration.num_minutes()); // 525600
    println!("duration hours is : {:?}", duration.num_hours()); // 8760
    println!("duration days is : {:?}", duration.num_days()); // 365
}
```
