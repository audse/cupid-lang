use crate::arena::EntryId;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct SourceId(EntryId);

impl From<EntryId> for SourceId {
    fn from(value: EntryId) -> Self {
        Self(value)
    }
}

impl From<SourceId> for EntryId {
    fn from(value: SourceId) -> Self {
        value.0
    }
}
