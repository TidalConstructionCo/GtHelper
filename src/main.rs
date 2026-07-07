use crate::gtc_api::{
    ExchangeOrder, get_bases, get_crafting_recipes, get_exchange_data, get_exchange_orders,
    get_materials_data,
};
use clap::{Parser, Subcommand};
use colored::Colorize;
mod gtc_api;

/// Helper tool to query the Galactic Tycoons API. Requires the environment variable GT_API_KEY to contain your API key.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print all exchange listings for items you are selling.
    CheckListings,
}

// TODO: clean up everything
// TODO: add proper item API
// TODO: add missing fields
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::CheckListings) => {
            if let Some(api_key) = get_gt_api_key() {
                let exchange_orders = get_exchange_orders(&api_key).await;
                print_exchange_data(&api_key, &exchange_orders).await;
            } else {
                println!(
                    "Could not find GT API key. Make sure to store it in the environment variable GT_API_KEY."
                );
            }
        }
        None => {
            println!(
                "Running gtc without a command is not supported. Run 'gtc help' or 'gtc --help' to see available commands."
            )
        }
    }
    // // TODO: hardcode materials once / add a subcommand to refresh this info
    // //     let materials = get_materials_data(&api_key);
    // //     print_orders(&exchange_rders.await, &materials.await);
    // let bases = get_bases(&api_key).await;
    // let (recipes, materials) = get_materials_data(&api_key).await;
    // let crafting_recipes = get_crafting_recipes(&recipes, &materials);
    // for base in &bases {
    //     // println!("{:?}", base);
    //     base.print_production(&crafting_recipes);
    // }
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

// fn print_orders(orders: &[ExchangeOrder], materials: &[Material]) {
//     for order in orders.iter() {
//         let material_info = materials
//             .iter()
//             .find(|material| material.id == order.matId)
//             .unwrap();
//
//         println!(
//             "item: {}, amount: {}, price: {}",
//             material_info.name, order.qty, order.unitPrice
//         );
//     }
// }

fn get_gt_api_key() -> Option<String> {
    let api_key = std::env::vars().find(|(key, _value)| key == "GT_API_KEY");
    if let Some((_, value)) = api_key {
        return Some(value);
    }
    None
}
