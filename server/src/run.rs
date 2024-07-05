use rand::{seq::IteratorRandom, thread_rng};
use spacetimedb::Timestamp;

use self::base_unit::BaseUnit;

use super::*;

#[spacetimedb(table)]
struct Run {
    #[primarykey]
    id: u64,
    #[unique]
    user_id: u64,

    next_id: u64,

    team: Vec<TeamSlot>,
    shop: Vec<ShopSlot>,
    fusion: Option<Fusion>,
    g: i32,
    price_reroll: i32,
    price_unit: i32,

    round: u32,

    last_updated: Timestamp,
}

#[derive(SpacetimeType, Clone, Default)]
struct ShopSlot {
    unit: String,
    id: u64,
    price: i32,
    freeze: bool,
    discount: bool,
    available: bool,
}

#[derive(SpacetimeType, Default, Clone)]
struct TeamSlot {
    unit: Option<FusedUnit>,
    extra: bool,
}

#[derive(SpacetimeType)]
struct Fusion {
    options: Vec<FusedUnit>,
    source: u8,
    target: u8,
}

#[spacetimedb(reducer)]
fn run_start(ctx: ReducerContext) -> Result<(), String> {
    let user = User::find_by_identity(&ctx.sender)?;
    Run::delete_by_user_id(&user.id);
    let mut run = Run::new(user.id);
    run.fill_case();
    Run::insert(run)?;
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_reroll(ctx: ReducerContext) -> Result<(), String> {
    let mut run = Run::current(&ctx)?;
    if run.g < run.price_reroll {
        return Err("Not enough G".into());
    }
    run.g -= run.price_reroll;
    run.fill_case();
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn shop_buy(ctx: ReducerContext, slot: u8) -> Result<(), String> {
    let mut run = Run::current(&ctx)?;
    run.buy(slot)?;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn fuse_start(ctx: ReducerContext, target: u8, source: u8) -> Result<(), String> {
    let mut run = Run::current(&ctx)?;
    let source_unit = run.get_team_mut(source)?.clone();
    if source_unit.bases.len() != 1 {
        return Err("Source can only be non-fused unit".to_owned());
    }
    let target_unit = run.get_team_mut(target)?.clone();
    run.fusion = Some(Fusion {
        options: Vec::default(),
        source,
        target,
    });
    let options = &mut run.fusion.as_mut().unwrap().options;

    let mut target_trigger = target_unit.clone();
    target_trigger
        .triggers
        .extend(source_unit.triggers.clone().into_iter());
    options.push(target_trigger);

    let mut target_target = target_unit.clone();
    target_target
        .targets
        .extend(source_unit.targets.clone().into_iter());
    options.push(target_target);

    let mut target_effect = target_unit.clone();
    target_effect
        .effects
        .extend(source_unit.effects.clone().into_iter());
    options.push(target_effect);
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn fuse_cancel(ctx: ReducerContext) -> Result<(), String> {
    let mut run = Run::current(&ctx)?;
    if run.fusion.is_none() {
        return Err("Fusion not started".to_owned());
    }
    run.fusion = None;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn fuse_choose(ctx: ReducerContext, ind: u8) -> Result<(), String> {
    let mut run = Run::current(&ctx)?;
    let fusion = run.fusion.take().context_str("Fusion not started")?;
    if fusion.options.len() > ind as usize {
        return Err("Wrong fusion index".to_owned());
    }
    run.remove_team(fusion.source)?;
    run.remove_team(fusion.target)?;
    let slot = fusion.source.min(fusion.target);
    run.team
        .get_mut(slot as usize)
        .context_str("Fusion insert error")?
        .unit = Some(
        fusion
            .options
            .get(ind as usize)
            .cloned()
            .context_str("Fusion option get error")?,
    );
    run.fusion = None;
    run.save();
    Ok(())
}

#[spacetimedb(reducer)]
fn stack(ctx: ReducerContext, source: u8, target: u8) -> Result<(), String> {
    let mut run = Run::current(&ctx)?;
    let source = run.remove_team(source)?;
    let target = run.get_team_mut(target)?;
    if source.bases.len() != 1 {
        return Err("Fused unit can't be stack source".to_owned());
    }
    if !(target.stacks == 1
        && source
            .bases
            .first()
            .unwrap()
            .eq(target.bases.first().unwrap()))
    {
        return Err("First level unit can only be stacked with same unit".to_owned());
    }
    if !target
        .get_houses()
        .contains(&source.get_houses().first().unwrap())
    {
        return Err("Source house has to be one of target houses".to_owned());
    }
    target.stacks += source.stacks;
    run.save();
    Ok(())
}

impl Run {
    fn new(user_id: u64) -> Self {
        let gs = GlobalSettings::get();
        Self {
            id: GlobalData::next_id(),
            user_id,
            team: vec![TeamSlot::default(); gs.team_slots as usize],
            shop: vec![ShopSlot::default(); gs.shop_slots_max as usize],
            fusion: None,
            round: 0,
            last_updated: Timestamp::now(),
            next_id: 0,
            g: gs.shop_g_start,
            price_reroll: gs.shop_price_reroll,
            price_unit: gs.shop_price_unit,
        }
    }
    fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }
    fn current(ctx: &ReducerContext) -> Result<Self, String> {
        Run::filter_by_user_id(&User::find_by_identity(&ctx.sender)?.id)
            .context_str("No arena run in progress")
    }
    fn buy(&mut self, slot: u8) -> Result<(), String> {
        let s = self
            .shop
            .get_mut(slot as usize)
            .context_str("Wrong shop slot")?;
        if !s.available {
            return Err("Unit already bought".to_owned());
        }
        if s.price > self.g {
            return Err("Not enough G".to_owned());
        }
        self.g -= s.price;
        s.available = false;
        let unit = FusedUnit::from_base(s.unit.clone(), self.next_id());
        let slot = if let Some(slot) = self.team.iter_mut().find(|s| s.unit.is_none()) {
            slot
        } else {
            self.team.push(TeamSlot {
                unit: None,
                extra: true,
            });
            self.team.last_mut().unwrap()
        };
        slot.unit = Some(unit);
        Ok(())
    }
    fn get_team_mut(&mut self, slot: u8) -> Result<&mut FusedUnit, String> {
        self.team
            .get_mut(slot as usize)
            .and_then(|u| u.unit.as_mut())
            .context_str("Unit not found")
    }
    fn remove_team(&mut self, slot: u8) -> Result<FusedUnit, String> {
        if let Some(slot) = self.team.get_mut(slot as usize) {
            if let Some(unit) = slot.unit.take() {
                return Ok(unit);
            } else {
                return Err("Slot is empty".to_owned());
            }
        } else {
            return Err("Wrong team slot".to_owned());
        }
    }
    fn save(mut self) {
        self.last_updated = Timestamp::now();
        Self::update_by_user_id(&self.user_id.clone(), self);
    }
    fn fill_case(&mut self) {
        let gs = GlobalSettings::get();
        let slots = (gs.shop_slots_min + (gs.shop_slots_per_round * self.round as f32) as u32)
            .min(gs.shop_slots_max) as usize;
        self.shop = vec![ShopSlot::default(); slots];
        for i in 0..slots {
            let id = self.next_id();
            let s = &mut self.shop[i];
            s.available = true;
            s.price = self.price_unit;
            s.id = id;
            s.unit = BaseUnit::iter().choose(&mut thread_rng()).unwrap().name;
        }
    }
}
