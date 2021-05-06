extern crate blake2b_simd;
extern crate mithril;

use self::blake2b_simd::blake2b;
use mithril::byte_string::{string_to_u8_array, u8_array_to_string};
use mithril::randomx::common::randomx_reciprocal;
use mithril::randomx::hash::gen_program_aes_4rx4;
use mithril::randomx::m128::m128d;
use mithril::randomx::memory::VmMemory;
use mithril::randomx::program::{
    a_reg, e_reg, f_reg, r_reg, Instr, Mode, Opcode, Program, Store, REG_NEEDS_DISPLACEMENT,
    REG_NEEDS_DISPLACEMENT_IX,
};
use mithril::randomx::vm::{hash_to_m128i_array, new_register, new_vm, Vm};
use std::sync::Arc;

#[allow(overflowing_literals)]
const IMM32: i32 = 0xc0cb96d2; //3234567890
const IMM64: u64 = 0xffffffffc0cb96d2;
const ROUND_TO_NEAREST: u32 = 0;
const ROUND_DOWN: u32 = 1;
const ROUND_UP: u32 = 2;
const ROUND_TO_ZERO: u32 = 3;

#[test]
fn test_calculate_hash_1_with_light_memory() {
    let mut vm = new_vm(Arc::new(VmMemory::light(b"test key 000")));
    let result = vm.calculate_hash(b"This is a test");
    assert_eq!(
        "639183aae1bf4c9a35884cb46b09cad9175f04efd7684e7262a0ac1c2f0b4e3f",
        u8_array_to_string(result.as_bytes())
    );

    let result = vm.calculate_hash(b"Lorem ipsum dolor sit amet");
    assert_eq!(
        "300a0adb47603dedb42228ccb2b211104f4da45af709cd7547cd049e9489c969",
        u8_array_to_string(result.as_bytes())
    );

    let result =
        vm.calculate_hash(b"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua");
    assert_eq!(
        "c36d4ed4191e617309867ed66a443be4075014e2b061bcdaf9ce7b721d2b77a8",
        u8_array_to_string(result.as_bytes())
    );
}

#[test]
fn test_calculate_hash_1_with_full_memory() {
    let mut vm = new_vm(Arc::new(VmMemory::full(b"test key 000")));
    let result = vm.calculate_hash(b"This is a test");
    assert_eq!(
        "639183aae1bf4c9a35884cb46b09cad9175f04efd7684e7262a0ac1c2f0b4e3f",
        u8_array_to_string(result.as_bytes())
    );

    let result = vm.calculate_hash(b"Lorem ipsum dolor sit amet");
    assert_eq!(
        "300a0adb47603dedb42228ccb2b211104f4da45af709cd7547cd049e9489c969",
        u8_array_to_string(result.as_bytes())
    );

    let result =
        vm.calculate_hash(b"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua");
    assert_eq!(
        "c36d4ed4191e617309867ed66a443be4075014e2b061bcdaf9ce7b721d2b77a8",
        u8_array_to_string(result.as_bytes())
    );
}

#[test]
fn test_calculate_hash_2_with_light_memory() {
    let mut vm = new_vm(Arc::new(VmMemory::light(b"test key 001")));

    let result =
        vm.calculate_hash(b"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua");
    assert_eq!(
        "e9ff4503201c0c2cca26d285c93ae883f9b1d30c9eb240b820756f2d5a7905fc",
        u8_array_to_string(result.as_bytes())
    );

    let seed = string_to_u8_array("0b0b98bea7e805e0010a2126d287a2a0cc833d312cb786385a7c2f9de69d25537f584a9bc9977b00000000666fd8753bf61a8631f12984e3fd44f4014eca629276817b56f32e9b68bd82f416");
    let result = vm.calculate_hash(&seed);
    assert_eq!(
        "c56414121acda1713c2f2a819d8ae38aed7c80c35c2a769298d34f03833cd5f1",
        u8_array_to_string(result.as_bytes())
    );
}

#[test]
fn test_calculate_hash_2_with_full_memory() {
    let mut vm = new_vm(Arc::new(VmMemory::full(b"test key 001")));

    let result =
        vm.calculate_hash(b"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua");
    assert_eq!(
        "e9ff4503201c0c2cca26d285c93ae883f9b1d30c9eb240b820756f2d5a7905fc",
        u8_array_to_string(result.as_bytes())
    );

    let seed = string_to_u8_array("0b0b98bea7e805e0010a2126d287a2a0cc833d312cb786385a7c2f9de69d25537f584a9bc9977b00000000666fd8753bf61a8631f12984e3fd44f4014eca629276817b56f32e9b68bd82f416");
    let result = vm.calculate_hash(&seed);
    assert_eq!(
        "c56414121acda1713c2f2a819d8ae38aed7c80c35c2a769298d34f03833cd5f1",
        u8_array_to_string(result.as_bytes())
    );
}

