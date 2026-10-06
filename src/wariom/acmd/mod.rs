use smash::app;
use super::*;

mod ground;
mod throw;
mod areals;
mod specials;
mod tilts;
mod jabs;
mod smashes;

pub fn install() {
    ground::install();
    throw::install();
    areals::install();
    specials::install();
    tilts::install();
    jabs::install();
    smashes::install();
}