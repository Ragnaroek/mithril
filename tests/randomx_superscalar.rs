extern crate mithril;
#[macro_use(assert_diff)]
extern crate difference;

use mithril::randomx::superscalar::{ScProgram, Blake2Generator};

#[test]
fn test_generate() {
	let key_str = b"test key 000";
	let mut key : [u8; 60] = [0; 60];
	key[..key_str.len()].copy_from_slice(key_str);
	
	let mut gen = Blake2Generator::new(key, 0);
	let prog = ScProgram::generate(&mut gen);
	
	//assert_diff!(EXPECTED_SUPERSCALAR_PROG_1, &prog.to_string(), "\n", 0);
	//assert_eq!(true, false);
}

//helper

/*
	runTest("SuperscalarHash generator", RANDOMX_SUPERSCALAR_LATENCY == 170, []() {
		char sprogHash[32];
		randomx::SuperscalarProgram sprog;
		const char key[] = "test key 000";
		constexpr size_t keySize = sizeof(key) - 1;
		randomx::Blake2Generator gen(key, keySize);

		const char superscalarReferences[10][65] = {
			"d3a4a6623738756f77e6104469102f082eff2a3e60be7ad696285ef7dfc72a61",
			"f5e7e0bbc7e93c609003d6359208688070afb4a77165a552ff7be63b38dfbc86",
			"85ed8b11734de5b3e9836641413a8f36e99e89694f419c8cd25c3f3f16c40c5a",
			"5dd956292cf5d5704ad99e362d70098b2777b2a1730520be52f772ca48cd3bc0",
			"6f14018ca7d519e9b48d91af094c0f2d7e12e93af0228782671a8640092af9e5",
			"134be097c92e2c45a92f23208cacd89e4ce51f1009a0b900dbe83b38de11d791",
			"268f9392c20c6e31371a5131f82bd7713d3910075f2f0468baafaa1abd2f3187",
			"c668a05fd909714ed4a91e8d96d67b17e44329e88bc71e0672b529a3fc16be47",
			"99739351315840963011e4c5d8e90ad0bfed3facdcb713fe8f7138fbf01c4c94",
			"14ab53d61880471f66e80183968d97effd5492b406876060e595fcf9682f9295",
		};

		for (int i = 0; i < 10; ++i) {
			randomx::generateSuperscalar(sprog, gen);
			blake2b(sprogHash, sizeof(sprogHash), &sprog.programBuffer, sizeof(randomx::Instruction) * sprog.getSize(), nullptr, 0);
			assert(equalsHex(sprogHash, superscalarReferences[i]));
		}
	});
*/



