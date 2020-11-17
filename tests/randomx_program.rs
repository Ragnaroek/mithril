extern crate mithril;
#[macro_use(assert_diff)]
extern crate difference;

use mithril::randomx::hash::{gen_program};
use mithril::randomx::m128::{m128i};
use mithril::randomx::program::{from_bytes};

#[test]
fn test_decode_program_1000() {
    let bytes = gen_test_program_nonce_1000();
    let program = from_bytes(bytes);
    assert_diff!(EXPECTED_OUT_NONCE_1000, &program.to_string(), "\n", 0);
}

#[test]
fn test_decode_program_1002() {
    let bytes = gen_test_program_nonce_1002();
    let program = from_bytes(bytes);
    assert_diff!(EXPECTED_OUT_NONCE_1002, &program.to_string(), "\n", 0);
}

#[test]
fn test_decode_program_666() {
    let bytes = gen_test_program_nonce_666();
    let program = from_bytes(bytes);
    assert_diff!(EXPECTED_OUT_NONCE_666, &program.to_string(), "\n", 0);
}

#[allow(overflowing_literals)]
fn gen_test_program_nonce_1000() -> Vec<m128i> {
    let input0 = m128i::from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let input1 = m128i::from_i32(0xb5a8ef67, 0x749809c8, 0xf349884a, 0x05c9f5ef);
    let input2 = m128i::from_i32(0xa9a93ab0, 0x22e46d0a, 0x1a1fe305, 0xb42708c0);
    let input3 = m128i::from_i32(0x68247034, 0xed99ee84, 0x438f563a, 0x138612ff);
    
    let input:[m128i;4] = [input0, input1, input2, input3];
    gen_program(input, 136)
}

#[allow(overflowing_literals)]
fn gen_test_program_nonce_1002() -> Vec<m128i> {
    let input0 = m128i::from_i32(0xd0e8695f, 0x310cd519, 0xd5904b69, 0xb7f63f45);
    let input1 = m128i::from_i32(0x02ebf53a, 0x570aa2e9, 0x738c8a99, 0xec7f686a);
    let input2 = m128i::from_i32(0xca86dba0, 0xa5d073bc, 0x1fdd7a0f, 0xb16fae80);
    let input3 = m128i::from_i32(0x064195be, 0x2cd32a66, 0x21d727dd, 0xe5d1da28);
    
    let input:[m128i;4] = [input0, input1, input2, input3];
    gen_program(input, 136)
}

#[allow(overflowing_literals)]
fn gen_test_program_nonce_666() -> Vec<m128i> {
    let input0 = m128i::from_i32(0x5368fad2, 0x0a4abde8, 0xccc9e57a, 0x6fb70839);
    let input1 = m128i::from_i32(0x86153ab9, 0x0daaff46, 0x1b6745d3, 0x30f5fee7);
    let input2 = m128i::from_i32(0x5fb74742, 0x686fad2b, 0x7da05aa6, 0x4ef450fd);
    let input3 = m128i::from_i32(0xbc9ce7b6, 0x1d52c94c, 0x650bea68, 0x898b02d9);
    
    let input:[m128i;4] = [input0, input1, input2, input3];
    gen_program(input, 136)
}

