use super::super::callback::standard_control_callbacks;
use super::super::persistence::batch_deserializer_local::BatchDeserializerLocal;
use gol_core::{BoardCallbackManager, IndexedDataOwned, StatesCallback, StatesReadOnly};
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::hash::Hash;

pub struct ReplayerLocal<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    deserializer: BatchDeserializerLocal<U, Vec<IndexedDataOwned<CI, T>>>,
    states: StatesCallback<CI, T>,
    controls: BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>,
}

impl<T, CI, U> ReplayerLocal<T, CI, U>
where
    T: Send + Sync,
    CI: Send + Sync + Hash,
{
    pub fn new(trivial_state: T, history_path: &String) -> Self
    where
        T: 'static + Clone + DeserializeOwned,
        CI: 'static + Clone + DeserializeOwned,
        U: Send + Sync + DeserializeOwned,
    {
        let deserializer = BatchDeserializerLocal::new(history_path);
        let states = StatesCallback::new(trivial_state);
        let (callbacks, _keyboard_control) =
            standard_control_callbacks(true, std::time::Duration::new(1, 0));
        let callback_manager = BoardCallbackManager::new(callbacks);
        Self {
            deserializer,
            states,
            controls: callback_manager,
        }
    }
}