const EXPECTED_SUPERSCALAR_PROG_1 : &str = r#"op: IMUL_R, src: 0, dst: 3
op: IMUL_R, src: 1, dst: 4
op: IMUL_R, src: 7, dst: 6
op: IROR_C, src: -1, dst: 7
op: IADD_RS, src: 1, dst: 2
op: IXOR_C9, src: -1, dst: 0
op: ISMULH_R, src: 5, dst: 1
op: IMUL_RCP, src: -1, dst: 0
op: IADD_C8, src: -1, dst: 5
op: IROR_C, src: -1, dst: 4
op: IROR_C, src: -1, dst: 3
op: IADD_C9, src: -1, dst: 3
op: ISMULH_R, src: 3, dst: 7
op: IMUL_RCP, src: -1, dst: 1
op: IMUL_R, src: 2, dst: 3
op: IMUL_R, src: 4, dst: 0
op: IROR_C, src: -1, dst: 2
op: IADD_C7, src: -1, dst: 6
op: IXOR_R, src: 2, dst: 4
op: ISUB_R, src: 4, dst: 5
op: IXOR_R, src: 2, dst: 6
op: IADD_C7, src: -1, dst: 5
op: IXOR_R, src: 2, dst: 7
op: IXOR_R, src: 6, dst: 2
op: ISMULH_R, src: 4, dst: 4
op: IMUL_RCP, src: -1, dst: 6
op: IMUL_R, src: 5, dst: 2
op: IMUL_R, src: 0, dst: 5
op: IADD_RS, src: 7, dst: 1
op: IXOR_C7, src: -1, dst: 0
op: ISUB_R, src: 3, dst: 1
op: ISUB_R, src: 3, dst: 2
op: IXOR_R, src: 3, dst: 7
op: ISUB_R, src: 3, dst: 0
op: IXOR_C7, src: -1, dst: 2
op: ISUB_R, src: 1, dst: 3
op: IXOR_R, src: 2, dst: 4
op: IMUL_R, src: 2, dst: 1
op: IMUL_R, src: 0, dst: 3
op: IMUL_R, src: 6, dst: 4
op: IROR_C, src: -1, dst: 0
op: IXOR_R, src: 7, dst: 0
op: IXOR_C7, src: -1, dst: 5
op: IXOR_R, src: 6, dst: 2
op: IXOR_R, src: 7, dst: 6
op: IADD_RS, src: 2, dst: 6
op: IXOR_C9, src: -1, dst: 0
op: ISUB_R, src: 5, dst: 2
op: IMUL_R, src: 2, dst: 0
op: IMUL_R, src: 2, dst: 6
op: IMUL_R, src: 4, dst: 2
op: IROR_C, src: -1, dst: 3
op: IROR_C, src: -1, dst: 1
op: IADD_C9, src: -1, dst: 5
op: IMULH_R, src: 3, dst: 7
op: IMUL_RCP, src: -1, dst: 1
op: IXOR_C9, src: -1, dst: 3
op: IMULH_R, src: 4, dst: 4
op: IMUL_RCP, src: -1, dst: 2
op: IXOR_C8, src: -1, dst: 5
op: IADD_RS, src: 3, dst: 0
op: IROR_C, src: -1, dst: 5
op: IXOR_C9, src: -1, dst: 5
op: ISMULH_R, src: 0, dst: 3
op: IMUL_RCP, src: -1, dst: 0
op: IMUL_R, src: 5, dst: 4
op: IMUL_R, src: 7, dst: 1
op: IADD_RS, src: 5, dst: 6
op: IROR_C, src: -1, dst: 5
op: IXOR_C8, src: -1, dst: 6
op: IROR_C, src: -1, dst: 7
op: IROR_C, src: -1, dst: 6
op: IADD_C9, src: -1, dst: 5
op: ISUB_R, src: 7, dst: 2
op: IMUL_R, src: 7, dst: 0
op: IMUL_R, src: 2, dst: 5
op: IMUL_R, src: 1, dst: 2
op: IROR_C, src: -1, dst: 3
op: IADD_RS, src: 7, dst: 6
op: IADD_C8, src: -1, dst: 6
op: IROR_C, src: -1, dst: 4
op: IADD_RS, src: 7, dst: 6
op: IXOR_C8, src: -1, dst: 4
op: IROR_C, src: -1, dst: 6
op: IMUL_R, src: 7, dst: 4
op: IMUL_R, src: 0, dst: 3
op: IMUL_R, src: 1, dst: 7
op: IADD_RS, src: 0, dst: 6
op: IADD_RS, src: 6, dst: 0
op: IADD_C9, src: -1, dst: 1
op: IMULH_R, src: 1, dst: 1
op: IMUL_RCP, src: -1, dst: 6
op: IADD_C8, src: -1, dst: 0
op: IADD_RS, src: 5, dst: 2
op: IROR_C, src: -1, dst: 4
op: IXOR_C9, src: -1, dst: 0
op: ISUB_R, src: 4, dst: 2
op: IMUL_R, src: 4, dst: 0
op: IMUL_R, src: 5, dst: 4
op: IMUL_R, src: 7, dst: 2
op: IROR_C, src: -1, dst: 5
op: IXOR_C7, src: -1, dst: 7
op: IXOR_R, src: 5, dst: 7
op: ISUB_R, src: 5, dst: 3
op: IMULH_R, src: 3, dst: 5
op: IMUL_RCP, src: -1, dst: 4
op: IADD_C9, src: -1, dst: 6
op: ISMULH_R, src: 6, dst: 7
op: IMUL_RCP, src: -1, dst: 0
op: IXOR_C9, src: -1, dst: 1
op: ISMULH_R, src: 2, dst: 3
op: IMUL_RCP, src: -1, dst: 5
op: IADD_C9, src: -1, dst: 2
op: ISUB_R, src: 1, dst: 2
op: IXOR_R, src: 1, dst: 6
op: IXOR_C7, src: -1, dst: 6
op: ISUB_R, src: 1, dst: 6
op: IMULH_R, src: 2, dst: 1
op: IMUL_RCP, src: -1, dst: 2
op: IMUL_R, src: 7, dst: 6
op: IMUL_R, src: 5, dst: 4
op: IROR_C, src: -1, dst: 7
op: IROR_C, src: -1, dst: 0
op: IADD_C9, src: -1, dst: 7
op: ISMULH_R, src: 0, dst: 7
op: IMUL_RCP, src: -1, dst: 0
op: IXOR_C8, src: -1, dst: 3
op: IADD_RS, src: 5, dst: 3
op: IROR_C, src: -1, dst: 5
op: IXOR_C9, src: -1, dst: 5
op: IMULH_R, src: 6, dst: 1
op: IMUL_RCP, src: -1, dst: 5
op: IMUL_R, src: 0, dst: 7
op: IMUL_R, src: 6, dst: 0
op: IADD_RS, src: 6, dst: 3
op: IADD_C7, src: -1, dst: 6
op: IXOR_R, src: 6, dst: 3
op: IXOR_R, src: 6, dst: 4
op: ISUB_R, src: 4, dst: 3
op: IXOR_C7, src: -1, dst: 4
op: IXOR_R, src: 4, dst: 2
op: IXOR_R, src: 3, dst: 6
op: ISMULH_R, src: 4, dst: 4
op: IMUL_RCP, src: -1, dst: 6
op: IMUL_R, src: 3, dst: 5
op: IMUL_R, src: 0, dst: 1
op: IADD_RS, src: 7, dst: 2
op: IROR_C, src: -1, dst: 7
op: IADD_C9, src: -1, dst: 2
op: IMULH_R, src: 3, dst: 3
op: IMUL_RCP, src: -1, dst: 2
op: IXOR_C9, src: -1, dst: 7
op: IMULH_R, src: 4, dst: 0
op: IMUL_RCP, src: -1, dst: 7
op: IXOR_C9, src: -1, dst: 4
op: IMULH_R, src: 3, dst: 6
op: IMUL_RCP, src: -1, dst: 3
op: IADD_C8, src: -1, dst: 5
op: IROR_C, src: -1, dst: 4
op: IADD_RS, src: 4, dst: 1
op: IXOR_C8, src: -1, dst: 5
op: IADD_RS, src: 5, dst: 2
op: IMUL_R, src: 4, dst: 2
op: IMUL_R, src: 5, dst: 1
op: IMUL_R, src: 5, dst: 6
op: IADD_RS, src: 5, dst: 4
op: IADD_RS, src: 7, dst: 4
op: IADD_C9, src: -1, dst: 5
op: IMULH_R, src: 0, dst: 5
op: IMUL_RCP, src: -1, dst: 2
op: IXOR_C8, src: -1, dst: 4
op: IADD_RS, src: 0, dst: 7
op: IADD_RS, src: 3, dst: 7
op: IADD_C8, src: -1, dst: 4
op: IADD_RS, src: 0, dst: 3
op: IMUL_R, src: 4, dst: 7
op: IMUL_R, src: 1, dst: 0
op: IMUL_R, src: 1, dst: 3
op: IROR_C, src: -1, dst: 4
op: IXOR_R, src: 1, dst: 6
op: IXOR_C7, src: -1, dst: 1
op: ISUB_R, src: 1, dst: 4
op: IMULH_R, src: 2, dst: 1
op: IMUL_RCP, src: -1, dst: 6
op: IXOR_C8, src: -1, dst: 4
op: IADD_RS, src: 2, dst: 4
op: IROR_C, src: -1, dst: 2
op: IADD_C8, src: -1, dst: 2
op: IROR_C, src: -1, dst: 7
op: IMUL_R, src: 4, dst: 2
op: IMUL_R, src: 0, dst: 7
op: IMUL_R, src: 0, dst: 5
op: IROR_C, src: -1, dst: 4
op: IADD_RS, src: 4, dst: 0
op: IADD_C9, src: -1, dst: 4
op: IXOR_R, src: 0, dst: 4
op: IADD_RS, src: 0, dst: 3
op: IADD_C9, src: -1, dst: 0
op: ISUB_R, src: 6, dst: 4
op: IMUL_R, src: 2, dst: 3
op: IMUL_R, src: 4, dst: 1
op: IMUL_R, src: 4, dst: 6
op: IADD_RS, src: 2, dst: 0
op: ISUB_R, src: 4, dst: 2
op: IADD_C7, src: -1, dst: 4
op: IXOR_R, src: 2, dst: 7
op: ISUB_R, src: 0, dst: 7
op: ISUB_R, src: 0, dst: 5
op: IXOR_C7, src: -1, dst: 2
op: IXOR_R, src: 3, dst: 5
op: IXOR_R, src: 3, dst: 7
op: IMUL_R, src: 2, dst: 7
op: IMUL_R, src: 3, dst: 2
op: IMUL_R, src: 0, dst: 4
op: IROR_C, src: -1, dst: 1
op: IADD_C7, src: -1, dst: 3
op: IXOR_R, src: 1, dst: 6
op: IXOR_R, src: 3, dst: 1
op: IMULH_R, src: 5, dst: 0
op: IMUL_RCP, src: -1, dst: 3
op: IXOR_C9, src: -1, dst: 1
op: ISUB_R, src: 7, dst: 5
op: IADD_RS, src: 5, dst: 1
op: IADD_C9, src: -1, dst: 5
op: ISUB_R, src: 5, dst: 4
op: IMUL_R, src: 1, dst: 5
op: IMUL_R, src: 4, dst: 6
op: IMUL_R, src: 2, dst: 1
op: IROR_C, src: -1, dst: 2
op: IXOR_R, src: 2, dst: 4
op: IADD_C7, src: -1, dst: 2
op: ISUB_R, src: 0, dst: 3
op: IXOR_R, src: 4, dst: 7
op: ISUB_R, src: 0, dst: 2
op: IXOR_C7, src: -1, dst: 5
op: ISUB_R, src: 4, dst: 7
op: ISUB_R, src: 0, dst: 7
op: IMUL_R, src: 3, dst: 2
op: IMUL_R, src: 1, dst: 3
op: IMUL_R, src: 0, dst: 7
op: IADD_RS, src: 6, dst: 0
op: IXOR_R, src: 1, dst: 4
op: IXOR_C7, src: -1, dst: 0
op: ISUB_R, src: 5, dst: 6
op: ISUB_R, src: 4, dst: 0
op: IXOR_R, src: 1, dst: 2
op: IXOR_C7, src: -1, dst: 6
op: ISUB_R, src: 5, dst: 1
op: ISMULH_R, src: 1, dst: 5
op: IMUL_RCP, src: -1, dst: 4
op: IMUL_R, src: 3, dst: 2
op: IMUL_R, src: 1, dst: 6
op: IROR_C, src: -1, dst: 7
op: IADD_RS, src: 0, dst: 3
op: IADD_C9, src: -1, dst: 7
op: IMULH_R, src: 5, dst: 1
op: IMUL_RCP, src: -1, dst: 0
op: IADD_C9, src: -1, dst: 3
op: IXOR_R, src: 7, dst: 3
op: IXOR_C7, src: -1, dst: 7
op: ISUB_R, src: 4, dst: 2
op: IXOR_R, src: 4, dst: 5
op: ISUB_R, src: 3, dst: 2
op: IMUL_R, src: 6, dst: 3
op: IMUL_R, src: 2, dst: 4
op: IMUL_R, src: 1, dst: 5
op: IADD_RS, src: 7, dst: 6
op: IADD_C7, src: -1, dst: 6
op: IXOR_R, src: 7, dst: 2
op: ISUB_R, src: 0, dst: 7
op: ISUB_R, src: 0, dst: 1
op: IADD_C7, src: -1, dst: 0
op: IXOR_R, src: 6, dst: 1
op: ISUB_R, src: 6, dst: 2
op: IXOR_R, src: 0, dst: 1
op: IMUL_R, src: 2, dst: 0
op: IMUL_R, src: 5, dst: 7
op: IMUL_R, src: 6, dst: 2
op: IROR_C, src: -1, dst: 6
op: IADD_C7, src: -1, dst: 1
op: IXOR_R, src: 4, dst: 6
op: IXOR_R, src: 3, dst: 5
op: ISMULH_R, src: 1, dst: 3
op: IMUL_RCP, src: -1, dst: 5
op: IADD_C9, src: -1, dst: 6
op: IMULH_R, src: 0, dst: 4
op: IMUL_RCP, src: -1, dst: 1
op: IADD_C8, src: -1, dst: 7
op: IROR_C, src: -1, dst: 6
op: IXOR_R, src: 6, dst: 0
op: IADD_C7, src: -1, dst: 2
op: ISUB_R, src: 5, dst: 0
op: ISMULH_R, src: 7, dst: 7
op: IMUL_RCP, src: -1, dst: 3
op: IMUL_R, src: 5, dst: 6
op: IMUL_R, src: 4, dst: 0
op: IROR_C, src: -1, dst: 5
op: IADD_C7, src: -1, dst: 5
op: ISUB_R, src: 4, dst: 2
op: ISUB_R, src: 1, dst: 6
op: IMULH_R, src: 1, dst: 1
op: IMUL_RCP, src: -1, dst: 7
op: IXOR_C9, src: -1, dst: 5
op: IXOR_R, src: 2, dst: 6
op: IADD_RS, src: 2, dst: 4
op: IXOR_C9, src: -1, dst: 2
op: ISUB_R, src: 5, dst: 6
op: IMUL_R, src: 5, dst: 3
op: IMUL_R, src: 2, dst: 6
op: IMUL_R, src: 5, dst: 1
op: IADD_RS, src: 5, dst: 0
op: IADD_RS, src: 0, dst: 7
op: IXOR_C8, src: -1, dst: 4
op: IADD_RS, src: 4, dst: 0
op: IADD_C7, src: -1, dst: 5
op: ISUB_R, src: 7, dst: 2
op: IXOR_R, src: 3, dst: 5
op: IMULH_R, src: 5, dst: 4
op: IMUL_RCP, src: -1, dst: 0
op: IMUL_R, src: 3, dst: 7
op: IMUL_R, src: 6, dst: 2
op: IROR_C, src: -1, dst: 6
op: IADD_C7, src: -1, dst: 5
op: IXOR_R, src: 5, dst: 3
op: IXOR_R, src: 1, dst: 5
op: ISUB_R, src: 1, dst: 3
op: ISUB_R, src: 5, dst: 6
op: IADD_C7, src: -1, dst: 5
op: ISUB_R, src: 0, dst: 1
op: ISUB_R, src: 3, dst: 0
op: IMUL_R, src: 0, dst: 3
op: IMUL_R, src: 2, dst: 5
op: IMUL_R, src: 4, dst: 0
op: IROR_C, src: -1, dst: 7
op: IADD_RS, src: 6, dst: 4
op: IXOR_C8, src: -1, dst: 4
op: IADD_RS, src: 1, dst: 6
op: IADD_RS, src: 6, dst: 2
op: IXOR_C9, src: -1, dst: 7
op: ISUB_R, src: 3, dst: 1
op: IMUL_R, src: 6, dst: 2
op: IMUL_R, src: 6, dst: 7
op: IMUL_R, src: 0, dst: 6
op: IROR_C, src: -1, dst: 3
op: IXOR_R, src: 3, dst: 5
op: IADD_C7, src: -1, dst: 4
op: IXOR_R, src: 4, dst: 1
op: IXOR_R, src: 0, dst: 5
op: ISUB_R, src: 5, dst: 2
op: IXOR_C7, src: -1, dst: 1
op: ISUB_R, src: 0, dst: 5
op: IXOR_R, src: 7, dst: 5
op: IMUL_R, src: 7, dst: 1
op: IMUL_R, src: 4, dst: 2
op: IMUL_R, src: 3, dst: 5
op: IADD_RS, src: 3, dst: 4
op: IADD_RS, src: 7, dst: 6
op: IXOR_C8, src: -1, dst: 4
op: IROR_C, src: -1, dst: 7
op: IXOR_R, src: 4, dst: 0
op: IXOR_C7, src: -1, dst: 1
op: IXOR_R, src: 7, dst: 4
op: IXOR_R, src: 6, dst: 0
op: IMUL_R, src: 0, dst: 7
op: IMUL_R, src: 2, dst: 4
op: IMUL_R, src: 5, dst: 3
op: IROR_C, src: -1, dst: 2
op: IADD_RS, src: 6, dst: 1
op: IADD_C8, src: -1, dst: 2
op: IROR_C, src: -1, dst: 5
op: IADD_RS, src: 0, dst: 1
op: IXOR_C8, src: -1, dst: 2
op: IADD_RS, src: 5, dst: 7
op: IMUL_R, src: 1, dst: 5
op: IMUL_R, src: 0, dst: 2
op: IMUL_R, src: 1, dst: 7
op: IROR_C, src: -1, dst: 0
op: IROR_C, src: -1, dst: 6
op: IADD_C9, src: -1, dst: 1
op: IXOR_R, src: 3, dst: 1
op: IADD_C7, src: -1, dst: 0
op: IXOR_R, src: 4, dst: 6
op: ISUB_R, src: 3, dst: 4
op: IXOR_R, src: 0, dst: 1
op: IMUL_R, src: 2, dst: 4
op: IMUL_R, src: 0, dst: 6
op: IMUL_R, src: 2, dst: 0
op: IROR_C, src: -1, dst: 5
op: IADD_RS, src: 2, dst: 3
op: IADD_C8, src: -1, dst: 1
op: IADD_RS, src: 2, dst: 7
op: IROR_C, src: -1, dst: 2
op: IADD_C9, src: -1, dst: 3
op: ISUB_R, src: 2, dst: 3
op: IMUL_R, src: 7, dst: 1
op: IMUL_R, src: 3, dst: 2
op: IMUL_R, src: 3, dst: 7
op: IADD_RS, src: 5, dst: 4
op: IROR_C, src: -1, dst: 6
op: IXOR_C9, src: -1, dst: 5
op: ISUB_R, src: 4, dst: 6
op: ISUB_R, src: 5, dst: 3
op: IADD_C7, src: -1, dst: 1
op: ISUB_R, src: 4, dst: 5
op: IXOR_R, src: 4, dst: 3
op: IMUL_R, src: 3, dst: 5
op: IMUL_R, src: 0, dst: 6
op: IMUL_R, src: 3, dst: 1
op: IADD_RS, src: 2, dst: 3
op: IADD_RS, src: 7, dst: 2
op: IADD_C8, src: -1, dst: 0
op: IROR_C, src: -1, dst: 7
op: IADD_RS, src: 0, dst: 4
op: IADD_C8, src: -1, dst: 7
op: IADD_RS, src: 2, dst: 0
op: IMUL_R, src: 3, dst: 2
op: IMUL_R, src: 6, dst: 3
op: IMUL_R, src: 6, dst: 4
op: IROR_C, src: -1, dst: 7
op: IADD_RS, src: 5, dst: 7
op: IXOR_C9, src: -1, dst: 0
op: IMULH_R, src: 2, dst: 5
op: IMUL_RCP, src: -1, dst: 6
op: IXOR_C8, src: -1, dst: 7
op: IROR_C, src: -1, dst: 1
op: IADD_C7, src: -1, dst: 3
op: ISUB_R, src: 7, dst: 2
op: ISUB_R, src: 1, dst: 7
op: ISUB_R, src: 2, dst: 1
op: IMUL_R, src: 3, dst: 0
op: IMUL_R, src: 4, dst: 2
op: IMUL_R, src: 1, dst: 5
op: IADD_RS, src: 3, dst: 7
op: ISUB_R, src: 4, dst: 3
op: IADD_C7, src: -1, dst: 3
op: ISUB_R, src: 4, dst: 7
op: ISUB_R, src: 1, dst: 0
op: IADD_RS, src: 3, dst: 6
op: IADD_C9, src: -1, dst: 7
op: IMULH_R, src: 7, dst: 1
op: IMUL_RCP, src: -1, dst: 3
op: IMUL_R, src: 5, dst: 6
op: IMUL_R, src: 5, dst: 0
op: IADD_RS, src: 7, dst: 2
op: IROR_C, src: -1, dst: 7
op: IXOR_C9, src: -1, dst: 7
op: ISMULH_R, src: 2, dst: 4"#;