const EXPECTED_OUT_NONCE_1000 : &str = r#"FMUL_R e0, a3
FMUL_R e0, a1
CBRANCH r2, -1630366089, COND 9
IMUL_R r6, r7
IXOR_R r7, r3
CBRANCH r0, 2028867863, COND 2
IMUL_R r6, r0
CBRANCH r0, 411377654, COND 10
FSUB_R f1, a3
ISUB_R r6, r7
FMUL_R e1, a1
ISTORE L1[r5-241860501], r0
FMUL_R e3, a1
FADD_R f0, a0
FDIV_M e1, L1[r6-1254815598]
FSUB_R f1, a1
FADD_R f1, a1
FMUL_R e3, a3
ISTORE L1[r5+184502782], r7
ISUB_R r0, r1
FSUB_R f1, a3
IXOR_R r0, r5
FDIV_M e1, L1[r0-77199149]
FMUL_R e2, a1
IXOR_M r6, L1[r7-2037661772]
FSUB_R f3, a0
ISUB_R r7, r4
FMUL_R e2, a2
ISUB_R r7, r4
CBRANCH r2, 1442104762, COND 9
FMUL_R e0, a1
ISUB_M r3, L1[r7-490209812]
IMUL_M r7, L2[r4+963927534]
IXOR_R r3, 1448754900
IMUL_RCP r0, 2468050740
FSUB_R f1, a0
FMUL_R e2, a1
IROL_R r2, r7
IMUL_R r0, r4
IADD_RS r1, r0, SHFT 3
CBRANCH r0, 1256279335, COND 7
FSUB_R f0, a1
IADD_M r7, L3[1299904]
FSQRT_R e0
IADD_RS r2, r1, SHFT 1
ISTORE L2[r0-1820125421], r0
IMUL_RCP r4, 2310636501
IMUL_R r4, -1297675806
FSCAL_R f2
ISUB_R r1, r3
ISUB_M r7, L2[r2-748343651]
ISWAP_R r5, r2
FSUB_R f1, a2
IADD_RS r3, r5, SHFT 1
IXOR_R r6, r0
FSUB_R f1, a3
FSUB_R f1, a2
CBRANCH r1, -279795887, COND 10
ISUB_R r3, r4
FSWAP_R f3
CBRANCH r3, 2026498346, COND 5
IMUL_M r5, L1[r7-152660037]
IMUL_M r5, L1[r3+111909964]
FMUL_R e2, a0
ISMULH_R r0, r1
FDIV_M e1, L1[r2-1741423952]
ISMULH_R r2, r4
FDIV_M e1, L1[r4-1624788056]
IADD_RS r3, r5, SHFT 3
ISUB_R r2, r0
FSUB_M f2, L1[r5+672776253]
ISUB_M r5, L2[r3-1499902540]
IADD_RS r4, r1, SHFT 3
FSUB_M f2, L1[r0+34382801]
IROR_R r5, r3
IMUL_R r0, r5
ISTORE L2[r6+309808466], r6
FMUL_R e3, a3
FSUB_R f3, a1
IMUL_R r5, 570452629
IXOR_R r0, r4
FMUL_R e1, a1
ISUB_R r5, 96447373
FSCAL_R f3
IXOR_R r4, r7
FDIV_M e3, L1[r6+2008404126]
FADD_R f3, a0
ISUB_M r5, L1[r6-50500738]
IMUL_R r6, r0
IMUL_R r5, r7
ISTORE L1[r1+2118002880], r5
FMUL_R e3, a2
FSWAP_R f1
ISWAP_R r7, r3
ISUB_M r5, L1[r7+404453794]
CFROUND r7, 9
IXOR_M r3, L1[r2-1034633132]
IROR_R r7, r0
IADD_M r3, L2[r2+518458598]
ISUB_M r7, L2[r0+679142455]
ISTORE L3[r3+698711442], r2
FSQRT_R e1
FSUB_R f3, a2
IXOR_R r3, r6
FSUB_R f3, a0
IXOR_R r0, r2
ISMULH_R r6, r4
FADD_R f3, a3
IADD_M r1, L1[r4-1589699100]
CBRANCH r4, -2001621287, COND 12
CBRANCH r6, 117197499, COND 1
CFROUND r3, 31
ISTORE L3[r5+2045065871], r6
ISTORE L2[r0-1627689177], r0
IADD_RS r4, r0, SHFT 3
ISUB_R r0, r3
CBRANCH r6, -1583952796, COND 9
FMUL_R e1, a0
ISTORE L2[r2-1388387726], r0
FSUB_M f1, L1[r3-1680412010]
IROR_R r7, r4
INEG_R r4
ISWAP_R r1, r0
ISTORE L1[r2+1135703162], r7
FSWAP_R f1
ISTORE L3[r3-2096947159], r4
IADD_RS r0, r4, SHFT 2
CBRANCH r7, 717223143, COND 14
CBRANCH r3, -250805577, COND 13
IADD_RS r7, r3, SHFT 2
IMUL_RCP r6, 1526874357
FDIV_M e3, L2[r5-807611906]
FADD_R f3, a2
FMUL_R e0, a3
ISUB_M r6, L1[r4-2039653197]
FMUL_R e3, a2
ISUB_R r6, 1421357849
ISTORE L1[r7+552369479], r6
CBRANCH r5, -1825629749, COND 7
ISTORE L1[r0+1323579009], r4
FSUB_R f1, a1
IMUL_R r1, r2
IXOR_R r5, r6
IXOR_M r6, L1[r1+914887014]
IMUL_R r0, r5
ISTORE L1[r6-1838972956], r3
FADD_M f3, L2[r3+1706223267]
IMULH_R r0, r0
FSCAL_R f3
IMUL_R r1, r2
CBRANCH r5, -409740775, COND 11
CBRANCH r3, 1951602487, COND 6
ISUB_R r1, r4
IADD_M r3, L1[r1+114570948]
FSUB_R f2, a2
IADD_M r7, L1[r6-2120644382]
FMUL_R e0, a3
IROR_R r1, r7
IMUL_R r4, r5
ISUB_R r2, r3
IXOR_M r1, L3[43968]
IADD_RS r4, r5, SHFT 2
IADD_RS r3, r6, SHFT 3
IXOR_R r1, r7
IROR_R r5, r6
FADD_R f0, a3
CBRANCH r3, 2137910998, COND 11
ISUB_M r4, L1[r2-1710082608]
ISTORE L3[r6+2101899920], r1
CBRANCH r1, -1600211899, COND 7
ISTORE L1[r7-888298740], r0
FADD_R f1, a3
IXOR_M r5, L2[r7-499685672]
CBRANCH r2, -803868677, COND 1
CBRANCH r6, 1634965261, COND 0
IXOR_R r1, r5
FADD_R f3, a1
IMUL_M r3, L1[r6+1828758187]
IMULH_M r5, L1[r3-258712815]
IXOR_M r1, L3[905176]
FMUL_R e0, a1
IADD_RS r4, r2, SHFT 3
CBRANCH r1, -1987557739, COND 12
FADD_R f0, a1
FMUL_R e3, a3
FSWAP_R e0
IMUL_RCP r0, 3775613376
IMULH_R r6, r2
FMUL_R e3, a2
IMUL_RCP r3, 2837353026
FSUB_R f3, a0
FMUL_R e3, a3
IADD_M r5, L1[r4+385175108]
CBRANCH r3, 384176278, COND 6
FSCAL_R f1
FSUB_M f3, L1[r5-1828439736]
ISTORE L1[r5-966638834], r1
IXOR_R r1, r5
INEG_R r2
FADD_R f0, a3
ISTORE L2[r3+1488616740], r6
FMUL_R e2, a1
FSQRT_R e2
CBRANCH r6, 1682114671, COND 14
IROR_R r2, r5
ISUB_R r6, r0
ISUB_R r7, r0
CFROUND r2, 21
IROR_R r7, r5
IMULH_R r1, r7
FSCAL_R f0
FSUB_R f0, a2
IROR_R r5, r3
IXOR_R r3, r1
IMUL_RCP r7, 4050336879
FADD_R f3, a2
FMUL_R e3, a3
IMUL_R r0, r2
IADD_M r4, L1[r6+2094259989]
FSUB_R f3, a0
ISWAP_R r0, r7
ISWAP_R r7, r1
CBRANCH r7, -2081336905, COND 0
FADD_R f2, a0
IADD_M r0, L1[r1-1925689157]
IMULH_R r3, r6
IADD_RS r6, r6, SHFT 3
IXOR_R r4, r2
IADD_RS r2, r4, SHFT 0
IXOR_R r6, r2
IADD_M r6, L1[r5+395944839]
ISTORE L3[r6-661024501], r1
CBRANCH r3, 482346307, COND 0
FDIV_M e1, L1[r5+424616842]
IMUL_R r7, -924237797
IMUL_R r5, r7
ISTORE L1[r6-1683376019], r2
FADD_R f0, a1
FMUL_R e2, a1
FSCAL_R f3
ISTORE L2[r3+1379331534], r0
FADD_R f1, a2
ISUB_R r0, -651572469
CBRANCH r6, -731112719, COND 5
FSUB_R f1, a2
CBRANCH r7, -1495751957, COND 13
IROR_R r0, r4
IMUL_M r3, L1[r0-1477618562]
IROR_R r3, r0
FADD_R f0, a1
IMULH_R r3, r0
ISUB_M r3, L3[1012760]
IADD_RS r5, r4, 1970426166, SHFT 2
FSUB_R f3, a1
FSUB_R f3, a0
IADD_RS r0, r7, SHFT 0
"#;

