fn test_add_instruction() {
    let mut state = data_types::State8080::default();
    state.a = 0x14;
    state.b = 0x22;
    add(&mut state, data_types::Register::B);
    assert_eq!(state.a, 0x36);
    assert_eq!(state.cc.z, false);
    assert_eq!(state.cc.s, false);
    assert_eq!(state.cc.p, true);
    assert_eq!(state.cc.cy, false);
}
