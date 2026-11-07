use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
        hash40
    },
    smash_script::*,
    smashline::*
};

use smashline::Priority::*;
use crate::MARKED_COLORS;

mod acmd;
mod wariobike;
mod status;

pub const FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_IMPACT:  i32 = 0x1F5;

pub fn is_metalwario() -> Vec<usize> {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    costume.to_vec()
}

pub fn install() {

    acmd::install();
    wariobike::install();
    status::install();
}