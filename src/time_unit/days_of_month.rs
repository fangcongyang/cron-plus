use crate::ordinal::{Ordinal, OrdinalSet, Pattern};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;
use once_cell::sync::Lazy;

static ALL: Lazy<OrdinalSet> = Lazy::new(|| { DaysOfMonth::supported_ordinals() });

#[derive(Clone, Debug, Eq)]
pub struct DaysOfMonth{
    ordinals: Option<OrdinalSet>,
    pattern: Pattern
}

impl TimeUnitField for DaysOfMonth {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, pattern: Pattern) -> Self {
        DaysOfMonth {
            ordinals: ordinal_set,
            pattern
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Days of Month")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        31
    }
    fn ordinals(&self) -> &OrdinalSet {
        match &self.ordinals {
            Some(ordinal_set) => ordinal_set,
            None => &ALL
        }
    }
    fn matching_pattern(&self) -> &str {
        &self.pattern
    }
}

impl PartialEq for DaysOfMonth {
    fn eq(&self, other: &DaysOfMonth) -> bool {
        self.ordinals() == other.ordinals()
    }
}