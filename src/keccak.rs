
//code taken from https://github.com/debris/tiny-keccak

const PLEN: usize = 25;
const TLEN: usize = 144;
const LANES: usize = 25;

const RHO: [u32; 24] = [
     1,  3,  6, 10, 15, 21,
    28, 36, 45, 55,  2, 14,
    27, 41, 56,  8, 25, 43,
    62, 18, 39, 61, 20, 44
];

const PI: [usize; 24] = [
    10,  7, 11, 17, 18, 3,
     5, 16,  8, 21, 24, 4,
    15, 23, 19, 13, 12, 2,
    20, 14, 22,  9,  6, 1
];

const RC: [u64; 24] = [
    1u64, 0x8082u64, 0x800000000000808au64, 0x8000000080008000u64,
    0x808bu64, 0x80000001u64, 0x8000000080008081u64, 0x8000000000008009u64,
    0x8au64, 0x88u64, 0x80008009u64, 0x8000000au64,
    0x8000808bu64, 0x800000000000008bu64, 0x8000000000008089u64, 0x8000000000008003u64,
    0x8000000000008002u64, 0x8000000000000080u64, 0x800au64, 0x800000008000000au64,
    0x8000000080008081u64, 0x8000000000008080u64, 0x80000001u64, 0x8000000080008008u64
];

macro_rules! REPEAT4 {
    ($e: expr) => ( $e; $e; $e; $e; )
}

macro_rules! REPEAT5 {
    ($e: expr) => ( $e; $e; $e; $e; $e; )
}

macro_rules! REPEAT6 {
    ($e: expr) => ( $e; $e; $e; $e; $e; $e; )
}

macro_rules! REPEAT24 {
    ($e: expr, $s: expr) => (
        REPEAT6!({ $e; $s; });
        REPEAT6!({ $e; $s; });
        REPEAT6!({ $e; $s; });
        REPEAT5!({ $e; $s; });
        $e;
    )
}

macro_rules! FOR5 {
    ($v: expr, $s: expr, $e: expr) => {
        $v = 0;
        REPEAT4!({
            $e;
            $v += $s;
        });
        $e;
    }
}

/// keccak-f[1600]
pub fn keccakf(a: &mut [u64; PLEN]) {
    let mut b: [u64; 5] = [0; 5];
    let mut t: u64;
    let mut x: usize;
    let mut y: usize;

    for i in 0..24 {
        // Theta
        FOR5!(x, 1, {
            b[x] = 0;
            FOR5!(y, 5, {
                b[x] ^= a[x + y];
            });
        });

        FOR5!(x, 1, {
            FOR5!(y, 5, {
                a[y + x] ^= b[(x + 4) % 5] ^ b[(x + 1) % 5].rotate_left(1);
            });
        });

        // Rho and pi
        t = a[1];
        x = 0;
        REPEAT24!({
            b[0] = a[PI[x]];
            a[PI[x]] = t.rotate_left(RHO[x]);
        }, {
            t = b[0];
            x += 1;
        });

        // Chi
        FOR5!(y, 5, {
            FOR5!(x, 1, {
                b[x] = a[y + x];
            });
            FOR5!(x, 1, {
                a[y + x] = b[x] ^ ((!b[(x + 1) % 5]) & (b[(x + 2) % 5]));
            });
        });

        // Iota
        a[0] ^= RC[i];
    }
}

fn xorin(dst: &mut [u8], src: &[u8]) {
    for (d, i) in dst.iter_mut().zip(src) {
        *d ^= *i;
    }
}

fn a_mut_bytes(a: &mut [u64; PLEN]) -> &mut [u8; PLEN * 8] {
    unsafe { ::std::mem::transmute(a) }
}

fn transmute_u64(t: &mut [u8; TLEN]) -> &mut [u64; TLEN / 8] {
    unsafe { ::std::mem::transmute(t) }
}

fn pad(dst: &mut [u8], l: usize, rate: usize) {
    println!("pad len {:?}", dst.len());
    let l = l + 1;
    dst[l] = 1;
    dst[rate - 1] |= 0x80;
}

pub fn keccak(input: &[u8]) -> [u64; PLEN] {

    let mut a: [u64; PLEN] = [0; PLEN];
    let init_rate = 136; //200 - 512/4;
    let mut rate = init_rate;
    let inlen = input.len();
    let mut tmp: [u8; TLEN] = [0; TLEN];
    tmp[..inlen].copy_from_slice(input);

    println!("rate {:?}", rate);
    print!("input: ");
    for i in 0..input.len() {
        print!("{:02x}", input[i]);
    }
    println!("");
    print!("tmp init: ");
    for i in 0..input.len() {
        print!("{:02x}", tmp[i]);
    }
    println!("");

    //first foldp
    let mut ip = 0;
    let mut l = inlen;
    while l >= rate {
        println!("new round");
        xorin(&mut a_mut_bytes(&mut a)[0..][..rate], &input[ip..]);
        keccakf(&mut a);
        ip += rate;
        l -= rate;
        rate = init_rate;
    }

    //pad
    tmp[inlen] = 1;
    tmp[rate - 1] |= 0x80;

    print!("after pad (tmp): ");
    for i in 0..TLEN {
        print!("{:02x}", tmp[i]);
    }
    println!("");

    let t64 = transmute_u64(&mut tmp);
    println!("####xoring");
    for i in 0..(rate/8) {
        println!("a[{:?}]={:02x} t64[{:?}]={:02x}", i, a[i], i, t64[i]);
        a[i] ^= t64[i];
    }
    println!("####xoring");

    println!("after xor: ");
    for i in 0..LANES {
        print!("{:02x}", a[i]);
    }
    println!("");

    keccakf(&mut a);
    return a;
}
