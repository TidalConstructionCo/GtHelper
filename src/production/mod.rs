use crate::gtc_api::game_data::get_game_data;
use crate::gtc_api::get_public_company_info;
use crate::gtc_api::public_info::SpecializationEnum;

pub async fn print_production() -> () {
    if let Some(api_key) = crate::listings::get_gt_api_key() {
        if let Some(company_info) = get_public_company_info(&api_key).await {
            let research = get_research_productivity(&company_info);
            let perks = get_perks_productivity(&company_info);
            println!("research: {research}, perks: {perks}");
        }
    }
    todo!("implement");
}

fn get_research_productivity(company_info: &crate::gtc_api::public_info::PMyCompanyModel) -> f64 {
    company_info
        .technologies
        .as_ref()
        .unwrap()
        .iter()
        .find(|t| {
            if let Some(id) = t.id {
                return id == SpecializationEnum::Electronics as i32;
            }
            false
        })
        .unwrap()
        .level
        .unwrap() as f64
        * 0.05
}

fn get_perks_productivity(company_info: &crate::gtc_api::public_info::PMyCompanyModel) -> f64 {
    let game_data = get_game_data();
    let productivity_perks: Vec<_> = game_data
        .perks
        .iter()
        .filter(|perk| perk.bonuses.iter().any(|bonus| bonus.type_field == 7))
        .collect(); // TODO: 8 is a multiplier to final prod speed (e.g., lax) => consider later too

    let mut result = 0f64;
    for perk in company_info.perks.as_ref().unwrap().iter() {
        if let (Some(id), Some(level)) = (perk.id, perk.lvl) {
            if let Some(perk_data) = productivity_perks.iter().find(|p| p.id == id as i64) {
                let perk_productivity: f64 = perk_data
                    .bonuses
                    .iter()
                    .filter(|b| b.type_field == 7)
                    .map(|bonus| bonus.per_level * level as f64)
                    .sum();
                println!("perk: {:?}, data: {:?}", perk, perk_data);
                result += perk_productivity;
            }
        }
    }
    result / 100f64
    //387.5
}

// fn get_productivity(research_level: i32) -> f64 {
//     let research_mult: f64 = 0.05 * research_level as f64;
//     1f64 * research_mult
//     // WorkforceSatisfaction * RequiredWorkforce * BuildingCondition * TechAndPerks
// }