//Bugfix Test
#[test]
fn test_calculate_hash_3_with_full_memory() {
    let mut vm = new_vm(Arc::new(VmMemory::light(&string_to_u8_array(
        "15564c3122550436919ac2f8a71baf7cbaf9a4117b842d7f2b19dfd27dd178e9",
    ))));

    /*
    let seed = string_to_u8_array("0e0eace28a84066e289987b45d30ad6b588623c490a67d714edfa25f98e71d51a66db481e257c5000015f5f01895b55b96a383f87c32eded92bc469df6c11ceca976d3016a3abc467cf0c007");
    let result = vm.calculate_hash(&seed);
    assert_eq!(
        "90138a49c1982fb72e1fcbafc2102e0068e16ea97ec2e7ef804c7c62ec520400",
        u8_array_to_string(result.as_bytes())
    );

    let seed = string_to_u8_array("0e0eace28a84066e289987b45d30ad6b588623c490a67d714edfa25f98e71d51a66db481e257c5000015f1f01895b55b96a383f87c32eded92bc469df6c11ceca976d3016a3abc467cf0c007");
    let result = vm.calculate_hash(&seed);
    assert_eq!(
        "ab7397afa1d1bb7773e4cc20f5234a2e443da9f1d6dcfe3ed856a8d78b95b3b4",
        u8_array_to_string(result.as_bytes())
    );*/

    let seed = string_to_u8_array("0e0e8bb48b8406bf43039198b7712a35031e0607036ebf9afb3096977e7b8fb88c751430e96b02000006ad82bd221c5e282d0533c5dcca38f30babc2e62cd3aa03a965f8aec8ad6f129f5211");
    let result = vm.calculate_hash(&seed);
    assert_eq!(
        "312a2ef18681e7b065f87e56b2627f0a11e19b30415314efa898a13f407f5d08",
        u8_array_to_string(result.as_bytes())
    );
}

#[test]
fn test_init_scratchpad() {
    let mut vm = new_test_vm();
    let hash = blake2b("This is a test".as_bytes());
    vm.init_scratchpad(&hash_to_m128i_array(&hash));
    //sample test scratchpad layout
    assert_eq!(vm.scratchpad[0], 0x45a1b4e3e7fea6c);
    assert_eq!(vm.scratchpad[1], 0xe287d43cd65fd299);
    assert_eq!(vm.scratchpad[2], 0xbb1f8ec38ad6bcef);
    assert_eq!(vm.scratchpad[3], 0xc138a9a5c95e7b0f);
    assert_eq!(vm.scratchpad[4], 0x5cb93a85f06ef6e8);
    assert_eq!(vm.scratchpad[5], 0x6db2f212bf8390f8);
    assert_eq!(vm.scratchpad[6], 0x742a671fe69f28ab);
    assert_eq!(vm.scratchpad[7], 0xd6eb5539a8b4e48f);

    assert_eq!(vm.scratchpad[33333], 0x5b85caaea16199bf);
    assert_eq!(vm.scratchpad[66666], 0x3b35256a8a5afc64);
    assert_eq!(vm.scratchpad[131071], 0xc87ac0bce6ef30e8);
    assert_eq!(vm.scratchpad[191000], 0xf5e560770bdd6a4f);
    assert_eq!(vm.scratchpad[262142], 0x2e417916bf21fc05);
    assert_eq!(vm.scratchpad[262143], 0x66db274303c4fd4);
}

#[test]
fn test_init_vm() {
    let mut vm = new_test_vm();

    let hash = blake2b("This is a test".as_bytes());
    let seed = hash_to_m128i_array(&hash);
    let seed = vm.init_scratchpad(&seed);
    let prog = Program::from_bytes(gen_program_aes_4rx4(&seed, 136));
    vm.init_vm(&prog);

    assert_eq!(
        vm.reg.a[0].as_u64(),
        (0x4019c856c26708a9, 0x418e4a297ebfc304)
    );
    assert_eq!(
        vm.reg.a[1].as_u64(),
        (0x41e807a5dc7740b5, 0x40cd8725df13238a)
    );
    assert_eq!(
        vm.reg.a[2].as_u64(),
        (0x417112c274f91d68, 0x4176971a789beed7)
    );
    assert_eq!(
        vm.reg.a[3].as_u64(),
        (0x40bd229eeedd8e98, 0x414e441747df76c6)
    );

    assert_eq!(vm.config.e_mask[0], 0x3c000000001e145f);
    assert_eq!(vm.config.e_mask[1], 0x3a0000000011d432);

    assert_eq!(vm.config.read_reg[0], 0);
    assert_eq!(vm.config.read_reg[1], 3);
    assert_eq!(vm.config.read_reg[2], 5);
    assert_eq!(vm.config.read_reg[3], 7);

    assert_eq!(vm.mem_reg.ma, 0x738ddb40);
    assert_eq!(vm.mem_reg.mx, 0x8a8a6230);
}

