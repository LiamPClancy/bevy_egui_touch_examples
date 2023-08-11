use std::collections::HashMap;

use pm_pattern_logic::{Pattern, BodyMeasurementKind, BodyMeasurement};

pub (crate) fn get_startup_pattern(bm : &HashMap<BodyMeasurementKind, BodyMeasurement>) -> Pattern{
    Pattern::new(bm)
}