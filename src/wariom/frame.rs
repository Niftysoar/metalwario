use super::*;

unsafe extern "C" fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let motion = MotionModule::motion_kind(boma);
        let status = StatusModule::status_kind(boma);
        
        if crate::MARKED_COLORS[color as usize] {

        }
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume(is_metalwario())
        .on_line(Main, wario_frame)

        .install();
}