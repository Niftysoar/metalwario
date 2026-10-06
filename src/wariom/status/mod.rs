use smash::app;
use super::*;

mod special_s;
mod special_s_impact;

pub fn install() {
    special_s::install();
    special_s_impact::install();
}