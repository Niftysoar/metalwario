use super::*;

unsafe extern "C" fn special_s_impact_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_impact").into(), Hash40::new("special_air_s_impact").into(), false.into(),);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_impact_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_impact_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation = StatusModule::situation_kind(boma);

    if !StatusModule::is_changing(boma) {
        if StatusModule::is_situation_changed(boma) {
            
            if situation == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
            else {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }

            let motion = if situation == *SITUATION_KIND_GROUND {
                Hash40::new("special_s_impact")
            } else {
                Hash40::new("special_air_s_impact")
            };
            
            MotionModule::change_motion_inherit_frame(boma, motion, -1.0, 1.0, 0.0, false, false);
        }
    }

    if MotionModule::is_end(boma) {
        if situation == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }

    0.into()
}

pub fn install() {
    Agent::new("wario")
        .set_costume(is_metalwario())
        .status(Main, FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_IMPACT, special_s_impact_main)
        .install();
}