const EXPECTED_OUT_NONCE_1002 : &str = r#"IADD_RS r1, r0, SHFT 2
ISUB_R r4, r3
IXOR_R r0, r2
FSUB_R f3, a1
ISMULH_M r7, L1[r4+285982547]
IADD_RS r5, r3, 2085749249, SHFT 1
CBRANCH r2, 140352333, COND 9
CBRANCH r6, -1617054832, COND 12
IROL_R r7, r0
FSUB_R f1, a0
ISTORE L3[r7+253197245], r6
FSUB_R f2, a3
FADD_M f0, L1[r6+1105858426]
CBRANCH r6, -1821588344, COND 13
FSQRT_R e3
ISUB_R r1, r0
FSUB_R f2, a0
FMUL_R e3, a3
ISUB_R r5, r1
IXOR_R r6, r2
IROR_R r0, r3
FSCAL_R f2
IMUL_R r1, r7
CBRANCH r5, 400605342, COND 10
IROR_R r5, r0
CBRANCH r5, -313118350, COND 0
IMUL_M r2, L1[r5-2040259463]
FSUB_R f0, a3
CBRANCH r7, -429562911, COND 5
IADD_RS r2, r0, SHFT 0
IROR_R r7, r0
ISTORE L2[r5-1829524160], r7
FMUL_R e3, a1
FSUB_R f3, a2
ISTORE L1[r5-1458490691], r0
CBRANCH r0, 1319054938, COND 0
IXOR_R r2, r0
FMUL_R e0, a1
FSCAL_R f3
IMULH_M r4, L1[r0+925884864]
FSUB_R f0, a1
IADD_RS r7, r4, SHFT 2
ISUB_R r6, r1
IADD_M r6, L1[r2-476182082]
IMUL_RCP r7, 2693133392
FADD_M f0, L2[r1+1407972115]
IMULH_R r7, r5
CBRANCH r6, 685052144, COND 6
FMUL_R e3, a0
ISUB_R r1, 196093907
FSUB_R f0, a1
IMUL_RCP r1, 3141556559
FADD_R f1, a3
FADD_R f0, a2
FSUB_R f1, a1
ISTORE L1[r1-396897627], r4
IMUL_M r5, L1[r6-1165962035]
ISUB_R r4, r0
ISUB_R r1, r2
FADD_R f3, a0
ISTORE L1[r2-1953418527], r0
IXOR_R r6, r2
IMUL_M r4, L1[r3+345008930]
IADD_M r7, L1[r3+157068585]
IROR_R r7, r5
FSUB_R f3, a2
ISTORE L1[r4-958006101], r3
FADD_R f1, a3
FSWAP_R e2
FMUL_R e2, a3
CBRANCH r1, 1861849632, COND 15
ISUB_R r1, r2
FSUB_M f0, L1[r3-85077338]
CBRANCH r4, 2086507249, COND 11
CBRANCH r5, 1049871309, COND 7
CBRANCH r4, 191109515, COND 0
FDIV_M e3, L1[r6+158775741]
IADD_RS r7, r1, SHFT 2
IROR_R r0, 52
ISUB_M r2, L3[1553360]
ISMULH_R r4, r5
IMUL_R r2, r7
IMUL_R r0, r7
FSCAL_R f1
FADD_R f3, a2
FSWAP_R e2
ISTORE L1[r6+1487089801], r6
FSWAP_R f3
IMUL_R r3, r1
IROR_R r0, r6
IADD_M r2, L1[r6+604474343]
ISUB_R r4, r0
IMUL_R r6, r3
FADD_M f1, L1[r7+962588452]
IMULH_R r2, r5
IMUL_R r4, 1913619886
FMUL_R e2, a0
CBRANCH r3, 61151805, COND 5
ISUB_M r5, L2[r6-1647879627]
IROL_R r5, r3
FADD_R f0, a3
ISUB_M r0, L1[r6-1160185537]
IADD_M r0, L2[r2+400903501]
CBRANCH r2, 221672894, COND 0
IMUL_R r2, r6
IMUL_M r3, L1[r1+1078146353]
IMULH_M r1, L1[r7-1811278368]
IROR_R r4, r5
FADD_R f0, a0
ISTORE L1[r4-584134522], r0
ISTORE L1[r3-1252505751], r0
IADD_M r3, L1[r0-1177497194]
IXOR_R r1, r6
FMUL_R e0, a0
ISTORE L3[r0-274342510], r0
INEG_R r0
IMUL_R r6, r0
IMUL_R r3, 176747896
ISUB_M r2, L1[r4-1048006796]
FMUL_R e2, a2
ISTORE L3[r5-561552293], r2
FSUB_M f3, L1[r4+1851951846]
IROR_R r7, r0
IADD_RS r1, r5, SHFT 1
IMUL_R r3, r1
CBRANCH r2, -1059734824, COND 7
FMUL_R e0, a3
IXOR_R r7, r1
IMUL_R r7, r6
IADD_M r7, L1[r0-1009739268]
CBRANCH r0, -1023228623, COND 2
ISMULH_M r4, L1[r5-1182625007]
IROR_R r3, r1
IADD_M r2, L1[r4-2123537054]
IMUL_M r3, L1[r5-1287165928]
CBRANCH r6, 1700147983, COND 8
IROR_R r2, r1
FADD_R f0, a1
IADD_RS r6, r2, SHFT 1
CBRANCH r1, -39165600, COND 13
IMUL_RCP r2, 573658069
IMUL_M r2, L2[r1+681947937]
CBRANCH r6, 1798561326, COND 10
FMUL_R e2, a0
FSWAP_R f1
FMUL_R e0, a2
ISTORE L2[r0-2024327846], r6
FMUL_R e2, a2
IXOR_M r3, L1[r1-1850071558]
FDIV_M e2, L1[r1-1451548484]
ISUB_R r5, r1
FADD_R f1, a0
FADD_R f3, a2
FSUB_R f3, a1
IMUL_RCP r2, 826755405
CBRANCH r6, -1293302966, COND 10
FSCAL_R f3
IMUL_R r3, r7
FSUB_R f0, a0
CBRANCH r5, 708755166, COND 9
IXOR_R r5, r3
FMUL_R e2, a2
CBRANCH r5, -1927843031, COND 8
IROR_R r0, r3
IMUL_M r2, L1[r3-709600307]
FMUL_R e0, a3
ISMULH_R r4, r0
FADD_R f0, a1
IMUL_R r3, r1
FSUB_M f3, L1[r0+530175244]
FSUB_M f0, L1[r2+1466755873]
ISUB_R r6, r2
FMUL_R e1, a0
FSUB_M f2, L1[r2+2125195082]
IADD_RS r5, r6, 343285809, SHFT 3
IADD_RS r5, r3, 1472936148, SHFT 3
ISUB_M r1, L1[r0+35083177]
ISTORE L1[r7+1390831002], r7
CBRANCH r7, -743260954, COND 3
IMUL_R r3, -1221867900
IADD_RS r0, r3, SHFT 2
FADD_M f0, L1[r7+765360996]
IROR_R r0, r1
IMUL_R r1, r5
IADD_M r7, L3[1703680]
ISTORE L3[r6+226092782], r6
FSWAP_R f1
IROR_R r3, r1
FSUB_R f3, a2
ISUB_M r5, L1[r0+675425680]
ISTORE L3[r5-1087908418], r1
FMUL_R e2, a2
IMUL_R r6, 1742181645
CBRANCH r1, -1214270619, COND 15
FSCAL_R f1
CBRANCH r6, -854566191, COND 4
IMUL_RCP r2, 119689157
IMUL_RCP r1, 807228158
IADD_M r5, L1[r2+1469393820]
FMUL_R e3, a0
FDIV_M e1, L1[r2-292388143]
CBRANCH r4, 190568982, COND 1
IROR_R r7, r2
FSWAP_R f0
CBRANCH r0, 974405303, COND 15
FADD_M f1, L2[r7-1472964628]
ISMULH_R r6, r4
FMUL_R e2, a2
FMUL_R e2, a3
IMULH_R r5, r7
IXOR_R r4, r3
ISTORE L1[r5+980437994], r3
FSQRT_R e3
FMUL_R e0, a0
IXOR_R r1, r2
IXOR_R r0, r6
FADD_R f2, a2
FADD_R f2, a1
IADD_RS r4, r0, SHFT 2
FMUL_R e2, a3
ISUB_R r0, r2
IXOR_R r2, r0
ISTORE L1[r2-636816373], r6
FMUL_R e1, a1
IXOR_R r5, r3
IROR_R r1, r0
IXOR_R r5, r0
FSUB_R f1, a1
ISMULH_R r5, r7
IXOR_R r4, r0
FMUL_R e1, a2
IADD_RS r4, r7, SHFT 3
ISTORE L2[r6-915918021], r3
CBRANCH r7, 736762870, COND 2
FSCAL_R f3
FMUL_R e1, a0
FSCAL_R f0
IMUL_RCP r7, 2689611840
FMUL_R e0, a1
FSUB_R f1, a3
ISWAP_R r6, r2
FSQRT_R e3
ISUB_R r3, r0
IMUL_M r3, L1[r2-479469103]
IXOR_R r4, r1
FSUB_M f1, L1[r5-1603320983]
FMUL_R e2, a1
FSUB_R f3, a1
IADD_RS r4, r5, SHFT 2
FSUB_R f1, a3
IMUL_R r7, r2
FADD_R f1, a3
FMUL_R e2, a0
FSQRT_R e2
CBRANCH r7, -1114067541, COND 5
ISUB_R r1, r2
"#;

