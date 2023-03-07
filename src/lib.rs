use fil_actors_runtime::{ActorError, INIT_ACTOR_ADDR, actor_error, actor_dispatch, restrict_internal_api};
use fil_actors_runtime::runtime::{Runtime, ActorCode};
use fvm_ipld_encoding::{RawBytes, to_vec};
use fvm_shared::METHOD_CONSTRUCTOR;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use fvm_shared::MethodNum;

pub use crate::state::State;

mod state;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(Actor);

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    SayHello = 2,
}

pub trait HelloWorldActor {
    fn constructor(rt: &mut impl Runtime) -> Result<(), ActorError>;

    fn say_hello(rt: &mut impl Runtime) -> Result<Option<RawBytes>, ActorError>;
}

pub struct Actor;

impl HelloWorldActor for Actor {
    fn constructor(rt: &mut impl Runtime) -> Result<(), ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&INIT_ACTOR_ADDR))?;
        
        rt.create(&State {count: 0})?;

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
}

impl ActorCode for Actor {
    type Methods = Method;

    actor_dispatch! {
        Constructor => constructor,
        SayHello => say_hello,
    }
}