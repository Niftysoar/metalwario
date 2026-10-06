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

pub const FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_IMPACT:  i32 = 0x1F5;