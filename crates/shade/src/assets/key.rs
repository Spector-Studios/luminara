use std::{fmt::Debug, hash::Hash, marker::PhantomData};

#[repr(transparent)]
pub(super) struct AssetKey<T>(slotmap::KeyData, PhantomData<T>);

impl<T> slotmap::__impl::From<slotmap::KeyData> for AssetKey<T> {
    fn from(k: slotmap::KeyData) -> Self {
        AssetKey(k, PhantomData)
    }
}
unsafe impl<T> slotmap::Key for AssetKey<T> {
    fn data(&self) -> slotmap::KeyData {
        self.0
    }
}

slotmap::__serialize_key!(AssetKey);
slotmap::new_key_type!();

impl<T> Clone for AssetKey<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AssetKey<T> {}

impl<T> Default for AssetKey<T> {
    fn default() -> Self {
        Self(slotmap::KeyData::default(), PhantomData)
    }
}

impl<T> PartialEq for AssetKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T> Eq for AssetKey<T> {}
impl<T> PartialOrd for AssetKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for AssetKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for AssetKey<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Debug for AssetKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
