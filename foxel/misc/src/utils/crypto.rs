#![allow(unused)]
/**
 * This is supposed to be a cool external crate
 * but I wrote it here for some reason ;p so yea 
 * not sure why but added encryption/decryption routines
 * to VM instruction and based on how CTR works we can essentially
 * just use aes_encrypt on the same data and skip aes_decrypt again,
 * you can once again use various cipher modes just add them here
 * and dont be an idiot :D
 */
pub const unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> () {
    let mut i = 0;
    while i < len {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
        i += 1;
    }
    return;
}

macro_rules! xtime {
    ($x:expr) => {
        (($x) << 1) ^ (((($x) >> 7) & 1) * 0x1B)
    }
}

const DIMENSION: usize = 4;
const AES_BLOCK: usize = DIMENSION*DIMENSION;

#[derive(Copy, Clone)]
pub struct KeyWrapper {
    pub round_key   : [u8; AES_BLOCK * 11],
    pub key         : [u8; AES_BLOCK     ],
    pub nonce       : [u8; AES_BLOCK     ],
}


const SBOX: [u8;256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

const INV_SBOX: [u8;256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d
];

const fn add_round_key(
    state: &mut [u8],
    round_key: &mut [u8],
    offset: usize
) -> () {

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < DIMENSION {
        while j < DIMENSION {
            state[(i << 2) + j] ^= round_key[offset + ((i << 2) + j)];
            j += 1;
        }
        i += 1;
    }

    return;
}

const fn sub_bytes(state: &mut [u8], sbox: &[u8]) -> () {

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < DIMENSION {
        while j < DIMENSION {
            state[(i << 2) + j] = sbox[state[(i << 2) + j] as usize];
            j += 1;
        }
        i += 1;
    }
    return;
}


const fn shift_rows(state: &mut [u8]) -> () {

    let mut temp: u8    = state[1 ];
    state[1 ]           = state[5 ];
    state[5 ]           = state[9 ];
    state[9 ]           = state[13];
    state[13]           = temp;
        
    temp                = state[2 ];
    state[2 ]           = state[10];
    state[10]           = temp;
    temp                = state[6 ];
    state[6 ]           = state[14];
    state[14]           = temp;

    temp                = state[3 ];
    state[3 ]           = state[15];
    state[15]           = state[11];
    state[11]           = state[7 ];
    state[7 ]           = temp;

    return;
}

const fn inv_shift_rows(state: &mut [u8]) -> () {

    let mut temp: u8    = state[13];
    state[13]           = state[9 ];
    state[9 ]           = state[5 ];
    state[5 ]           = state[1 ];
    state[1 ]           = temp;

    temp                = state[2 ];
    state[2 ]           = state[10];
    state[10]           = temp;
    temp                = state[6 ];
    state[6 ]           = state[14];
    state[14]           = temp;

    temp                = state[3 ];
    state[3 ]           = state[7 ];
    state[7 ]           = state[11];
    state[11]           = state[15];
    state[15]           = temp;

    return;
}

const fn mix_columns(state: &mut [u8]) -> () {

    let mut t: u8;
    let mut u: u8;

    let mut i: usize = 0;
    while i < DIMENSION {
        u = state[ i << 2     ];
        t = state[ i << 2     ] ^ 
            state[(i << 2) + 1] ^ 
            state[(i << 2) + 2] ^ 
            state[(i << 2) + 3];

        state[ i << 2     ] ^= t ^ xtime!(state[ i << 2     ] ^ state[(i << 2) + 1]);
        state[(i << 2) + 1] ^= t ^ xtime!(state[(i << 2) + 1] ^ state[(i << 2) + 2]);
        state[(i << 2) + 2] ^= t ^ xtime!(state[(i << 2) + 2] ^ state[(i << 2) + 3]);
        state[(i << 2) + 3] ^= t ^ xtime!(state[(i << 2) + 3] ^ u);

        i += 1;
    }

    return;
}

const fn inv_mix_columns(state: &mut [u8]) -> () {

    let mut u: u8;
    let mut v: u8;

    let mut i: usize = 0;
    while i < DIMENSION {
        u = xtime!(xtime!(state[ i << 2     ] ^ state[(i << 2) + 2]));
        v = xtime!(xtime!(state[(i << 2) + 1] ^ state[(i << 2) + 3]));

        state[ i << 2     ] ^= u;
        state[(i << 2) + 1] ^= v;
        state[(i << 2) + 2] ^= u;
        state[(i << 2) + 3] ^= v;

        i += 1;
    }
    mix_columns(state);

    return;
}

pub const fn key_expansion(key: &mut [u8], round_key: &mut [u8]) -> () {

    const R_CON: [u8;10] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
    let mut operand: [u8;4] = [0;4];
    
    let mut pidx: usize;
    let mut nidx: usize;

    unsafe {
        memcpy(round_key.as_mut_ptr(), key.as_ptr(), AES_BLOCK);
    }

    // first round is unchanged

    let mut i: usize = 4;

    while i < 44 {
        pidx = (i - 1) << 2;
        nidx = i << 2;

        operand[0] = round_key[pidx + 0];
        operand[1] = round_key[pidx + 1];
        operand[2] = round_key[pidx + 2];
        operand[3] = round_key[pidx + 3];
    
        if (i & 0x3) == 0 {     // multiple of 4
            let temp: u8    = operand[0];
                operand[0]  = operand[1];
                operand[1]  = operand[2];
                operand[2]  = operand[3];
                operand[3]  = temp;
                operand[0]  = SBOX[operand[0] as usize];
                operand[1]  = SBOX[operand[1] as usize];
                operand[2]  = SBOX[operand[2] as usize];
                operand[3]  = SBOX[operand[3] as usize];
                operand[0]  ^= R_CON[(i >> 2)-1];
        }
        round_key[nidx    ] = round_key[pidx - 12] ^ operand[0];
        round_key[nidx + 1] = round_key[pidx - 11] ^ operand[1];
        round_key[nidx + 2] = round_key[pidx - 10] ^ operand[2];
        round_key[nidx + 3] = round_key[pidx -  9] ^ operand[3];

        i += 1;

    }
    return;
}

const fn aes_encrypt(data: &mut [u8], keys: &mut KeyWrapper) -> () {

    {
        let mut i: usize = 0;

        while i < AES_BLOCK {
            data[i] ^= keys.key[i];
            i += 1;
        }
    }

    {
        let mut i: usize = 0;

        while i < 10 {

            sub_bytes(data, &SBOX);
            shift_rows(data);
            mix_columns(data);
            add_round_key(data, &mut keys.round_key, i << 4);

            i += 1;
        }
    }

    sub_bytes(data, &SBOX);
    shift_rows(data);
    add_round_key(data, &mut keys.round_key, AES_BLOCK * 10);

    return;
}

const fn aes_decrypt(data: &mut [u8], keys: &mut KeyWrapper) -> () {
    add_round_key(data, &mut keys.round_key, AES_BLOCK * 10);
    inv_shift_rows(data);
    sub_bytes(data, &INV_SBOX);

    let mut i: usize = 9;
    
    while i > 0 {
    
        add_round_key(data, &mut keys.round_key, i << 4);
        inv_mix_columns(data);
        inv_shift_rows(data);
        sub_bytes(data, &INV_SBOX);
    
        i -= 1;
    
    }
    add_round_key(data, &mut keys.key, 0);

    return;
}

pub const fn aes_ctr_xcryption(
    data: &mut [u8],
    length: usize,
    keys: &mut KeyWrapper,
    mut offset: usize
) -> () {

    let mut nonce: [u8;AES_BLOCK] = [0; AES_BLOCK];
    let mut i: usize = 0;

    unsafe {
        memcpy( nonce.as_mut_ptr(), 
                keys.nonce.as_ptr(), 
                AES_BLOCK
        );
    }
    aes_encrypt(&mut nonce, keys);

    /*
        below is just handling the XOR and the counter operation
        on the nonce you can change this how you wish
    */

    while i < length {
        data[i] ^= nonce[offset % 16];
        offset += 1;
        i += 1;
    }

    return;
}

pub const fn extract_offset(buffer: &[u8]) -> usize {
    let n = buffer.len();
    assert!(n >= 4);

    return u32::from_le_bytes([
        buffer[n - 4],
        buffer[n - 3],
        buffer[n - 2],
        buffer[n - 1],
    ]) as usize;
}

pub const fn code_cryption(
    buffer: &mut [u8],
    mut keys: &mut KeyWrapper
) ->() {
    let boundry: usize = extract_offset(buffer);
    key_expansion(&mut keys.key, &mut keys.round_key);
    aes_ctr_xcryption(buffer, boundry, &mut keys, 0);
    return;
}
