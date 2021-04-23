#![feature(test)]
#![feature(box_syntax)]

extern crate mithril;
extern crate test;

use mithril::byte_string;
use mithril::randomx::memory::VmMemory;
use mithril::randomx::vm::new_vm;
use std::sync::Arc;
use test::Bencher;

#[bench]
fn bench_hash_light_memory(b: &mut Bencher) {
    let input1 = byte_string::string_to_u8_array("0e0eb1e8de8306117d26f2afad8aa3a83cb0e210622dde0288ff29c45c3514d20f3a660b97307500000000c7980407e38b16dae2ed1b0264fec2b1d7fbbe11c1ffa0dd33f2bf84dee986ef05");
    let input2 = byte_string::string_to_u8_array("0e0ec9e9de8306117d26f2afad8aa3a83cb0e210622dde0288ff29c45c3514d20f3a660b97307500000000868133fcd973a1c9469c889e67286d1518d04ca8e54ad5b2773229a839a28fdc1d");
    let input3 = byte_string::string_to_u8_array("0e0ee0eade8306117d26f2afad8aa3a83cb0e210622dde0288ff29c45c3514d20f3a660b9730750000000065e5a134b8cbd566d434edc85cc124bb2139b77336728d0f01ba88dd0d5ad32c37");
    let seed_hash = "aef2d93d89bcfbe147cdf85ca3827d8a78ef687fd338b4da137ef3b403e7fef5";
    let mem = Arc::new(VmMemory::light(&byte_string::string_to_u8_array(seed_hash)));
    b.iter(|| {
        let mut vm = new_vm(mem.clone());
        vm.calculate_hash(&input1);
        vm.calculate_hash(&input2);
        vm.calculate_hash(&input3);
    });
}

#[bench]
fn bench_hash_full_memory(b: &mut Bencher) {
    let input1 = byte_string::string_to_u8_array("0e0eb1e8de8306117d26f2afad8aa3a83cb0e210622dde0288ff29c45c3514d20f3a660b97307500000000c7980407e38b16dae2ed1b0264fec2b1d7fbbe11c1ffa0dd33f2bf84dee986ef05");
    let input2 = byte_string::string_to_u8_array("0e0ec9e9de8306117d26f2afad8aa3a83cb0e210622dde0288ff29c45c3514d20f3a660b97307500000000868133fcd973a1c9469c889e67286d1518d04ca8e54ad5b2773229a839a28fdc1d");
    let input3 = byte_string::string_to_u8_array("0e0ee0eade8306117d26f2afad8aa3a83cb0e210622dde0288ff29c45c3514d20f3a660b9730750000000065e5a134b8cbd566d434edc85cc124bb2139b77336728d0f01ba88dd0d5ad32c37");
    let seed_hash = "aef2d93d89bcfbe147cdf85ca3827d8a78ef687fd338b4da137ef3b403e7fef5";
    let mem = Arc::new(VmMemory::full(&byte_string::string_to_u8_array(seed_hash)));
    b.iter(|| {
        let mut vm = new_vm(mem.clone());
        vm.calculate_hash(&input1);
        vm.calculate_hash(&input2);
        vm.calculate_hash(&input3);
    });
}