#[test]
fn test_register_to_bytes() {
    let mut reg = new_register();
    reg.r[0] = 0x34ffebd12d810880;
    reg.r[1] = 0x6a80260a6208adef;
    reg.r[2] = 0x4f5d1008ee3b292f;
    reg.r[3] = 0xb65180d5769c17d0;
    reg.r[4] = 0x2695aed734fdb28;
    reg.r[5] = 0x3c6a84d4c01ddff5;
    reg.r[6] = 0xa9d93cadfd06d699;
    reg.r[7] = 0xc8ae2f0947643d9;
    reg.f[0] = m128d::from_u64(0x8436536b210b2639, 0x856723cf061d0955);
    reg.f[1] = m128d::from_u64(0x8327712703bab8b8, 0xfe381f5303432413);
    reg.f[2] = m128d::from_u64(0x9213c07a21421d21, 0x928b31fb36ecba0a);
    reg.f[3] = m128d::from_u64(0x82886513418828bb, 0x7ebb3de2ae60d7f4);
    reg.e[0] = m128d::from_u64(0x45ea59134401e457, 0x44850ec11d8a94c7);
    reg.e[1] = m128d::from_u64(0x428870e600b31bd8, 0x3fea167cf9422f28);
    reg.e[2] = m128d::from_u64(0x53dd0cedf7e2d75e, 0x53c16a0c2972cc15);
    reg.e[3] = m128d::from_u64(0x4379fad7dcb15a7d, 0x3f6980958c0ab574);
    reg.a[0] = m128d::from_u64(0xd14dcee38dfdc313, 0x452bdbf00bb500dc);
    reg.a[1] = m128d::from_u64(0x863af2ea80c745a7, 0x3be75a066e67b2e3);
    reg.a[2] = m128d::from_u64(0x94ff8c6994073d88, 0xdc24859a54929d04);
    reg.a[3] = m128d::from_u64(0xe725aa19567fa59c, 0x4b7f3597f285ef34);

    let bytes = reg.to_bytes();

    assert_eq!(bytes[0], 0x80);
    assert_eq!(bytes[1], 0x8);
    assert_eq!(bytes[2], 0x81);
    assert_eq!(bytes[3], 0x2d);
    assert_eq!(bytes[4], 0xd1);
    assert_eq!(bytes[5], 0xeb);
    assert_eq!(bytes[6], 0xff);
    assert_eq!(bytes[7], 0x34);
    assert_eq!(bytes[8], 0xef);
    assert_eq!(bytes[9], 0xad);
    assert_eq!(bytes[10], 0x8);
    assert_eq!(bytes[11], 0x62);
    assert_eq!(bytes[12], 0xa);
    assert_eq!(bytes[13], 0x26);
    assert_eq!(bytes[14], 0x80);
    assert_eq!(bytes[15], 0x6a);
    assert_eq!(bytes[16], 0x2f);
    assert_eq!(bytes[17], 0x29);
    assert_eq!(bytes[18], 0x3b);
    assert_eq!(bytes[19], 0xee);
    assert_eq!(bytes[20], 0x8);
    assert_eq!(bytes[21], 0x10);
    assert_eq!(bytes[22], 0x5d);
    assert_eq!(bytes[23], 0x4f);
    assert_eq!(bytes[24], 0xd0);
    assert_eq!(bytes[25], 0x17);
    assert_eq!(bytes[26], 0x9c);
    assert_eq!(bytes[27], 0x76);
    assert_eq!(bytes[28], 0xd5);
    assert_eq!(bytes[29], 0x80);
    assert_eq!(bytes[30], 0x51);
    assert_eq!(bytes[31], 0xb6);
    assert_eq!(bytes[32], 0x28);
    assert_eq!(bytes[33], 0xdb);
    assert_eq!(bytes[34], 0x4f);
    assert_eq!(bytes[35], 0x73);
    assert_eq!(bytes[36], 0xed);
    assert_eq!(bytes[37], 0x5a);
    assert_eq!(bytes[38], 0x69);
    assert_eq!(bytes[39], 0x2);
    assert_eq!(bytes[40], 0xf5);
    assert_eq!(bytes[41], 0xdf);
    assert_eq!(bytes[42], 0x1d);
    assert_eq!(bytes[43], 0xc0);
    assert_eq!(bytes[44], 0xd4);
    assert_eq!(bytes[45], 0x84);
    assert_eq!(bytes[46], 0x6a);
    assert_eq!(bytes[47], 0x3c);
    assert_eq!(bytes[48], 0x99);
    assert_eq!(bytes[49], 0xd6);
    assert_eq!(bytes[50], 0x6);
    assert_eq!(bytes[51], 0xfd);
    assert_eq!(bytes[52], 0xad);
    assert_eq!(bytes[53], 0x3c);
    assert_eq!(bytes[54], 0xd9);
    assert_eq!(bytes[55], 0xa9);
    assert_eq!(bytes[56], 0xd9);
    assert_eq!(bytes[57], 0x43);
    assert_eq!(bytes[58], 0x76);
    assert_eq!(bytes[59], 0x94);
    assert_eq!(bytes[60], 0xf0);
    assert_eq!(bytes[61], 0xe2);
    assert_eq!(bytes[62], 0x8a);
    assert_eq!(bytes[63], 0xc);
    assert_eq!(bytes[64], 0x55);
    assert_eq!(bytes[65], 0x9);
    assert_eq!(bytes[66], 0x1d);
    assert_eq!(bytes[67], 0x6);
    assert_eq!(bytes[68], 0xcf);
    assert_eq!(bytes[69], 0x23);
    assert_eq!(bytes[70], 0x67);
    assert_eq!(bytes[71], 0x85);
    assert_eq!(bytes[72], 0x39);
    assert_eq!(bytes[73], 0x26);
    assert_eq!(bytes[74], 0xb);
    assert_eq!(bytes[75], 0x21);
    assert_eq!(bytes[76], 0x6b);
    assert_eq!(bytes[77], 0x53);
    assert_eq!(bytes[78], 0x36);
    assert_eq!(bytes[79], 0x84);
    assert_eq!(bytes[80], 0x13);
    assert_eq!(bytes[81], 0x24);
    assert_eq!(bytes[82], 0x43);
    assert_eq!(bytes[83], 0x3);
    assert_eq!(bytes[84], 0x53);
    assert_eq!(bytes[85], 0x1f);
    assert_eq!(bytes[86], 0x38);
    assert_eq!(bytes[87], 0xfe);
    assert_eq!(bytes[88], 0xb8);
    assert_eq!(bytes[89], 0xb8);
    assert_eq!(bytes[90], 0xba);
    assert_eq!(bytes[91], 0x3);
    assert_eq!(bytes[92], 0x27);
    assert_eq!(bytes[93], 0x71);
    assert_eq!(bytes[94], 0x27);
    assert_eq!(bytes[95], 0x83);
    assert_eq!(bytes[96], 0xa);
    assert_eq!(bytes[97], 0xba);
    assert_eq!(bytes[98], 0xec);
    assert_eq!(bytes[99], 0x36);
    assert_eq!(bytes[100], 0xfb);
    assert_eq!(bytes[101], 0x31);
    assert_eq!(bytes[102], 0x8b);
    assert_eq!(bytes[103], 0x92);
    assert_eq!(bytes[104], 0x21);
    assert_eq!(bytes[105], 0x1d);
    assert_eq!(bytes[106], 0x42);
    assert_eq!(bytes[107], 0x21);
    assert_eq!(bytes[108], 0x7a);
    assert_eq!(bytes[109], 0xc0);
    assert_eq!(bytes[110], 0x13);
    assert_eq!(bytes[111], 0x92);
    assert_eq!(bytes[112], 0xf4);
    assert_eq!(bytes[113], 0xd7);
    assert_eq!(bytes[114], 0x60);
    assert_eq!(bytes[115], 0xae);
    assert_eq!(bytes[116], 0xe2);
    assert_eq!(bytes[117], 0x3d);
    assert_eq!(bytes[118], 0xbb);
    assert_eq!(bytes[119], 0x7e);
    assert_eq!(bytes[120], 0xbb);
    assert_eq!(bytes[121], 0x28);
    assert_eq!(bytes[122], 0x88);
    assert_eq!(bytes[123], 0x41);
    assert_eq!(bytes[124], 0x13);
    assert_eq!(bytes[125], 0x65);
    assert_eq!(bytes[126], 0x88);
    assert_eq!(bytes[127], 0x82);
    assert_eq!(bytes[128], 0xc7);
    assert_eq!(bytes[129], 0x94);
    assert_eq!(bytes[130], 0x8a);
    assert_eq!(bytes[131], 0x1d);
    assert_eq!(bytes[132], 0xc1);
    assert_eq!(bytes[133], 0xe);
    assert_eq!(bytes[134], 0x85);
    assert_eq!(bytes[135], 0x44);
    assert_eq!(bytes[136], 0x57);
    assert_eq!(bytes[137], 0xe4);
    assert_eq!(bytes[138], 0x1);
    assert_eq!(bytes[139], 0x44);
    assert_eq!(bytes[140], 0x13);
    assert_eq!(bytes[141], 0x59);
    assert_eq!(bytes[142], 0xea);
    assert_eq!(bytes[143], 0x45);
    assert_eq!(bytes[144], 0x28);
    assert_eq!(bytes[145], 0x2f);
    assert_eq!(bytes[146], 0x42);
    assert_eq!(bytes[147], 0xf9);
    assert_eq!(bytes[148], 0x7c);
    assert_eq!(bytes[149], 0x16);
    assert_eq!(bytes[150], 0xea);
    assert_eq!(bytes[151], 0x3f);
    assert_eq!(bytes[152], 0xd8);
    assert_eq!(bytes[153], 0x1b);
    assert_eq!(bytes[154], 0xb3);
    assert_eq!(bytes[155], 0x0);
    assert_eq!(bytes[156], 0xe6);
    assert_eq!(bytes[157], 0x70);
    assert_eq!(bytes[158], 0x88);
    assert_eq!(bytes[159], 0x42);
    assert_eq!(bytes[160], 0x15);
    assert_eq!(bytes[161], 0xcc);
    assert_eq!(bytes[162], 0x72);
    assert_eq!(bytes[163], 0x29);
    assert_eq!(bytes[164], 0xc);
    assert_eq!(bytes[165], 0x6a);
    assert_eq!(bytes[166], 0xc1);
    assert_eq!(bytes[167], 0x53);
    assert_eq!(bytes[168], 0x5e);
    assert_eq!(bytes[169], 0xd7);
    assert_eq!(bytes[170], 0xe2);
    assert_eq!(bytes[171], 0xf7);
    assert_eq!(bytes[172], 0xed);
    assert_eq!(bytes[173], 0xc);
    assert_eq!(bytes[174], 0xdd);
    assert_eq!(bytes[175], 0x53);
    assert_eq!(bytes[176], 0x74);
    assert_eq!(bytes[177], 0xb5);
    assert_eq!(bytes[178], 0xa);
    assert_eq!(bytes[179], 0x8c);
    assert_eq!(bytes[180], 0x95);
    assert_eq!(bytes[181], 0x80);
    assert_eq!(bytes[182], 0x69);
    assert_eq!(bytes[183], 0x3f);
    assert_eq!(bytes[184], 0x7d);
    assert_eq!(bytes[185], 0x5a);
    assert_eq!(bytes[186], 0xb1);
    assert_eq!(bytes[187], 0xdc);
    assert_eq!(bytes[188], 0xd7);
    assert_eq!(bytes[189], 0xfa);
    assert_eq!(bytes[190], 0x79);
    assert_eq!(bytes[191], 0x43);
    assert_eq!(bytes[192], 0xdc);
    assert_eq!(bytes[193], 0x0);
    assert_eq!(bytes[194], 0xb5);
    assert_eq!(bytes[195], 0xb);
    assert_eq!(bytes[196], 0xf0);
    assert_eq!(bytes[197], 0xdb);
    assert_eq!(bytes[198], 0x2b);
    assert_eq!(bytes[199], 0x45);
    assert_eq!(bytes[200], 0x13);
    assert_eq!(bytes[201], 0xc3);
    assert_eq!(bytes[202], 0xfd);
    assert_eq!(bytes[203], 0x8d);
    assert_eq!(bytes[204], 0xe3);
    assert_eq!(bytes[205], 0xce);
    assert_eq!(bytes[206], 0x4d);
    assert_eq!(bytes[207], 0xd1);
    assert_eq!(bytes[208], 0xe3);
    assert_eq!(bytes[209], 0xb2);
    assert_eq!(bytes[210], 0x67);
    assert_eq!(bytes[211], 0x6e);
    assert_eq!(bytes[212], 0x6);
    assert_eq!(bytes[213], 0x5a);
    assert_eq!(bytes[214], 0xe7);
    assert_eq!(bytes[215], 0x3b);
    assert_eq!(bytes[216], 0xa7);
    assert_eq!(bytes[217], 0x45);
    assert_eq!(bytes[218], 0xc7);
    assert_eq!(bytes[219], 0x80);
    assert_eq!(bytes[220], 0xea);
    assert_eq!(bytes[221], 0xf2);
    assert_eq!(bytes[222], 0x3a);
    assert_eq!(bytes[223], 0x86);
    assert_eq!(bytes[224], 0x4);
    assert_eq!(bytes[225], 0x9d);
    assert_eq!(bytes[226], 0x92);
    assert_eq!(bytes[227], 0x54);
    assert_eq!(bytes[228], 0x9a);
    assert_eq!(bytes[229], 0x85);
    assert_eq!(bytes[230], 0x24);
    assert_eq!(bytes[231], 0xdc);
    assert_eq!(bytes[232], 0x88);
    assert_eq!(bytes[233], 0x3d);
    assert_eq!(bytes[234], 0x7);
    assert_eq!(bytes[235], 0x94);
    assert_eq!(bytes[236], 0x69);
    assert_eq!(bytes[237], 0x8c);
    assert_eq!(bytes[238], 0xff);
    assert_eq!(bytes[239], 0x94);
    assert_eq!(bytes[240], 0x34);
    assert_eq!(bytes[241], 0xef);
    assert_eq!(bytes[242], 0x85);
    assert_eq!(bytes[243], 0xf2);
    assert_eq!(bytes[244], 0x97);
    assert_eq!(bytes[245], 0x35);
    assert_eq!(bytes[246], 0x7f);
    assert_eq!(bytes[247], 0x4b);
    assert_eq!(bytes[248], 0x9c);
    assert_eq!(bytes[249], 0xa5);
    assert_eq!(bytes[250], 0x7f);
    assert_eq!(bytes[251], 0x56);
    assert_eq!(bytes[252], 0x19);
    assert_eq!(bytes[253], 0xaa);
    assert_eq!(bytes[254], 0x25);
    assert_eq!(bytes[255], 0xe7);
}

