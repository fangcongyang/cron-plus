use crate::ordinal::{Ordinal, OrdinalSet, Pattern, OrdinalList};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;
use once_cell::sync::Lazy;

static ALL: Lazy<OrdinalSet> = Lazy::new(|| { Seconds::supported_ordinals() });

#[derive(Clone, Debug, Eq)]
pub struct Seconds {
    ordinals: Option<OrdinalSet>,
    pattern: Pattern,
    ordinal_list: OrdinalList,
}

impl TimeUnitField for Seconds {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, pattern: Pattern, ordinal_list: OrdinalList) -> Self {
        Seconds{
            ordinals: ordinal_set,
            pattern,
            ordinal_list
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Seconds")
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

impl PartialEq for Seconds {
    fn eq(&self, other: &Seconds) -> bool {
        self.ordinals() == other.ordinals()
    }
}