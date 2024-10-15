use super::data_types;

pub fn unimplemented_instruction() {
    println!("Error: Unimplemented instruction\n");
    std::process::exit(1);
}

pub fn emulate_8080_op(state: &mut data_types::State8080) {
    let op_code = state.memory[state.pc as usize];
    match op_code {
        0x00..=0xff => {
            unimplemented_instruction();
        }
    }
    state.pc += 1;
}
