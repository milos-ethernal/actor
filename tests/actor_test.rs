#[cfg(test)]
mod test {
    use actor::{Actor, Method, State};
    use fil_actors_runtime::INIT_ACTOR_ADDR;
    use fil_actors_runtime::test_utils::{MockRuntime, INIT_ACTOR_CODE_ID};
    use fvm_ipld_encoding::{RawBytes, to_vec};
    use fvm_shared::address::Address;

    pub fn new_runtime(receiver: Address) -> MockRuntime {
        MockRuntime {
            receiver,
            caller: INIT_ACTOR_ADDR,
            caller_type: *INIT_ACTOR_CODE_ID,
            ..Default::default()
        }
    }

    fn construct_runtime_with_receiver(receiver: Address) -> MockRuntime {
        let mut runtime = new_runtime(receiver);
        runtime.set_caller(*INIT_ACTOR_CODE_ID, INIT_ACTOR_ADDR);

        runtime.expect_validate_caller_addr(vec![INIT_ACTOR_ADDR]);

        runtime
            .call::<Actor>(
                Method::Constructor as u64,
                None,
            )
            .unwrap();

        runtime
    }

    fn construct_runtime() -> MockRuntime {
        let receiver = Address::new_id(1);
        construct_runtime_with_receiver(receiver)
    }

    #[test]
    fn test_constructor() {
        let runtime = construct_runtime();
        assert_eq!(runtime.state.is_some(), true);

        let state: State = runtime.get_state();
        assert_eq!(state.count, 0);
    }

    #[test]
    fn test_say_hello() {
        let mut runtime = construct_runtime();

        let st: State = runtime.get_state();
        assert_eq!(st.count, 0);

        runtime.expect_validate_caller_any();

        //Expects string Hello World#1! in RawBytes
        let mut resp = runtime
            .call::<Actor>(
                Method::SayHello as u64,
                None
            ).unwrap();
        
        
        assert_eq!(resp.unwrap().deserialize::<RawBytes>().unwrap(), RawBytes::new(to_vec("Hello world #1!").unwrap()));
        let st: State = runtime.get_state();
        assert_eq!(st.count, 1);

        runtime.expect_validate_caller_any();

        //Expects string Hello World#2! in RawBytes
        resp = runtime
            .call::<Actor>(
                Method::SayHello as u64,
                None
            ).unwrap();
        
        assert_eq!(resp.unwrap().deserialize::<RawBytes>().unwrap(), RawBytes::new(to_vec("Hello world #2!").unwrap()));
        let st: State = runtime.get_state();
        assert_eq!(st.count, 2);
    }

}