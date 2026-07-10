use crate::gtc_api::{ExchangeOrder, get_exchange_data, get_exchange_orders};
use colored::Colorize;
pub async fn print_listings() {
    if let Some(api_key) = get_gt_api_key() {
        let exchange_orders = get_exchange_orders(&api_key).await;
        print_exchange_data(&api_key, &exchange_orders).await;
    } else {
        println!(
            "Could not find GT API key. Make sure to store it in the environment variable GT_API_KEY."
        );
    }
}

async fn print_exchange_data(api_key: &str, exchange_orders: &[ExchangeOrder]) -> () {
    let material_ids: std::collections::HashSet<_> =
        exchange_orders.iter().map(|e| e.mat_id).collect();
    let mut tasks = tokio::task::JoinSet::new();
    for id in material_ids {
        let key = api_key.to_owned();
        tasks.spawn(async move { get_exchange_data(key, id).await });
    }
    while let Some(res) = &tasks.join_next().await {
        if let Ok(Some(info)) = res {
            println!("======== {} ========", info.mat_name);
            for order in &info.orders {
                let output = format!(
                    "{:.2}: {} ({})",
                    get_float_price(order.unit_price),
                    order.qty,
                    order.c_name
                );
                if order.c_name == "Tidal Construction Co." {
                    println!("{}", output.purple());
                } else {
                    println!("{output}");
                }
            }
        }
    }
}

fn get_float_price(price: i32) -> f32 {
    price as f32 / 100f32
}

fn get_gt_api_key() -> Option<String> {
    let api_key = std::env::vars().find(|(key, _value)| key == "GT_API_KEY");
    if let Some((_, value)) = api_key {
        return Some(value);
    }
    None
}
