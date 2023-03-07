use fil_actors_runtime::{ActorError, actor_error};
use fil_actors_runtime::fvm_ipld_hamt::BytesKey;
use fvm_ipld_blockstore::Blockstore;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use primitives::{TCid, THamt};

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct State {
    pub count: u64,
    pub map: TCid<THamt<u64, u64>>,
}

impl State {
    pub fn new<BS: Blockstore>(store: &BS) -> anyhow::Result<State> {
        let state = State {
            count: 0,
            map: TCid::new_hamt(store)?
        };

        Ok(state)
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn get_value<BS: Blockstore>(
        &self,
        store: &BS,
        key: u64
    ) -> Result<Option<u64>, ActorError>
     {
        let hamt = self.map
            .load(store)
            .map_err(|_| actor_error!(illegal_state, "Cannot load map"))?;
        let value = hamt
            .get(&BytesKey::from(&key.to_string()[..]))
            .map_err(|_| actor_error!(illegal_state, "Cannot get value from hamt"))?;
        Ok(value.copied())
    }

    pub fn set_value<BS: Blockstore>(
        &mut self,
        store: &BS,
        key: u64,
        value: u64
    ) -> Result<(), ActorError> {
        self.map
            .modify(store, |hamt| {
                hamt.set(BytesKey::from(&key.to_string()[..]), value)
                    .map_err(|_| actor_error!(illegal_state, "Cannot set value in hamt"))?;
                Ok(())
            })
            .map_err(|_| actor_error!(illegal_state, "Cannot modify map"))?;
        Ok(())
    }
}
