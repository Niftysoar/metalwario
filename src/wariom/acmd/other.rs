use super::*;

unsafe extern "C" fn game_entryr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_entryr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_entryr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_metalbox"));
    }
    frame(agent.lua_state_agent, 81.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_down_m_02"));
    }
}

unsafe extern "C" fn expression_entryr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_none"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
}

unsafe extern "C" fn game_win1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn sound_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_jump02"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_landing01"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_jump02"));
    }
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_swing_s"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
    }
    frame(agent.lua_state_agent, 129.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_swing_m"));
    }
    frame(agent.lua_state_agent, 133.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_attack05"));
    }
}

unsafe extern "C" fn sound_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 130.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_win02"));
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume(is_metalwario())
        .acmd("game_entryr", game_entryr, High)
        .acmd("game_entryl", game_entryr, High)
        .acmd("effect_entryr", effect_entryr, High)
        .acmd("effect_entryl", effect_entryr, High)
        .acmd("sound_entryr", sound_entryr, High)  
        .acmd("sound_entryl", sound_entryr, High)
        .acmd("expression_entryr", expression_entryr, High)
        .acmd("expression_entryl", expression_entryr, High)

        .acmd("game_win1", game_win1, High)
        .acmd("effect_win1", effect_win1, High)
        .acmd("sound_win1", sound_win1, High)  

        .acmd("game_win2", game_win1, High)
        .acmd("effect_win2", effect_win1, High)
        .acmd("sound_win2", sound_win2, High)  

        .install();
}