const EXPECTED_OUT_NONCE_666 : &str = r#"FADD_R f1, a1
IADD_M r4, L3[868576]
IXOR_R r5, r3
FSUB_R f0, a1
ISUB_M r0, L1[r1-1853665508]
ISUB_M r1, L2[r2-323745888]
IXOR_R r5, r7
FSCAL_R f0
CBRANCH r4, -287101952, COND 6
IADD_M r0, L1[r5+811378945]
FMUL_R e0, a1
ISUB_M r1, L1[r2-1516661560]
ISMULH_R r7, r5
ISUB_R r4, r0
INEG_R r3
IMUL_R r3, r0
IROR_R r6, r7
CBRANCH r7, -1853246667, COND 8
IADD_RS r3, r5, SHFT 1
IXOR_R r0, r1
IMUL_R r1, r0
ISUB_M r0, L1[r1-407200231]
ISWAP_R r6, r6
FSQRT_R e3
FADD_R f1, a3
CBRANCH r1, -1980918442, COND 15
IADD_RS r3, r3, SHFT 0
IROR_R r1, r4
FSUB_M f2, L1[r1+1150287044]
FDIV_M e2, L2[r5+1750696904]
ISUB_R r2, r1
FSUB_R f3, a1
FSQRT_R e3
ISUB_R r5, r0
IXOR_R r4, 1877858599
ISMULH_R r4, r5
FADD_R f0, a3
ISUB_R r7, r4
CBRANCH r5, -1765392988, COND 9
IMUL_R r1, r5
CBRANCH r0, 1188542674, COND 1
FSCAL_R f1
ISTORE L1[r5+1295709664], r7
FMUL_R e0, a1
FDIV_M e2, L1[r3-1198617727]
FMUL_R e0, a2
FSWAP_R f0
FDIV_M e3, L1[r1+2058221085]
FMUL_R e0, a2
ISTORE L1[r1-36986794], r4
INEG_R r3
CBRANCH r0, 1483509827, COND 11
IADD_M r2, L1[r7+1950101834]
ISUB_R r0, r1
FSUB_M f0, L1[r1-1401501511]
IMUL_R r4, r6
FMUL_R e3, a0
IADD_M r6, L2[r3-1919840048]
FMUL_R e1, a0
ISTORE L1[r5-1998600913], r0
FMUL_R e3, a0
CBRANCH r2, -1762595203, COND 4
IADD_M r3, L3[1322448]
ISTORE L2[r4+82309411], r4
CBRANCH r6, 1217808930, COND 15
CBRANCH r5, -1311186227, COND 15
FDIV_M e0, L1[r4+2033396012]
IMUL_R r2, r3
IADD_M r0, L1[r1+432363973]
FSWAP_R f3
ISUB_M r2, L2[r0+1480308490]
FADD_R f3, a2
FADD_R f1, a3
IMUL_R r0, r1
FADD_R f1, a0
CFROUND r1, 57
INEG_R r7
IADD_RS r7, r5, SHFT 0
IMUL_RCP r5, 2189173901
IMUL_RCP r3, 3677077642
IMUL_R r2, r1
FSUB_M f2, L1[r2-547887621]
ISMULH_R r5, r5
ISUB_R r0, r2
FSUB_M f1, L1[r1-1155493106]
ISUB_M r5, L1[r0-1050506834]
FSQRT_R e3
ISWAP_R r1, r4
ISWAP_R r1, r7
ISTORE L1[r4+844766657], r5
IMUL_M r5, L3[350888]
IMUL_M r0, L1[r2-2008668744]
FSUB_R f3, a2
IROR_R r4, r7
FMUL_R e1, a2
FMUL_R e2, a2
FSUB_M f2, L1[r6-1647410476]
CBRANCH r6, 2079918581, COND 13
FMUL_R e1, a0
IMUL_R r6, r3
FMUL_R e0, a0
IADD_RS r7, r1, SHFT 1
ISUB_M r5, L1[r7+1845119497]
IADD_RS r6, r1, SHFT 3
IXOR_R r4, r3
FSUB_R f1, a0
CBRANCH r3, 1606366055, COND 7
ISUB_R r6, r3
ISTORE L2[r0+1182629685], r5
ISUB_M r1, L1[r4-517600139]
FMUL_R e3, a3
IADD_RS r7, r5, SHFT 1
FSUB_R f3, a1
ISUB_M r5, L1[r2+2123655698]
FMUL_R e1, a2
FMUL_R e1, a3
CBRANCH r2, -368882563, COND 7
FMUL_R e0, a3
IXOR_M r2, L1[r4+2073724322]
IMUL_M r1, L1[r2+1495539107]
IROR_R r5, r3
IROL_R r5, 29
FMUL_R e0, a0
IADD_M r7, L1[r3+380648575]
ISUB_R r4, r0
IROR_R r3, r4
FSCAL_R f3
IMUL_R r7, r2
FMUL_R e0, a0
ISUB_R r0, r1
ISTORE L2[r5-2118886057], r1
FSUB_R f3, a2
FMUL_R e2, a2
CBRANCH r6, 2082825183, COND 13
CBRANCH r1, 1863744558, COND 10
FMUL_R e3, a0
ISMULH_R r6, r5
FSUB_R f1, a2
IXOR_R r0, r5
FSUB_R f3, a1
FADD_R f1, a1
IXOR_R r2, 1851414861
ISUB_R r5, r3
IMUL_R r1, r5
IMUL_R r1, r3
ISUB_M r4, L1[r5-63331218]
FMUL_R e0, a1
FMUL_R e0, a3
FMUL_R e2, a2
IMUL_RCP r0, 1062647089
IMUL_R r4, r6
FSCAL_R f1
FSCAL_R f3
FMUL_R e0, a3
ISWAP_R r6, r4
FSUB_R f1, a0
ISUB_R r3, r2
IMUL_RCP r6, 621470798
IMULH_R r7, r7
FADD_R f3, a2
FADD_R f0, a2
ISUB_M r5, L1[r7-1372238108]
ISTORE L3[r1+1684405281], r4
IMUL_R r3, r6
FADD_R f0, a1
CBRANCH r6, 1059166032, COND 2
FSWAP_R f2
CBRANCH r2, 1718562081, COND 2
IROR_R r0, 56
CBRANCH r7, -1555122919, COND 6
FSWAP_R f3
IMUL_M r1, L2[r7+620479680]
FADD_R f1, a0
IXOR_R r3, r5
FMUL_R e2, a2
FSUB_R f3, a0
IXOR_M r3, L1[r1+848730726]
IMUL_R r1, r5
CBRANCH r2, 1108248885, COND 13
FSWAP_R e3
ISUB_M r4, L2[r6+1621448335]
IXOR_R r4, r3
FSUB_R f0, a0
ISTORE L2[r0+1698708443], r0
IXOR_R r4, r2
FADD_M f1, L1[r1+1032232593]
ISMULH_M r0, L1[r5-933785634]
ISTORE L1[r2-821347337], r7
FMUL_R e3, a0
FSUB_R f2, a1
FADD_M f3, L1[r1-763787362]
IXOR_R r0, r7
IMUL_R r0, r6
CBRANCH r1, 1949163293, COND 8
IADD_M r5, L2[r7-22875781]
FSQRT_R e1
ISUB_M r6, L1[r2-730081571]
FADD_R f0, a1
ISUB_R r1, r0
IMULH_R r2, r1
FADD_R f1, a3
IADD_RS r1, r4, SHFT 0
CBRANCH r7, 1586531303, COND 5
FADD_R f1, a1
CBRANCH r3, -1517524448, COND 2
IADD_RS r5, r4, -1231240314, SHFT 1
IADD_RS r5, r2, 1497723896, SHFT 1
IROR_R r3, 53
FMUL_R e1, a3
IMUL_R r4, r0
CBRANCH r2, -1188629257, COND 3
ISMULH_R r6, r2
IROR_R r1, r6
CBRANCH r7, 358324548, COND 8
IMUL_R r1, r7
CBRANCH r6, 2007972966, COND 0
CBRANCH r1, -398514214, COND 11
IXOR_R r5, r2
FMUL_R e2, a3
FSUB_M f2, L1[r1-1153640752]
IXOR_M r2, L2[r4-227197249]
FADD_M f0, L1[r6-2118303640]
CBRANCH r2, 2129007003, COND 11
FADD_R f2, a2
FDIV_M e0, L1[r3+1477536790]
FSUB_R f1, a1
IMUL_M r7, L2[r2+1588224173]
CBRANCH r4, 1686536929, COND 3
FSCAL_R f2
FSUB_R f3, a1
IXOR_R r0, -881360696
FSWAP_R f0
FSWAP_R f2
CBRANCH r4, -26888721, COND 9
IROR_R r1, r7
IXOR_R r7, r4
CBRANCH r3, 1354722312, COND 12
CBRANCH r2, -769951000, COND 2
IXOR_M r5, L1[r3-1895489792]
ISUB_M r3, L1[r1+485945084]
CBRANCH r3, 451922100, COND 6
ISUB_R r3, r7
IXOR_R r1, r5
FMUL_R e1, a3
FSUB_R f3, a3
IADD_RS r1, r5, SHFT 3
CBRANCH r3, 672565745, COND 6
ISUB_R r3, 1836399737
IADD_RS r3, r0, SHFT 2
IMUL_R r2, r0
FMUL_R e3, a1
IXOR_R r1, r0
FSCAL_R f1
ISUB_R r5, r2
CBRANCH r3, -1910256508, COND 1
IADD_RS r0, r7, SHFT 3
"#;