#[test]
fn test_exec_iadd_rs() {
    let instr = Instr {
        op: Opcode::IADD_RS,
        dst: r_reg(0),
        src: r_reg(1),
        imm: None,
        unsigned_imm: false,
        mode: Mode::Shft(3),
        target: None,
        effect: Vm::exec_iadd_rs,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0x8000000000000000;
    vm.reg.r[1] = 0x1000000000000000;
    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], 0x0);
}

#[test]
fn test_exec_iadd_rs_with_immediate() {
    let instr = Instr {
        op: Opcode::IADD_RS,
        dst: REG_NEEDS_DISPLACEMENT,
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::Shft(2),
        target: None,
        effect: Vm::exec_iadd_rs,
    };
    let mut vm = new_test_vm();
    vm.reg.r[REG_NEEDS_DISPLACEMENT_IX] = 0x8000000000000000;
    vm.reg.r[1] = 0x2000000000000000;
    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[REG_NEEDS_DISPLACEMENT_IX], IMM64);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_isub_r() {
    let instr = Instr {
        op: Opcode::ISUB_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: None,
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_isub_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 1;
    vm.reg.r[1] = 0xFFFFFFFF;
    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], 0xFFFFFFFF00000002);
}

#[test]
fn test_exec_isub_r_with_immediate() {
    let instr = Instr {
        op: Opcode::ISUB_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_isub_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0;
    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], (!IMM64 + 1));
}

