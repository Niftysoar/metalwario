use super::*;

unsafe extern "C" fn effect_entryr(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario_wariobike")
    .set_costume(is_metalwario())
        .acmd("effect_entryr", effect_entryr, High)
        .acmd("effect_entryl", effect_entryr, High)

        .install();
}