use super::m128::{m128i};

#[allow(overflowing_literals)]
pub fn gen_program_aes_1rx4(input:[m128i;4], output_size: usize) -> Vec<m128i> {
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

#[allow(overflowing_literals)]
pub fn gen_program_aes_4rx4(input:[m128i;4], output_size: usize) -> Vec<m128i> {
    debug_assert!(output_size % 4 == 0);
    
    let mut result = Vec::with_capacity(output_size);
    
    let key0 = m128i::from_i32(0x99e5d23f, 0x2f546d2b, 0xd1833ddb, 0x6421aadd);
    let key1 = m128i::from_i32(0xa5dfcde5, 0x06f79d53, 0xb6913f55, 0xb20e3450);
    let	key2 = m128i::from_i32(0x171c02bf, 0x0aa4679f, 0x515e7baf, 0x5c3ed904);
    let	key3 = m128i::from_i32(0xd8ded291, 0xcd673785, 0xe78f5d08, 0x85623763);
    let	key4 = m128i::from_i32(0x229effb4, 0x3d518b6d, 0xe3d6a7a6, 0xb5826f73);
    let	key5 = m128i::from_i32(0xb272b7d2, 0xe9024d4e, 0x9c10b3d9, 0xc7566bf3);
    let	key6 = m128i::from_i32(0xf63befa7, 0x2ba9660a, 0xf765a38b, 0xf273c9e7);
    let	key7 = m128i::from_i32(0xc0b0762d, 0x0c06d1fd, 0x915839de, 0x7a7cd609);

    let mut state0 = input[0];
	let mut state1 = input[1];
	let mut state2 = input[2];
	let mut state3 = input[3];
    
    let mut out_ix = 0;
    while out_ix < output_size {
        state0 = state0.aesdec(key0);
        state1 = state1.aesenc(key0);
        state2 = state2.aesdec(key4);
        state3 = state3.aesenc(key4);
       
        state0 = state0.aesdec(key1);
        state1 = state1.aesenc(key1);
        state2 = state2.aesdec(key5);
        state3 = state3.aesenc(key5); 

        state0 = state0.aesdec(key2);
        state1 = state1.aesenc(key2);
        state2 = state2.aesdec(key6);
        state3 = state3.aesenc(key6);
        
        state0 = state0.aesdec(key3);
        state1 = state1.aesenc(key3);
        state2 = state2.aesdec(key7);
        state3 = state3.aesenc(key7); 

        result.push(state0);
        result.push(state1);
        result.push(state2);
        result.push(state3);

		out_ix = out_ix + 4;
	}
    return result;
}