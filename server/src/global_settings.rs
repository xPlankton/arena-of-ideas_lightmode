use super::*;

#[spacetimedb(table(public))]
pub struct GlobalSettings {
    #[unique]
    always_zero: u32,
    pub arena: ArenaSettings,
    pub rarities: RaritySettings,
    pub battle: BattleSettings,
    pub craft_shards_cost: u32,
    pub meta: MetaSettings,
    pub ghost_unit: String,
}

impl GlobalSettings {
    pub fn get() -> Self {
        GlobalSettings::filter_by_always_zero(&0).unwrap()
    }
    pub fn replace(self) {
        GlobalSettings::delete_by_always_zero(&0);
        let _ = GlobalSettings::insert(self);
    }
}

#[derive(SpacetimeType)]
pub struct BattleSettings {
    pub fatigue_start: u32,
    pub deafness_start: u32,
    pub deafness_per_turn: f32,
}

#[derive(SpacetimeType)]
pub struct ArenaSettings {
    pub ranked_cost_min: i64,
    pub ranked_cost_max: i64,
    pub ranked_cost_increase: i64,
    pub const_cost_min: i64,
    pub const_cost_max: i64,
    pub const_cost_increase: i64,

    pub slots_min: u32,
    pub slots_max: u32,
    pub slots_per_round: f32,
    pub g_start: i32,
    pub g_income_min: i32,
    pub g_income_max: i32,
    pub g_income_per_round: i32,
    pub price_reroll: i32,
    pub sell_discount: i32,
    pub stack_discount: i32,
    pub team_slots: u32,
    pub lives_initial: u32,
    pub lives_per_wins: u32,
    pub free_rerolls_initial: u32,
    pub free_rerolls_income: u32,
}

#[derive(SpacetimeType)]
pub struct RaritySettings {
    pub prices: Vec<i32>,
    pub weights_initial: Vec<i32>,
    pub weights_per_round: Vec<i32>,
    pub lootbox_weights: Vec<i32>,
}

#[derive(SpacetimeType)]
pub struct MetaSettings {
    pub price_lootbox: i64,
    pub price_shard: i64,
    pub shop_refresh_period_secs: u64,
    pub shop_shard_slots: u32,
}
