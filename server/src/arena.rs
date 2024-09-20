use std::mem;

use itertools::Itertools;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use spacetimedb::Timestamp;

use self::base_unit::TBaseUnit;

use super::*;

fn settings() -> GlobalSettings {
    GlobalSettings::get()
}

#[spacetimedb(table(public))]
pub struct TArenaRun {
    mode: GameMode,
    #[primarykey]
    id: u64,
    #[unique]
    owner: u64,
    team: u64,
    battles: Vec<u64>,
    enemies: Vec<u64>,
    shop_slots: Vec<ShopSlot>,
    team_slots: Vec<TeamSlot>,
    fusion: Option<Fusion>,
    g: i32,
    price_reroll: i32,
    free_rerolls: u32,
    lives: u32,
    active: bool,
    finale: bool,

    round: u32,
    rerolls: u32,
    score: u32,
    rewards: Vec<Reward>,
    reward_limit: i64,

    last_updated: Timestamp,
}

#[spacetimedb(table(public))]
pub struct TArenaRunArchive {
    mode: GameMode,
    #[primarykey]
    id: u64,
    owner: u64,
    team: u64,
    battles: Vec<u64>,
    round: u32,
}

impl TArenaRunArchive {
    fn add_from_run(run: TArenaRun) {
        Self::insert(Self {
            mode: run.mode,
            id: run.id,
            owner: run.owner,
            team: run.team,
            battles: run.battles,
            round: run.round,
        })
        .expect("Failed to archive a run");
    }
}

#[derive(SpacetimeType, Clone, Default)]
pub struct ShopSlot {
    unit: String,
    id: u64,
    buy_price: i32,
    stack_price: i32,
    freeze: bool,
    discount: bool,
    available: bool,
    stack_targets: Vec<u8>,
    house_filter: Vec<String>,
}

#[derive(SpacetimeType, Clone, Default)]
pub struct TeamSlot {
    stack_targets: Vec<u8>,
    fuse_targets: Vec<u8>,
    sell_price: i32,
}

#[derive(SpacetimeType)]
pub struct Fusion {
    options: Vec<FusedUnit>,
    a: u8,
    b: u8,
}

#[derive(SpacetimeType)]
pub struct Reward {
    name: String,
    amount: i64,
}

impl Reward {
    fn new(name: String, amount: i64) -> Self {
        Self { name, amount }
    }
}

#[spacetimedb(reducer)]
fn run_start_normal(ctx: ReducerContext) -> Result<(), String> {
    let user = ctx.user()?;
    TArenaRun::start(user, GameMode::ArenaNormal)
}

