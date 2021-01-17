pub type IndexedDataRef<'dref, CI, T> = (CI, &'dref T);
pub type IndexedDataOwned<CI, T> = (CI, T);
