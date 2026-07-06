// pub async fn get_public_company_info(api_key: &str) -> String {
//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert("Authorization", api_key.parse().unwrap());
//     let client = reqwest::Client::new();
//     let response = client
//         .get("https://api.g2.galactictycoons.com/public/company/exchangeorders")
//         .headers(headers)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await;
//     response.unwrap()
// }

use std::{collections::HashMap, convert};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOrder {
    // id: i32,
    // c_id: i32,
    pub mat_id: i32,
    // pub qty: i32,
    // qty_tot: i32,
    // pub unit_price: i32,
}

pub async fn get_exchange_orders(api_key: &str) -> Vec<ExchangeOrder> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.g2.galactictycoons.com/public/company/exchangeorders")
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    let orders: Vec<ExchangeOrder> = serde_json::from_str(&response.unwrap()).unwrap();
    orders
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub id: i32,
    s_name: String,
    pub name: String,
    description: String,
    //     type // don't care
    weight: f64,
    // source: // don't care
    req_tech: i32,
    tier: i32,
    cp: i32,
}

// pub async fn get_materials_data(api_key: &str) -> Vec<Material> {
//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert("Authorization", api_key.parse().unwrap());
//     let client = reqwest::Client::new();
//     let response = client
//         .get("https://api.g2.galactictycoons.com/gamedata.json")
//         .headers(headers)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await;
//     let json: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
//     let materials: Vec<Material> = serde_json::from_value(json["materials"].clone()).unwrap();
//     materials
// }

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecipeMaterial {
    id: i32,
    am: i32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    id: i32,
    time_minutes: i32,
    inputs: Vec<RecipeMaterial>,
    output: RecipeMaterial,
}

#[derive(Debug)]
pub struct CraftingRecipe {
    recipe_id: i32,
    time_minutes: i32,
    inputs: Vec<(String, i32)>,
    output: (String, i32),
}

impl CraftingRecipe {
    pub fn format(&self, multiplier: &i32) -> String {
        let format_material =
            |material: &(String, i32)| format!("({}x {})", material.1 * multiplier, material.0);
        let inputs = self
            .inputs
            .iter()
            .map(&format_material)
            .collect::<Vec<_>>()
            .join(" + ");
        format!(
            "[{}]-> {} ({} min)",
            inputs,
            format_material(&self.output),
            self.time_minutes
        )
    }
}

pub fn get_crafting_recipes(
    recipes: &[Recipe],
    materials: &[Material],
) -> HashMap<i32, CraftingRecipe> {
    let mut result: HashMap<i32, CraftingRecipe> = HashMap::new();
    let convert_material = |material: &RecipeMaterial| {
        (
            materials
                .iter()
                .find(|m| m.id == material.id)
                .unwrap()
                .name
                .to_owned(),
            material.am,
        )
    };
    for recipe in recipes {
        let crafting_recipe = CraftingRecipe {
            recipe_id: recipe.id,
            time_minutes: recipe.time_minutes,
            inputs: recipe.inputs.iter().map(convert_material).collect(),
            output: convert_material(&recipe.output),
        };
        result.insert(recipe.id, crafting_recipe);
    }
    result
}

pub async fn get_materials_data(api_key: &str) -> (Vec<Recipe>, Vec<Material>) {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.g2.galactictycoons.com/gamedata.json")
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    let json: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
    let recipes: Vec<Recipe> = serde_json::from_value(json["recipes"].clone()).unwrap();
    let materials: Vec<Material> = serde_json::from_value(json["materials"].clone()).unwrap();
    (recipes, materials)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOrderData {
    pub c_name: String,
    pub unit_price: i32,
    pub qty: i32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeData {
    // pub mat_id: i32,
    pub mat_name: String,
    // pub current_price: i32,
    // pub avg_price: i32,
    // pub total_qty_available: i32,
    pub orders: Vec<ExchangeOrderData>,
}

// TODO: extract API access stuff
pub async fn get_exchange_data(api_key: String, material_id: i32) -> Option<ExchangeData> {
    // pub async fn get_exchange_data(api_key: String, material_id: i32) -> Option<ExchangeData> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://api.g2.galactictycoons.com/public/exchange/mat-details/{material_id}"
        ))
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    let exchange_data: ExchangeData = serde_json::from_str(&response.unwrap()).unwrap();
    Some(exchange_data)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductionTask {
    // Recipe ID
    r_id: i32,
    mul: i32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Building {
    cond: f64,
    task: Option<ProductionTask>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildingSlot {
    building: Option<Building>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Workforce {
    workers_satisfaction: [f64; 4],
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseInfo {
    name: String,
    building_slots: Vec<BuildingSlot>,
    workforce: Workforce,
}

use itertools::Itertools;
// temp helper
impl BaseInfo {
    pub fn print_production(&self, recipe_definitions: &HashMap<i32, CraftingRecipe>) {
        let tmp = self
            .building_slots
            .iter()
            .filter_map(|slot| slot.building.as_ref())
            .filter_map(|b| b.task.as_ref())
            .into_grouping_map_by(|task| task.r_id)
            .fold(0, |acc, _key, val| acc + val.mul);
        println!("{}", self.name);
        for task in tmp {
            let recipe = recipe_definitions.get(&task.0).unwrap();
            println!("recipe: {}", recipe.format(&task.1)); //, task.1);
        }
    }
}

pub async fn get_bases(api_key: &str) -> Vec<BaseInfo> {
    if let Ok(response) = query_api("company/bases", api_key).await {
        let exchange_data: Vec<BaseInfo> = serde_json::from_str(&response).unwrap();
        return exchange_data;
    }
    Vec::default()
}

async fn query_api(subpath: &str, api_key: &str) -> Result<String, reqwest::Error> {
    const BASE_PATH: &str = "https://api.g2.galactictycoons.com/public";
    let full_path = format!("{BASE_PATH}/{subpath}");
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", api_key.parse().unwrap());
    let response = client
        .get(full_path)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
