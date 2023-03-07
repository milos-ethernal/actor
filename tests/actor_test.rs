#[cfg(test)]
mod test {
    use actor::{Actor, Method, State, KeyValueType};
    use fil_actors_runtime::INIT_ACTOR_ADDR;
    use fil_actors_runtime::test_utils::{MockRuntime, INIT_ACTOR_CODE_ID};
    use fvm_ipld_encoding::ipld_block::IpldBlock;
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
        assert!(runtime.state.is_some());

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

    #[test]
    fn test_hamt() {
        let mut runtime = construct_runtime();
        runtime.expect_validate_caller_any();

        // Expect None when key is not present in map
        let mut resp = runtime
            .call::<Actor>(
                Method::GetValue as u64,
                IpldBlock::serialize_cbor(&5).unwrap()
            ).unwrap();
        
        let none_ret: Option<u64> = None;
        assert_eq!(resp.unwrap().deserialize::<RawBytes>().unwrap(), RawBytes::serialize(none_ret).unwrap());

        //Add (1,2) to map and check
        let params = KeyValueType {
            key: 1,
            value: 2,
        };

        runtime.expect_validate_caller_any();
        runtime
            .call::<Actor>(
                Method::SetValue as u64,
                IpldBlock::serialize_cbor(&params).unwrap()
        ).unwrap();

        runtime.expect_validate_caller_any();
        resp = runtime
            .call::<Actor>(
                Method::GetValue as u64,
                IpldBlock::serialize_cbor(&1).unwrap()
            ).unwrap();

        assert_eq!(resp.unwrap().deserialize::<RawBytes>().unwrap(), RawBytes::serialize(&2).unwrap());
    }
}