#[spacetimedb(reducer)]
fn run_start_ranked(ctx: ReducerContext, team_id: u64) -> Result<(), String> {
    let user = ctx.user()?;
    TWallet::change(user.id, -settings().arena.ranked_cost_min)?;
    let team = TTeam::get_owned(team_id, user.id)?;
    TArenaRun::start(user, GameMode::ArenaRanked)?;
    let mut run = TArenaRun::current(&ctx)?;
    run.team = team.id;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn run_start_const(ctx: ReducerContext) -> Result<(), String> {
    let user = ctx.user()?;
    TArenaRun::start(
        user,
        GameMode::ArenaConst(GlobalData::get().constant_seed.clone()),
    )
}

#[spacetimedb(reducer)]
fn run_finish(ctx: ReducerContext) -> Result<(), String> {
    let run = TArenaRun::current(&ctx)?;
    let mut reward: i64 = run.rewards.iter().map(|r| r.amount).sum();
    if run.reward_limit > 0 {
        reward = reward.min(run.reward_limit);
    }
    TWallet::change(run.owner, reward)?;
    TArenaRun::delete_by_id(&run.id);
    TArenaRunArchive::add_from_run(run);
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_finish(ctx: ReducerContext) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    run.round += 1;
    let team = TTeam::get(run.team)?.save_clone();
    let enemy = if let Some(enemy) = run.enemies.first().copied() {
        run.enemies.remove(0);
        enemy
    } else {
        let c = TArenaLeaderboard::current_champion(&run.mode);
        if c.as_ref().is_some_and(|c| c.round == run.round) {
            run.enemies.push(team.id);
            run.finale = true;
            c.unwrap().team
        } else {
            TArenaPool::get_random(&run.mode, run.round)
                .map(|d| d.team)
                .unwrap_or_default()
        }
    };
    if enemy == 0 {
        run.finale = true;
    }

    run.battles
        .push(TBattle::new(run.mode.clone(), run.owner, team.id, enemy));
    if !team.units.is_empty() {
        TArenaPool::add(run.mode.clone(), team.id, run.round);
    }
    run.rerolls = 0;
    run.fill_case()?;
    let ars = &settings().arena;
    run.g += (ars.g_income_min + ars.g_income_per_round * run.round as i32).max(ars.g_income_max);
    run.free_rerolls += ars.free_rerolls_income;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn submit_battle_result(ctx: ReducerContext, result: TBattleResult) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let bid = *run.battles.last().context_str("Last battle not present")?;
    let battle = TBattle::get(bid)?;
    if !battle.is_tbd() {
        return Err("Result already submitted".to_owned());
    }
    battle.set_result(result).save();
    if matches!(result, TBattleResult::Left) {
        run.score += run.round;
    } else {
        if run.finale {
            run.lives = 0;
        } else {
            run.lives -= 1;
        }
    }
    if run.lives == 0 || (run.finale && run.enemies.is_empty()) {
        run.finish();
    }
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_reorder(ctx: ReducerContext, from: u8, to: u8) -> Result<(), String> {
    let run = TArenaRun::current(&ctx)?;
    let mut team = TTeam::get(run.team)?;
    let from = from as usize;
    let to = (to as usize).min(team.units.len() - 1);
    if team.units.len() < from {
        return Err("Wrong from index".into());
    }
    let unit = team.units.remove(from);
    team.units.insert(to, unit);
    team.save();
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_set_freeze(ctx: ReducerContext, slot: u8, value: bool) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let slot = run
        .shop_slots
        .get_mut(slot as usize)
        .context_str("Wrong slot index")?;
    if slot.freeze == value {
        if value {
            return Err("Slot already frozen".into());
        } else {
            return Err("Slot already unfrozen".into());
        }
    }
    slot.freeze = value;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_reroll(ctx: ReducerContext) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    if run.free_rerolls > 0 {
        run.free_rerolls -= 1;
    } else {
        if run.g < run.price_reroll {
            return Err("Not enough G".into());
        }
        run.g -= run.price_reroll;
    }
    run.rerolls += 1;
    run.fill_case()?;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_buy(ctx: ReducerContext, slot: u8) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let price = run.shop_slots[slot as usize].buy_price;
    let unit = run.buy(slot, price)?;
    run.add_to_team(FusedUnit::from_base_name(unit, next_id())?)?;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_sell(ctx: ReducerContext, slot: u8) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    run.sell(slot as usize)?;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_change_g(ctx: ReducerContext, delta: i32) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    run.g += delta;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn fuse_start(ctx: ReducerContext, a: u8, b: u8) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let team = run.team()?;
    let fusion = Fusion {
        options: FusedUnit::fuse(team.get_unit(a)?, team.get_unit(b)?)?,
        a,
        b,
    };
    run.fusion = Some(fusion);
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn fuse_cancel(ctx: ReducerContext) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    if run.fusion.is_none() {
        return Err("Fusion not started".to_owned());
    }
    run.fusion = None;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn fuse_choose(ctx: ReducerContext, slot: u8) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let Fusion { options, a, b } = run.fusion.take().context_str("Fusion not started")?;
    let slot = slot as usize;
    let a = a as usize;
    let b = b as usize;
    if slot >= options.len() {
        return Err("Wrong fusion index".to_owned());
    }
    let mut team = run.team()?;
    team.units.remove(a);
    team.units.insert(a, options[slot].clone());
    team.units.remove(b);
    team.save();
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn stack_shop(ctx: ReducerContext, source: u8, target: u8) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let price = run.shop_slots[source as usize].stack_price;
    let unit = run.buy(source, price)?;
    let mut team = run.team()?;
    let target = team
        .units
        .get_mut(target as usize)
        .context_str("Team unit not found")?;
    if !target.can_stack(&unit) {
        return Err(format!(
            "Units {unit} and {} can not be stacked",
            target.bases.join("+")
        ));
    }
    target.add_xp(1);
    team.save();
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn stack_team(ctx: ReducerContext, source: u8, target: u8) -> Result<(), String> {
    let mut run = TArenaRun::current(&ctx)?;
    let mut team = run.team()?;
    let source_unit = team.units[source as usize].clone();
    let target_unit = team
        .units
        .get_mut(target as usize)
        .context_str("Team unit not found")?;
    if !target_unit.can_stack_fused(&source_unit) {
        return Err(format!(
            "Units {} and {} can not be stacked",
            source_unit.bases.join("+"),
            target_unit.bases.join("+")
        ));
    }
    target_unit.add_xp(1);
    team.units.remove(source as usize);
    team.save();
    run.save();
    Ok(())
}

impl TArenaRun {
    fn new(user_id: u64, mode: GameMode) -> Self {
        let ars = settings().arena;
        Self {
            id: next_id(),
            owner: user_id,
            team: TTeam::new(user_id, TeamPool::Arena).save(),
            shop_slots: Vec::new(),
            team_slots: Vec::new(),
            fusion: None,
            round: 0,
            rerolls: 0,
            score: 0,
            last_updated: Timestamp::now(),
            g: ars.g_start,
            price_reroll: ars.price_reroll,
            battles: Vec::new(),
            lives: ars.lives_initial,
            free_rerolls: ars.free_rerolls_initial,
            active: true,
            finale: false,
            mode,
            rewards: Vec::new(),
            reward_limit: 0,
            enemies: GlobalData::get().initial_enemies,
        }
    }
    fn start(user: TUser, mode: GameMode) -> Result<(), String> {
        TArenaRun::delete_by_owner(&user.id);
        let reward_limit = match mode {
            GameMode::ArenaNormal | GameMode::ArenaConst(_) => settings().arena.ranked_cost_min,
            GameMode::ArenaRanked => 0,
        };
        let mut run = TArenaRun::new(user.id, mode);
        run.fill_case()?;
        run.reward_limit = reward_limit;
        TArenaRun::insert(run)?;
        Ok(())
    }
    fn current(ctx: &ReducerContext) -> Result<Self, String> {
        Self::filter_by_owner(&ctx.user()?.id).context_str("No arena run in progress")
    }
    fn buy(&mut self, slot: u8, price: i32) -> Result<String, String> {
        let s = self
            .shop_slots
            .get_mut(slot as usize)
            .context_str("Wrong shop slot")?;
        if !s.available {
            return Err("Unit already bought".to_owned());
        }
        if price > self.g {
            return Err("Not enough G".to_owned());
        }
        self.g -= price;
        s.available = false;
        Ok(s.unit.clone())
    }
    fn add_to_team(&mut self, unit: FusedUnit) -> Result<(), String> {
        let mut team = self.team()?;
        team.units.insert(0, unit);
        team.save();
        Ok(())
    }
    fn sell(&mut self, slot: usize) -> Result<(), String> {
        self.remove_team(slot)?;
        self.g += self.team_slots[slot].sell_price;
        Ok(())
    }
    fn team(&mut self) -> Result<TTeam, String> {
        TTeam::get(self.team)
    }
    fn remove_team(&mut self, slot: usize) -> Result<FusedUnit, String> {
        let mut team = self.team()?;
        if team.units.len() > slot {
            let unit = team.units.remove(slot);
            team.save();
            return Ok(unit);
        } else {
            return Err("Slot is empty".to_owned());
        }
    }
    fn save(mut self) {
        self.last_updated = Timestamp::now();
        let team = TTeam::get(self.team).unwrap();
        for slot in &mut self.shop_slots {
            slot.stack_targets.clear();
            if !slot.available {
                continue;
            }
            for (i, unit) in team.units.iter().enumerate() {
                if unit.can_stack(&slot.unit) {
                    slot.stack_targets.push(i as u8);
                }
            }
        }
        let GlobalSettings {
            arena: ars,
            rarities: rs,
            ..
        } = &settings();
        self.team_slots = vec![TeamSlot::default(); team.units.len()];
        for (slot_i, slot) in self.team_slots.iter_mut().enumerate() {
            slot.stack_targets.clear();
            if let Some(unit) = team.units.get(slot_i) {
                let rarity = unit.rarity() as usize;
                slot.sell_price = rs.prices[rarity] - ars.sell_discount;
            }
            for (i, unit) in team.units.iter().enumerate() {
                if i == slot_i {
                    continue;
                }
                if unit.can_stack_fused(&team.units[slot_i]) {
                    slot.stack_targets.push(i as u8);
                }
                if FusedUnit::can_fuse(unit, &team.units[slot_i]) {
                    slot.fuse_targets.push(i as u8);
                }
            }
        }
        Self::update_by_owner(&self.owner.clone(), self);
    }
    fn fill_case(&mut self) -> Result<(), String> {
        let GlobalSettings {
            arena: ars,
            rarities,
            ..
        } = settings();
        let slots = (ars.slots_min + (ars.slots_per_round * self.round as f32) as u32)
            .min(ars.slots_max) as usize;
        let mut old_slots = Vec::default();
        mem::swap(&mut self.shop_slots, &mut old_slots);
        for i in 0..slots {
            if let Some(slot) = old_slots.get(i) {
                if slot.available && slot.freeze {
                    self.shop_slots.push(slot.clone());
                    continue;
                }
            }
            self.shop_slots.push(ShopSlot::default());
        }
        let team = self.team()?;
        if !team.units.is_empty() {
            self.shop_slots[0].house_filter = team
                .units
                .iter()
                .flat_map(|u| u.get_houses())
                .unique()
                .collect();
        }
        let round = self.round as i32;
        let weights = rarities
            .weights_initial
            .iter()
            .enumerate()
            .map(|(i, w)| (*w + rarities.weights_per_round[i] * round).max(0))
            .collect_vec();
        let mut rng = self.get_rng();
        for i in 0..slots {
            let id = next_id();
            let s = &mut self.shop_slots[i];
            if s.freeze {
                continue;
            }
            s.available = true;
            s.id = id;
            let unit = TBaseUnit::get_random(&s.house_filter, &weights, &mut rng);
            s.unit = unit.name.clone();
            s.buy_price = rarities.prices[unit.rarity as usize];
            s.stack_price = s.buy_price - ars.stack_discount;
        }
        Ok(())
    }
    fn get_seed(&self) -> String {
        let Self {
            id, round, rerolls, ..
        } = self;
        match &self.mode {
            GameMode::ArenaNormal | GameMode::ArenaRanked => format!("{id}_{round}_{rerolls}"),
            GameMode::ArenaConst(seed) => format!("{seed}_{round}_{rerolls}"),
        }
    }
    fn get_rng(&self) -> Pcg64 {
        Seeder::from(self.get_seed()).make_rng()
    }
    fn finish(&mut self) {
        self.active = false;
        if GameMode::ArenaRanked.eq(&self.mode) {
            self.rewards.push(Reward::new(
                "Ranked score * 2".into(),
                self.score as i64 * 2,
            ));
        } else {
            self.rewards
                .push(Reward::new("Score".into(), self.score as i64));
        }
        if self.finale {
            if self
                .battles
                .last()
                .and_then(|id| TBattle::filter_by_id(id))
                .map(|b| b.result.is_win())
                .unwrap_or_default()
            {
                self.rewards
                    .push(Reward::new("Champion defeated".into(), 10));
                TArenaLeaderboard::insert(TArenaLeaderboard::new(
                    self.mode.clone(),
                    self.round,
                    self.score,
                    self.owner,
                    self.team,
                    self.id,
                ));
            }
        }
    }
}
