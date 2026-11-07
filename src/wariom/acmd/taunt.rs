use super::*;

unsafe extern "C" fn effect_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), -6, 3.5, 0, 0, 0, 0, 0.7, false);
    }
}

unsafe extern "C" fn sound_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("vc_wario_appeal01"));
    }
    wait(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_appeal_h01"));
    }
}

unsafe extern "C" fn sound_appealsr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("vc_wario_appeal02"));
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_appeal_s01"));
    }
}

unsafe extern "C" fn sound_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
        macros::PLAY_SE(agent, Hash40::new("vc_wario_007"));
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume(is_metalwario())
        .acmd("effect_appealhir", effect_appealhir, High)
        .acmd("effect_appealhil", effect_appealhir, High)
        .acmd("sound_appealhir", sound_appealhir, High)
        .acmd("sound_appealhil", sound_appealhir, High)

        .acmd("sound_appeallwr", sound_appeallwr, High)
        .acmd("sound_appeallwl", sound_appeallwr, High)

        .install();
}