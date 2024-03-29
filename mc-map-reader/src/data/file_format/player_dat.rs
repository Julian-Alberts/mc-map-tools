//! The player.dat file format

use jbe::Builder;

use crate::{
    data::{
        entity::{Entity, Mob},
        item::{Item, ItemWithSlot},
    },
    nbt::{Array, List},
};

/// Information about the player.
/// [Minecraft Wiki](https://minecraft.fandom.com/wiki/Player.dat_format)
#[derive(Debug, Builder, PartialEq)]
pub struct Player {
    /// Generic mob data
    pub mob: Mob,
    /// The player's abilities
    pub abilities: PlayerAbilities,
    /// The version of the data. This value is incremented with every new version of Minecraft.
    pub data_version: i32,
    /// The dimension the player is currently in.
    pub dimension: String,
    /// The player's ender chest
    pub ender_items: List<ItemWithSlot>,
    /// The position the player entered the nether at.
    pub entered_nether_position: Option<EnteredNetherPosition>,

    pub food_exhaustion_level: f32,
    pub food_level: i32,
    pub food_saturation_level: f32,
    pub food_tick_timer: i32,
    pub inventory: List<ItemWithSlot>,
    pub last_death_location: Option<LastDeathLocation>,
    pub player_game_type: i32,
    pub previous_player_game_type: i32,
    pub recipe_book: RecipeBook,
    pub root_vehicle: Option<RootVehicle>,
    pub score: i32,
    pub seen_credits: bool,
    pub selected_item: Option<Item>,
    pub selected_item_slot: i32,
    pub shoulder_entity_left: Option<Entity>,
    pub shoulder_entity_right: Option<Entity>,
    pub sleep_timer: i32,
    pub spawn_dimension: String,
    pub spawn_forced: bool,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub warden_spawn_tracker: Option<WardenSpawnTracker>,
    pub xp_level: i32,
    pub xp_p: f32,
    pub xp_seed: i32,
    pub xp_total: i32,
}

#[derive(Debug, Builder, PartialEq)]
pub struct EnteredNetherPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Builder, PartialEq)]
pub struct LastDeathLocation {
    pub pos: Array<i32>,
    pub dimension: String,
}

#[derive(Debug, Builder, PartialEq)]
pub struct RecipeBook {
    pub recipes: List<String>,
    pub to_be_displayed: List<String>,
    pub is_filtering_craftable: bool,
    pub is_gui_open: bool,
    pub is_furnace_filtering_craftable: bool,
    pub is_furnace_gui_open: bool,
    pub is_blasting_furnace_filtering_craftable: bool,
    pub is_blasting_furnace_gui_open: bool,
    pub is_smoker_filtering_craftable: bool,
    pub is_smoker_gui_open: bool,
}

#[derive(Debug, Builder, PartialEq)]
pub struct RootVehicle {
    pub entity: Entity,
    pub attach: Array<i32>,
}

#[derive(Debug, Builder, PartialEq)]
pub struct WardenSpawnTracker {
    pub cooldown_ticks: i32,
    pub ticks_since_last_warning: i32,
    pub warning_level: i32,
}

#[derive(Debug, Builder, PartialEq)]
pub struct PlayerAbilities {
    pub flying: bool,
    pub fly_speed: f32,
    pub insta_build: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub may_fly: bool,
    pub walk_speed: f32,
}