#[test]
fn test_exec_imul_r() {
    let instr = Instr {
        op: Opcode::IMUL_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], 0x28723424A9108E51);
}

#[test]
fn test_exec_imul_r_with_immediate() {
    let instr = Instr {
        op: Opcode::IMUL_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 1;

    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], IMM64);
}

#[test]
fn test_exec_imulh_r() {
    let instr = Instr {
        op: Opcode::IMULH_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imulh_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], 0xB4676D31D2B34883);
}

#[test]
fn test_exec_ismulh_r() {
    let instr = Instr {
        op: Opcode::ISMULH_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ismulh_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], 0x02D93EF1269D3EE5);
}

#[test]
fn test_exec_ineg_r() {
    let instr = Instr {
        op: Opcode::INEG_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ineg_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xFFFFFFFFFFFFFFFF;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 1);
}

#[test]
fn test_exec_ineg_r_overflow() {
    let instr = Instr {
        op: Opcode::INEG_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ineg_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0x0;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0);
}

#[test]
fn test_exec_ixor_r() {
    let instr = Instr {
        op: Opcode::IXOR_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ixor_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0x8888888888888888;
    vm.reg.r[1] = 0xAAAAAAAAAAAAAAAA;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x2222222222222222);
}

#[test]
fn test_exec_ixor_r_with_immediate() {
    let instr = Instr {
        op: Opcode::IXOR_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ixor_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xFFFFFFFFFFFFFFFF;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], !IMM64);
}

