#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![allow(warnings)]
#![deny(
    deprecated
)]

#[macro_use]

use std::collections::HashMap;
use smash::hash40;
use csk::*;
use param_config::*;
use smash::lib::lua_const::{FIGHTER_KIND_WARIO};

mod wariom;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];
pub static mut LAST_COLOR: i32 = -1;

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "wario";
    const MARKER_FILE: &str = "wariom.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }

    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            LAST_COLOR = index - 1;
            index - lowest_color
        }
    };

    // Param Edits

    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("weight"), 0, 230.0));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("walk_accel_mul"), 0, 0.05));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("walk_accel_add"), 0, 0.15));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("walk_speed_max"), 0, 0.9));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("dash_speed"), 0, 1.4));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.06));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("run_accel_add"), 0, 0.02));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("run_speed_max"), 0, 1.05));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("jump_y"), 0, 30.0));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("mini_jump_y"), 0, 7.72));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("jump_aerial_y"), 0, 13.5));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("air_speed_y_stable"), 0, 2.45));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("dive_speed_y"), 0, 2.6));
    update_float_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.145));
    update_int_2(*FIGHTER_KIND_WARIO, marked_slots.clone(), (hash40("attack_combo_max"), 0, 3));

    // CSK Stuff

    add_narration_characall_entry("vc_narration_characall_wariom");

    add_chara_db_entry_info(CharacterDatabaseEntry{
        ui_chara_id: hash40("ui_chara_wariom"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_wario")),
        fighter_type: Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
        alt_chara_id: Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
        shop_item_tag: Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
        name_id: StringType::Overwrite(CStrCSK::new("wariom")),
        exhibit_year: ShortType::Overwrite(2011), 
        extra_flags: IntType::Overwrite(0), 
        ext_skill_page_num: SignedByteType::Overwrite(0),
        ui_series_id: Hash40Type::Overwrite(hash40("ui_series_wario")),
        disp_order: SignedByteType::Optional(Some(90)), 
        chara_count: SignedByteType::Overwrite(1), 
        is_img_ext_skill_page0: BoolType::Overwrite(false), 
        is_img_ext_skill_page1: BoolType::Overwrite(false), 
        is_img_ext_skill_page2: BoolType::Overwrite(false), 
        can_select: BoolType::Overwrite(true), 
        is_usable_soundtest: BoolType::Overwrite(true), 
        is_called_pokemon: BoolType::Overwrite(false), 
        is_mii: BoolType::Overwrite(false), 
        is_boss: BoolType::Overwrite(false), 
        is_hidden_boss: BoolType::Overwrite(false), 
        is_dlc: BoolType::Overwrite(false), 
        is_patch: BoolType::Overwrite(false), 
        is_plural_message: BoolType::Overwrite(false), 
        is_plural_narration: BoolType::Overwrite(false), 
        is_article: BoolType::Overwrite(false), 
        result_pf0: BoolType::Overwrite(true), 
        result_pf1: BoolType::Overwrite(true), 
        result_pf2: BoolType::Overwrite(true), 
        color_num: UnsignedByteType::Overwrite(color_num as _),
        extra_index_maps: UnsignedByteMap::Overwrite(HashMap::from([
            (hash40("color_start_index"), UnsignedByteType::Overwrite(lowest_color as u8))
        ])),
        extra_hash_maps: Hash40Map::Overwrite(HashMap::from([
            (hash40("characall_label_c00"), Hash40Type::Overwrite(hash40("vc_narration_characall_wariom"))),
            (hash40("original_ui_chara_hash"), Hash40Type::Overwrite(hash40("ui_chara_wariom")))
        ])),
        ..Default::default()
    });

    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_wariom_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_wario_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_wariom")),
        ..Default::default()
    });

    add_series_db_entry_info(SeriesDatabaseEntry {
        ui_series_id: hash40("ui_series_wario"),
        name_id: StringType::Overwrite(CStrCSK::new("wario")),
        disp_order: SignedByteType::Overwrite(0),
        disp_order_sound: SignedByteType::Overwrite(0),
        save_no: SignedByteType::Overwrite(0),
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..Default::default()
    });

    wariom::install();
}

#[skyline::main(name = "smashline_test")]
pub fn main() {

    unsafe {
        extern "C" {
            fn allow_ui_chara_hash_online(ui_chara_hash: u64);
            fn arcrop_register_event_callback(ty: arcropolis_api::Event, callback: arcropolis_api::EventCallbackFn,);
        }
        allow_ui_chara_hash_online(hash40("ui_chara_wariom"));
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }

}