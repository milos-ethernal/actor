use fil_actors_runtime::{ActorError, INIT_ACTOR_ADDR, actor_error, actor_dispatch, restrict_internal_api, ActorDowncast};
use fil_actors_runtime::runtime::{Runtime, ActorCode};
use fvm_ipld_encoding::{RawBytes, to_vec};
use fvm_shared::METHOD_CONSTRUCTOR;
use fvm_shared::error::ExitCode;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use fvm_shared::MethodNum;

pub use crate::state::State;
pub use crate::types::*;

mod state;
pub mod types;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(Actor);

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    SayHello = 2,
    GetValue = 3,
    SetValue = 4,
}

pub trait HelloWorldActor {
    fn constructor(rt: &mut impl Runtime) -> Result<(), ActorError>;

    fn say_hello(rt: &mut impl Runtime) -> Result<Option<RawBytes>, ActorError>;

    fn get_value(rt: &mut impl Runtime, key: u64) -> Result<Option<RawBytes>, ActorError>;

    fn set_value(rt: &mut impl Runtime, params: KeyValueType) -> Result<(), ActorError>;
}

pub struct Actor;

impl HelloWorldActor for Actor {
    fn constructor(rt: &mut impl Runtime) -> Result<(), ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&INIT_ACTOR_ADDR))?;
        
        let state = State::new(rt.store()).map_err(|e| {
            e.downcast_default(ExitCode::USR_ILLEGAL_STATE, "Failed to create actor state")
        })?;

        rt.create(&state)?;

        Ok(())
    }

    fn say_hello(rt: &mut impl Runtime) -> Result<Option<RawBytes>, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        return rt.transaction(|st: &mut State, _| {
            st.increment();

            let ret = to_vec(format!("Hello world #{}!", &st.count).as_str());
            match ret {
                Ok(ret) => Ok(Some(RawBytes::new(ret))),
                Err(_) => {
                    Err(actor_error!(illegal_state, "Error in executing SayHello"))
                }
            }
        });
    }

    fn get_value(rt: &mut impl Runtime, key: u64) -> Result<Option<RawBytes>, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        return rt.transaction(|st: &mut State, rt| {
            let ret = st.get_value(rt.store(), key);
            match ret {
                Ok(ret) => Ok(Some(RawBytes::serialize(ret)?)),
                Err(e) => Err(e),
            }
        });
    }

    fn set_value(rt: &mut impl Runtime, params: KeyValueType) -> Result<(), ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        return rt.transaction(|st: &mut State, rt| {
            let ret = st.set_value(rt.store(), params.key, params.value);
            match ret {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        });
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    actor_dispatch! {
        Constructor => constructor,
        SayHello => say_hello,
        GetValue => get_value,
        SetValue => set_value,
    }
}