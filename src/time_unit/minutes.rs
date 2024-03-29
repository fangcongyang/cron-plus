use crate::ordinal::{Ordinal, OrdinalSet, Pattern, OrdinalList};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;
use once_cell::sync::Lazy;

static ALL: Lazy<OrdinalSet> = Lazy::new(|| { Minutes::supported_ordinals() });

#[derive(Clone, Debug, Eq)]
pub struct Minutes{
    ordinals: Option<OrdinalSet>, 
    pattern: Pattern,
    ordinal_list: OrdinalList
}

impl TimeUnitField for Minutes {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, pattern: Pattern, ordinal_list: OrdinalList) -> Self {
        Minutes{
            ordinals: ordinal_set,
            pattern,
            ordinal_list
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Minutes")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        59
    }
    fn ordinals(&self) -> &OrdinalSet {
        match &self.ordinals {
            Some(ordinal_set) => ordinal_set,
            None => &ALL
        }
    }
    fn ordinal_list(&self) -> &OrdinalList {
        &self.ordinal_list
    }
    fn matching_pattern(&self) -> &str {
        &self.pattern
    }
}

impl PartialEq for Minutes {
    fn eq(&self, other: &Minutes) -> bool {
        self.ordinals() == other.ordinals()
    }
}