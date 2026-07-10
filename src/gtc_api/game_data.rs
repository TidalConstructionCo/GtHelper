use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn get_game_data() -> Root {
    // let game_data = fs::read_to_string("./game_data.json").unwrap();
    let game_data = fs::read_to_string("src/gtc_api/game_data.json").unwrap();
    // println!("{game_data}");
    serde_json::from_str(&game_data).unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub materials: Vec<Material>,
    pub recipes: Vec<Recipe>,
    pub buildings: Vec<Building>,
    pub workers: Vec<Worker>,
    // pub systems: Vec<System>,
    pub base_building_cost: Vec<BaseBuildingCost>,
    pub ship_emitters: Vec<ShipEmitter>,
    pub ship_reactors: Vec<ShipReactor>,
    pub achievements: Vec<Achievement>,
    pub medal_types: Vec<MedalType>,
    pub build_projects: Vec<BuildProject>,
    pub galaxy_config: GalaxyConfig,
    pub perks: Vec<Perk>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub id: i64,
    pub s_name: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub weight: f64,
    pub source: i64,
    pub req_tech: i64,
    pub tier: i64,
    pub cp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: i64,
    pub produced_in: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub req_tech: i64,
    pub time_minutes: i64,
    pub inputs: Vec<Input>,
    pub output: Output,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub i: i64,
    pub id: i64,
    pub a: i64,
    pub am: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub id: i64,
    pub i: i64,
    pub am: i64,
    pub a: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Building {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub cost: i64,
    pub construction_materials: Vec<ConstructionMaterial>,
    pub workers_needed: Option<Vec<i64>>,
    pub workers_housing: Option<Vec<i64>>,
    pub specialization: i64,
    pub tier: i64,
    pub required_research: i64,
    #[serde(default)]
    pub recipes_ids: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstructionMaterial {
    pub i: i64,
    pub id: i64,
    pub a: i64,
    pub am: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Worker {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub admin_cost: i64,
    pub consumables: Vec<Consumable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumable {
    pub mat_id: i64,
    pub amount: i64,
    pub essential: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub planets: Vec<Planet>,
    pub x: i64,
    pub y: i64,
    pub v: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Planet {
    pub id: i64,
    pub s_id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub mats: Vec<Mat>,
    pub fert: i64,
    pub x: i64,
    pub y: i64,
    pub size: i64,
    pub tier: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mat {
    pub id: i64,
    pub ab: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseBuildingCost {
    pub i: i64,
    pub id: i64,
    pub a: i64,
    pub am: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipEmitter {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub acceleration: f64,
    pub max_speed: f64,
    pub field_capacity: i64,
    pub weight: i64,
    pub energy_draw: i64,
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub name: String,
    pub weight: i64,
    pub energy: i64,
    pub fuel_consumption: f64,
    pub fuel_capacity: i64,
    pub tanks: i64,
    pub fuel_id: i64,
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub other_id: i64,
    pub name: String,
    pub description: String,
    pub milestones: Vec<Milestone>,
    pub reward_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub milestone: i64,
    pub reward_amount: i64,
    pub pr: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedalType {
    pub event_type: i64,
    pub other_id: i64,
    pub period: i64,
    pub pe: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildProject {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub owner_type: i64,
    pub cost: Vec<Cost>,
    #[serde(default)]
    pub cons: Option<Vec<Con>>,
    pub max_level: i64,
    pub buff: i64,
    pub pr: i64,
    pub cost_growth: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub from: i64,
    pub to: i64,
    pub cost: Cost2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost2 {
    pub i: i64,
    pub a: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Con {
    pub from: i64,
    pub to: i64,
    pub cost: Cost3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost3 {
    pub i: i64,
    pub a: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GalaxyConfig {
    pub npc_id: i64,
    pub ship_speed_multipler: i64,
    pub research_speed_multiplier: i64,
    pub slots_per_base: i64,
    pub building_max_level: i64,
    pub wh_size_per_lvl: i64,
    pub base_warehouse_size: i64,
    #[serde(rename = "pxToLY")]
    pub px_to_ly: i64,
    pub prod_bonus_per_tech: i64,
    pub flight_boosted_cost_stars_per_hour: i64,
    #[serde(rename = "buildingUpgradeCostPOWGrowth")]
    pub building_upgrade_cost_powgrowth: f64,
    pub building_upgrade_cost_constant_growth: f64,
    pub hex_size: i64,
    pub hexes_x: i64,
    pub hexes_y: i64,
    pub gp_cons_storage_days: i64,
    pub daily_quest_action_reward_cash: i64,
    pub daily_quest_login_reward_stars: i64,
    pub building_life_days: i64,
    pub production_building_life_days: i64,
    pub building_decay_offset_days: f64,
    pub perk_point_cost_base: i64,
    pub perk_point_cost_step: i64,
    pub perk_tier_unlock_required_points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    pub id: i64,
    pub c: i64,
    pub group: i64,
    pub tier: i64,
    pub max_lvl: i64,
    pub cost_base: i64,
    pub cost_increment: f64,
    pub bonuses: Vec<Bonuse>,
    pub req: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bonuse {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub per_level: f64,
    pub growth: i64,
}