#[test]
fn test_exec_iror_r() {
    let instr = Instr {
        op: Opcode::IROR_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_iror_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 953360005391419562;
    vm.reg.r[1] = 4569451684712230561;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0xD835C455069D81EF);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_iror_r_with_immediate() {
    let instr = Instr {
        op: Opcode::IROR_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(4569451684712230561 as i32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_iror_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 953360005391419562;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0xD835C455069D81EF);
}

#[test]
fn test_exec_irol_r() {
    let instr = Instr {
        op: Opcode::IROL_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_irol_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 953360005391419562;
    vm.reg.r[1] = 4569451684712230561;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 6978065200552740799);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_irol_r_with_immediate() {
    let instr = Instr {
        op: Opcode::IROL_R,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(4569451684712230561 as i32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_irol_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 953360005391419562;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 6978065200552740799);
}

#[test]
fn test_exec_iswap_r() {
    let instr = Instr {
        op: Opcode::ISWAP_R,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_iswap_r,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 953360005391419562;
    vm.reg.r[1] = 4569451684712230561;

    instr.execute(&mut vm);
    assert_eq!(vm.reg.r[0], 4569451684712230561);
    assert_eq!(vm.reg.r[1], 953360005391419562);
}

#[test]
fn test_exec_fswap_r_from_f_reg() {
    let instr = Instr {
        op: Opcode::FSWAP_R,
        dst: f_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fswap_r,
    };
    let mut vm = new_test_vm();
    vm.reg.f[0] = m128d::from_u64(953360005391419562, 4569451684712230561);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(4569451684712230561, 953360005391419562)
    );
}

#[test]
fn test_exec_fswap_r_from_e_reg() {
    let instr = Instr {
        op: Opcode::FSWAP_R,
        dst: e_reg(3),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fswap_r,
    };
    let mut vm = new_test_vm();
    vm.reg.e[3] = m128d::from_u64(953360005391419562, 4569451684712230561);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[3],
        m128d::from_u64(4569451684712230561, 953360005391419562)
    );
}

#[test]
fn test_exec_fadd_r_round_to_nearest() {
    let instr = Instr {
        op: Opcode::FADD_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fadd_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0x402dcc3b63eaa6fe, 0xc1ce30a748e032b9)
    )
}

#[test]
fn test_exec_fadd_r_round_down() {
    let instr = Instr {
        op: Opcode::FADD_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fadd_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0x402dcc3b63eaa6fd, 0xc1ce30a748e032b9)
    )
}

#[test]
fn test_exec_fadd_r_round_up() {
    let instr = Instr {
        op: Opcode::FADD_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fadd_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0x402dcc3b63eaa6fe, 0xc1ce30a748e032b8)
    );
}

#[test]
fn test_exec_fadd_r_round_to_zero() {
    let instr = Instr {
        op: Opcode::FADD_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fadd_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0x402dcc3b63eaa6fd, 0xc1ce30a748e032b8)
    )
}

#[test]
fn test_exec_fsub_r_round_to_nearest() {
    let instr = Instr {
        op: Opcode::FSUB_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsub_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0xc026811570d6eaf8, 0xc1ce30c03f643833)
    )
}

#[test]
fn test_exec_fsub_r_round_down() {
    let instr = Instr {
        op: Opcode::FSUB_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsub_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0xc026811570d6eaf9, 0xc1ce30c03f643834)
    )
}

#[test]
fn test_exec_fsub_r_round_up() {
    let instr = Instr {
        op: Opcode::FSUB_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsub_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0xc026811570d6eaf8, 0xc1ce30c03f643833)
    )
}

#[test]
fn test_exec_fsub_r_round_to_zero() {
    let instr = Instr {
        op: Opcode::FSUB_R,
        dst: f_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsub_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0xc026811570d6eaf8, 0xc1ce30c03f643833)
    )
}

#[test]
fn test_exec_fscal_r() {
    let instr = Instr {
        op: Opcode::FSCAL_R,
        dst: f_reg(0),
        src: Store::L1(Box::new(Store::R(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fscal_r,
    };
    let mut vm = new_test_vm();
    vm.reg.f[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0xc12bc35cef248783, 0xc00dfdabb6173d07)
    );
}

#[test]
fn test_exec_fmul_r_round_to_nearest() {
    let instr = Instr {
        op: Opcode::FMUL_R,
        dst: e_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fmul_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x42d7feeccd89152f, 0x42d30f35ff7a6969)
    )
}

#[test]
fn test_exec_fmul_r_round_round_down() {
    let instr = Instr {
        op: Opcode::FMUL_R,
        dst: e_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fmul_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x42d7feeccd89152e, 0x42d30f35ff7a6969)
    )
}

#[test]
fn test_exec_fmul_r_round_up() {
    let instr = Instr {
        op: Opcode::FMUL_R,
        dst: e_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fmul_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x42d7feeccd89152f, 0x42d30f35ff7a696a)
    )
}

#[test]
fn test_exec_fmul_r_round_to_zero() {
    let instr = Instr {
        op: Opcode::FMUL_R,
        dst: e_reg(0),
        src: a_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fmul_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x42d7feeccd89152e, 0x42d30f35ff7a6969)
    )
}

#[test]
fn test_exec_fsqrt_r_round_to_nearest() {
    let instr = Instr {
        op: Opcode::FSQRT_R,
        dst: e_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsqrt_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x40d30e573fa3ba8d, 0x40212a610b301fe8)
    );
}

#[test]
fn test_exec_fsqrt_r_round_up() {
    let instr = Instr {
        op: Opcode::FSQRT_R,
        dst: e_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsqrt_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x40d30e573fa3ba8d, 0x40212a610b301fe9)
    );
}

#[test]
fn test_exec_fsqrt_r_round_down() {
    let instr = Instr {
        op: Opcode::FSQRT_R,
        dst: e_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsqrt_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x40d30e573fa3ba8c, 0x40212a610b301fe8)
    );
}

