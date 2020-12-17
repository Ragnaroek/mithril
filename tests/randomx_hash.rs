extern crate mithril;

use mithril::randomx::hash::{gen_program_aes_1rx4, gen_program_aes_4rx4};
use mithril::randomx::m128::{m128i};

#[test]
#[allow(overflowing_literals)]
fn test_gen_program_aes_1rx4() {
    let input0 = m128i::from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let input1 = m128i::from_i32(0xb5a8ef67, 0x749809c8, 0xf349884a, 0x05c9f5ef);
    let input2 = m128i::from_i32(0xa9a93ab0, 0x22e46d0a, 0x1a1fe305, 0xb42708c0);
    let input3 = m128i::from_i32(0x68247034, 0xed99ee84, 0x438f563a, 0x138612ff);    
    let input:[m128i;4] = [input0, input1, input2, input3];
    
    let result = gen_program_aes_1rx4(input, 136);

    assert_eq!(result.len(), 136);
    assert_eq!(result[0],   m128i::from_i32(0x27117584, 0x121aeeb3, 0x2f620901, 0xf788e553));
    assert_eq!(result[1],   m128i::from_i32(0x7b1951c7, 0x2ca4ef19, 0xf09f9310, 0x248ffc66));
    assert_eq!(result[9],   m128i::from_i32(0xf31272c9, 0x187f3e37, 0x9ed29677, 0x9cb3cad8));
    assert_eq!(result[29],  m128i::from_i32(0xb979c03b, 0xf3851cd4, 0x8053d5c4, 0xf167e714));
    assert_eq!(result[59],  m128i::from_i32(0x9edf9671, 0x351efb59, 0x3cd79767, 0x15624b91));
    assert_eq!(result[79],  m128i::from_i32(0x36881166, 0xf619e667, 0xf728102c, 0x103e2d56));
    assert_eq!(result[99],  m128i::from_i32(0xdda1adbf, 0xec39dc8a, 0x89884695, 0xc61ff1dd));
    assert_eq!(result[135], m128i::from_i32(0x778d555d, 0x82dfe800, 0xedbe8cae, 0x2fe08b9f));
}

#[test]
#[allow(overflowing_literals)]
fn test_gen_program_aes_4rx4() {
    let input0 = m128i::from_i32(0xb53a90c9, 0xf56f1bc9, 0x25a4424b, 0x727ab1b2);
    let input1 = m128i::from_i32(0x70152fd1, 0x377f234d, 0xe8027504, 0xfed70bc4);
    let input2 = m128i::from_i32(0xae1f977a, 0x841fdb02, 0x85b20930, 0xf22cf15b);
    let input3 = m128i::from_i32(0x2fd5f11,  0x28e94c44, 0x8a756cec, 0x33c0d189);
    let input:[m128i;4] = [input0, input1, input2, input3];
    
    let result = gen_program_aes_4rx4(input, 136);

    assert_eq!(result.len(), 136);
    assert_eq!(result[0],   m128i::from_i32(0xf76214a7, 0xcbe6ca8a, 0x71e1f016, 0x44ba2d2d));
    assert_eq!(result[1],   m128i::from_i32(0x3c19a6ae, 0x6201dd3a, 0x22dfa1c7, 0x977f13a5));
    assert_eq!(result[9],   m128i::from_i32(0xbdf2934c, 0x381a2d03, 0xae553192, 0xb0e5bb9f));
    assert_eq!(result[29],  m128i::from_i32(0xcafa93b4, 0xad33c065, 0x980587f5, 0xd1ae0a40));
    assert_eq!(result[59],  m128i::from_i32(0x7dc0a58a, 0x77d4f8dc, 0x4eaecb7d, 0x3d052cb5));
    assert_eq!(result[79],  m128i::from_i32(0x525b195b, 0x2f51cfca, 0xd948a805, 0xfe9eed66));
    assert_eq!(result[99],  m128i::from_i32(0x4ab9f16b, 0xbd648e91, 0xda7d9069, 0x1d9f6716));
    assert_eq!(result[135], m128i::from_i32(0x3f7fdb2f, 0x565cd0c7, 0xbe72f8e3, 0x5da409a1));
}