use crate::ordinal::{Ordinal, OrdinalSet, Pattern};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;
use once_cell::sync::Lazy;

static ALL: Lazy<OrdinalSet> = Lazy::new(|| { Hours::supported_ordinals() });

#[derive(Clone, Debug, Eq)]
pub struct Hours{
    ordinals: Option<OrdinalSet>,
    pattern: Pattern
}

impl TimeUnitField for Hours {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, pattern: Pattern) -> Self {
        Hours{
            ordinals: ordinal_set,
            pattern
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Hours")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        23
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

impl PartialEq for Hours {
    fn eq(&self, other: &Hours) -> bool {
        self.ordinals() == other.ordinals()
    }
}