#[test]
fn test_exec_fsqrt_r_round_to_zero() {
    let instr = Instr {
        op: Opcode::FSQRT_R,
        dst: e_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsqrt_r,
    };
    let mut vm = new_test_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x40d30e573fa3ba8c, 0x40212a610b301fe8)
    );
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_fadd_m() {
    let instr = Instr {
        op: Opcode::FADD_M,
        dst: f_reg(0),
        src: Store::L1(Box::new(Store::R(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fadd_m,
    };
    let mut vm = new_test_vm();
    vm.scratchpad[0] = 0x1234567890abcdef;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.f[0] = m128d::zero();
    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0x41b2345678000000, 0xc1dbd50c84400000)
    );
}

#[test]
fn test_exec_fsub_m() {
    let instr = Instr {
        op: Opcode::FSUB_M,
        dst: f_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fsub_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.scratchpad[0] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.f[0],
        m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b4c5a23576)
    );
}

#[test]
fn test_exec_cfround() {
    let instr = Instr {
        op: Opcode::CFROUND,
        dst: Store::NONE,
        src: r_reg(0),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_cfround,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xFFFFFFFFFFFC6800;

    assert_eq!(vm.get_rounding_mode(), ROUND_TO_NEAREST); //new vm starts with default rounding mode

    instr.execute(&mut vm);

    assert_eq!(vm.get_rounding_mode(), ROUND_TO_ZERO);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_cbranch_taken() {
    let instr = Instr {
        op: Opcode::CBRANCH,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(0xFFFFFFFFC0CB9AD2),
        unsigned_imm: false,
        mode: Mode::Cond(3),
        target: Some(100),
        effect: Vm::exec_cbranch,
    };
    let mut vm = new_test_vm();
    vm.pc = 200;
    vm.reg.r[0] = 0xFFFFFFFFFFFC6800;

    instr.execute(&mut vm);

    assert_eq!(vm.pc, 100)
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_cbranch_not_taken() {
    let instr = Instr {
        op: Opcode::CBRANCH,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(0xFFFFFFFFC0CB9AD2),
        unsigned_imm: false,
        mode: Mode::Cond(3),
        target: None,
        effect: Vm::exec_cbranch,
    };
    let mut vm = new_test_vm();
    vm.pc = 200;
    vm.reg.r[0] = 0;

    instr.execute(&mut vm);

    assert_eq!(vm.pc, 200)
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_istore_l1() {
    let instr = Instr {
        op: Opcode::ISTORE,
        dst: Store::L1(Box::new(r_reg(0))),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_istore,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFC6800;
    vm.reg.r[0] = 0xFFFFFFFFC0C802D2;

    instr.execute(&mut vm);

    assert_eq!(vm.scratchpad[0x19A0 / 8], 0xFFFFFFFFFFFC6800);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_istore_l2() {
    let instr = Instr {
        op: Opcode::ISTORE,
        dst: Store::L2(Box::new(r_reg(0))),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_istore,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFC6800;
    vm.reg.r[0] = 0xFFFFFFFFC0C802D2;

    instr.execute(&mut vm);

    assert_eq!(vm.scratchpad[0x399A0 / 8], 0xFFFFFFFFFFFC6800);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_istore_l3() {
    let instr = Instr {
        op: Opcode::ISTORE,
        dst: Store::L3(Box::new(r_reg(0))),
        src: r_reg(1),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_istore,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFC6800;
    vm.reg.r[0] = 0xFFFFFFFFC0C802D2;

    instr.execute(&mut vm);

    assert_eq!(vm.scratchpad[0x1399A0 / 8], 0xFFFFFFFFFFFC6800);
}

#[test]
fn test_exec_iadd_m_l1() {
    let instr = Instr {
        op: Opcode::IADD_M,
        dst: r_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_iadd_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x869);
}

#[test]
fn test_exec_iadd_m_l2() {
    let instr = Instr {
        op: Opcode::IADD_M,
        dst: r_reg(0),
        src: Store::L2(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_iadd_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0x38000 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x869);
}

#[test]
fn test_exec_iadd_m_l3() {
    let instr = Instr {
        op: Opcode::IADD_M,
        dst: r_reg(0),
        src: Store::L3(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_iadd_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0xb96d0 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x869);
}

#[test]
fn test_exec_isub_m_l1() {
    let instr = Instr {
        op: Opcode::ISUB_M,
        dst: r_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_isub_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 - 0x203);
}

#[test]
fn test_exec_isub_m_l2() {
    let instr = Instr {
        op: Opcode::ISUB_M,
        dst: r_reg(0),
        src: Store::L2(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_isub_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0x38000 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 - 0x0203);
}

#[test]
fn test_exec_isub_m_l3() {
    let instr = Instr {
        op: Opcode::ISUB_M,
        dst: r_reg(0),
        src: Store::L3(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_isub_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0xb96d0 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 - 0x0203);
}

#[test]
fn test_exec_imul_m_l1() {
    let instr = Instr {
        op: Opcode::IMUL_M,
        dst: r_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 * 0x203);
}

#[test]
fn test_exec_imul_m_l2() {
    let instr = Instr {
        op: Opcode::IMUL_M,
        dst: r_reg(0),
        src: Store::L2(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0x38000 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 * 0x0203);
}

#[test]
fn test_exec_imul_m_l3() {
    let instr = Instr {
        op: Opcode::IMUL_M,
        dst: r_reg(0),
        src: Store::L3(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0xb96d0 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 * 0x0203);
}

#[test]
fn test_exec_imulh_m_l1() {
    let instr = Instr {
        op: Opcode::IMULH_M,
        dst: r_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imulh_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.scratchpad[0] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0xB4676D31D2B34883);
}

#[test]
fn test_exec_imulh_m_l2() {
    let instr = Instr {
        op: Opcode::IMULH_M,
        dst: r_reg(0),
        src: Store::L2(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imulh_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.scratchpad[0x38000 / 8] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0xB4676D31D2B34883);
}

#[test]
fn test_exec_imulh_m_l3() {
    let instr = Instr {
        op: Opcode::IMULH_M,
        dst: r_reg(0),
        src: Store::L3(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imulh_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.scratchpad[0xb96d0 / 8] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0xB4676D31D2B34883);
}

#[test]
fn test_exec_ismulh_m_l1() {
    let instr = Instr {
        op: Opcode::ISMULH_M,
        dst: r_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ismulh_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.scratchpad[0] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x02D93EF1269D3EE5);
}

#[test]
fn test_exec_ismulh_m_l2() {
    let instr = Instr {
        op: Opcode::ISMULH_M,
        dst: r_reg(0),
        src: Store::L2(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ismulh_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.scratchpad[0x38000 / 8] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x02D93EF1269D3EE5);
}

#[test]
fn test_exec_ismulh_m_l3() {
    let instr = Instr {
        op: Opcode::ISMULH_M,
        dst: r_reg(0),
        src: Store::L3(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ismulh_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.scratchpad[0xb96d0 / 8] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x02D93EF1269D3EE5);
}

#[test]
fn test_exec_imul_rcp_non_zero_imm_from_reg() {
    let instr = Instr {
        op: Opcode::IMUL_RCP,
        dst: r_reg(0),
        src: Store::NONE,
        imm: Some(IMM32),
        unsigned_imm: true,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_rcp,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 666;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x2B2462DE8506B218);
}

#[test]
fn test_exec_imul_rcp_zero_imm() {
    let instr = Instr {
        op: Opcode::IMUL_RCP,
        dst: r_reg(0),
        src: r_reg(1),
        imm: Some(0),
        unsigned_imm: true,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_imul_rcp,
    };
    let mut vm = new_test_vm();
    vm.reg.r[0] = 0x666;

    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666);
}

#[test]
fn test_exec_ixor_m_l1() {
    let instr = Instr {
        op: Opcode::IXOR_M,
        dst: r_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ixor_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 ^ 0x203);
}

#[test]
fn test_exec_ixor_m_l2() {
    let instr = Instr {
        op: Opcode::IXOR_M,
        dst: r_reg(0),
        src: Store::L2(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ixor_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0x38000 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 ^ 0x203);
}

#[test]
fn test_exec_ixor_m_l3() {
    let instr = Instr {
        op: Opcode::IXOR_M,
        dst: r_reg(0),
        src: Store::L3(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_ixor_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0xb96d0 / 8] = 0x0203;
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x666 ^ 0x203);
}

#[test]
fn test_exec_fdiv_m_round_to_nearest() {
    let instr = Instr {
        op: Opcode::FDIV_M,
        dst: e_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fdiv_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.e[0] = m128d::from_u64(0x41937f76fede16ee, 0x411b414296ce93b6);
    vm.set_rounding_mode(ROUND_TO_NEAREST);
    vm.config.e_mask[0] = 0x3a0000000005d11a;
    vm.config.e_mask[1] = 0x39000000001ba31e;
    vm.scratchpad[0] = 0x8b2460d9d350a1b6;

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x47a55b63664a4732, 0x464384946369b2e7)
    );
}

#[test]
fn test_exec_fdiv_m_round_down_and_to_zero() {
    let instr = Instr {
        op: Opcode::FDIV_M,
        dst: e_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fdiv_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.e[0] = m128d::from_u64(0x41937f76fede16ee, 0x411b414296ce93b6);
    vm.set_rounding_mode(ROUND_TO_ZERO);
    vm.config.e_mask[0] = 0x3a0000000005d11a;
    vm.config.e_mask[1] = 0x39000000001ba31e;
    vm.scratchpad[0] = 0x8b2460d9d350a1b6;

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x47a55b63664a4732, 0x464384946369b2e6)
    );
}

#[test]
fn test_exec_fdiv_m_round_to_zero() {
    let instr = Instr {
        op: Opcode::FDIV_M,
        dst: e_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fdiv_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.e[0] = m128d::from_u64(0x41937f76fede16ee, 0x411b414296ce93b6);
    vm.set_rounding_mode(ROUND_DOWN);
    vm.config.e_mask[0] = 0x3a0000000005d11a;
    vm.config.e_mask[1] = 0x39000000001ba31e;
    vm.scratchpad[0] = 0x8b2460d9d350a1b6;

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x47a55b63664a4732, 0x464384946369b2e6)
    );
}

#[test]
fn test_exec_fdiv_m_round_up() {
    let instr = Instr {
        op: Opcode::FDIV_M,
        dst: e_reg(0),
        src: Store::L1(Box::new(r_reg(1))),
        imm: Some(IMM32),
        unsigned_imm: false,
        mode: Mode::None,
        target: None,
        effect: Vm::exec_fdiv_m,
    };
    let mut vm = new_test_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.e[0] = m128d::from_u64(0x41937f76fede16ee, 0x411b414296ce93b6);
    vm.set_rounding_mode(ROUND_UP);
    vm.config.e_mask[0] = 0x3a0000000005d11a;
    vm.config.e_mask[1] = 0x39000000001ba31e;
    vm.scratchpad[0] = 0x8b2460d9d350a1b6;

    instr.execute(&mut vm);

    assert_eq!(
        vm.reg.e[0],
        m128d::from_u64(0x47a55b63664a4733, 0x464384946369b2e7)
    );
}

#[test]
fn test_randomx_reciprocal() {
    let result = randomx_reciprocal(0xc0cb96d2);
    assert_eq!(result, 0xa9f671ed1d69b73c);
}

//helper

fn new_test_vm() -> Vm {
    new_vm(Arc::new(VmMemory::no_memory()))
}
