use super::*;

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let situation = StatusModule::situation_kind(boma);

    if !StatusModule::is_changing(boma) {
        if StatusModule::is_situation_changed(boma) {
            if situation == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            } else {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }

            let motion = if situation == *SITUATION_KIND_GROUND {
                Hash40::new("special_s_start")
            } else {
                Hash40::new("special_air_s_start")
            };

            MotionModule::change_motion_inherit_frame(boma, motion, -1.0, 1.0, 0.0, false, false);
        }
    }

    if (lr > 0.0 && GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32))
    || (lr < 0.0 && GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32))
    {
        if GroundModule::can_entry_cliff(boma) == 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_STOP_WALL.into(), false.into());
            return 1.into();
        }
    }

    if AttackModule::is_attack_occur(boma) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_IMPACT.into(), false.into());
        return 0.into();
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
            .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, special_s_main)
            .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, special_s_main)
        .install();
}