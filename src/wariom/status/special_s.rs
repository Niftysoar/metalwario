use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    } else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    KineticModule::clear_speed_all(boma);
    0.into()
}

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
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
            } else {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
            }
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
            .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, special_s_pre)
            .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, special_s_init)
            .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, special_s_main)

            .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, special_s_pre)
            .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, special_s_init)
            .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, special_s_main)
        .install();
}