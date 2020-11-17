use super::m128::{m128i};

#[allow(overflowing_literals)]
pub fn gen_program(input:[m128i;4], output_size: usize) -> Vec<m128i> {
    debug_assert!(output_size % 4 == 0);
    
    let mut result = Vec::with_capacity(output_size);
    
    let key0 = m128i::from_i32(0xb4f44917, 0xdbb5552b, 0x62716609, 0x6daca553);
    let key1 = m128i::from_i32(0x0da1dc4e, 0x1725d378, 0x846a710d, 0x6d7caf07);
    let	key2 = m128i::from_i32(0x3e20e345, 0xf4c0794f, 0x9f947ec6, 0x3f1262f1);
    let	key3 = m128i::from_i32(0x49169154, 0x16314c88, 0xb1ba317c, 0x6aef8135);
    
    let mut state0 = input[0];
	let mut state1 = input[1];
	let mut state2 = input[2];
	let mut state3 = input[3];
    
    let mut out_ix = 0;
    while out_ix < output_size {
        state0 = state0.aesdec(key0);
        state1 = state1.aesenc(key1);
        state2 = state2.aesdec(key2);
        state3 = state3.aesenc(key3);
        
        result.push(state0);
        result.push(state1);
        result.push(state2);
        result.push(state3);

		out_ix = out_ix + 4;
	}
    return result;
}
