pub fn u64AppendTou8arr(v: u64, bit: u32, arr: &mut Vec<u8>) {
    let mut value = v;
    for i in 0..bit {
        arr.push((value % 256) as u8);
        value /= 256;
    }
}

macro_rules! number_append_to_u8arr {
    ($v:ident, $arr:ident, $bit:expr) => ({
        let mut buf = [0; $bit];
        let bufLen = u64Tou8arr($v, &mut buf);
        u64AppendTou8arr(bufLen as u64, 1, $arr);
        $arr.extend_from_slice(&buf[..(bufLen as usize)]);
    })
}

//! first: calc u8 number length
//! second: append u8 number to arr
pub fn u8NumberAppendTou8arr(v: u64, arr: &mut Vec<u8>) {
    number_append_to_u8arr!(v, arr, 1);
}

pub fn u16NumberAppendTou8arr(v: u64, arr: &mut Vec<u8>) {
    number_append_to_u8arr!(v, arr, 2);
}

pub fn u32NumberAppendTou8arr(v: u64, arr: &mut Vec<u8>) {
    number_append_to_u8arr!(v, arr, 4);
}

pub fn u64NumberAppendTou8arr(v: u64, arr: &mut Vec<u8>) {
    number_append_to_u8arr!(v, arr, 8);
}

pub fn u64Tou8arr(v: u64, arr: &mut [u8]) -> u32 {
    let mut value = v;
    let mut i = 0;
    while value > 0 {
        arr[i] = (value % 256) as u8;
        value /= 256;
        i += 1;
    }
    i as u32
}

pub fn u8arrTou64(arr: &[u8], v: &mut u64) {
    *v = 0;
    for (index, item) in arr.iter().enumerate() {
        *v += *item as u64 * 256_u64.pow(index as u32) as u64;
    }
}

pub fn u64Tou8arrByBase(v: u64, arr: &mut [u8], byte: u32) {
    let mut value = v;
    let mut i = 0;
    let b = 2_u64.pow(byte);
    while value > 0 {
        arr[i] = (value % b) as u8;
        value /= b;
        i += 1;
    }
}

pub fn u8arrTou64ByBase(arr: &[u8], v: &mut u64, byte: u32) {
    *v = 0;
    let b = 2_u64.pow(byte);
    for (index, item) in arr.iter().enumerate() {
        *v += *item as u64 * b.pow(index as u32) as u64;
    }
}

#[test]
#[ignore]
fn u64Tou8arrTest() {
    let mut arr = [0; 8];
    u64Tou8arr(1000, &mut arr);
    assert_eq!(arr[0], 232);
    assert_eq!(arr[1], 3);
}

#[test]
#[ignore]
fn u8arrTou64Test() {
    let arr = [232, 3, 0, 0, 0, 0, 0, 0];
    let mut value = 0;
    u8arrTou64(&arr, &mut value);
    assert_eq!(1000, value);
}

#[test]
#[ignore]
fn u64Tou8arrByBaseTest() {
    let mut arr = [0; 8];
    u64Tou8arrByBase(1000, &mut arr, 8);
    assert_eq!(arr[0], 232);
    assert_eq!(arr[1], 3);
}

#[test]
#[ignore]
fn u8arrTou64ByBaseTest() {
    let arr = [232, 3, 0, 0, 0, 0, 0, 0];
    let mut value = 0;
    u8arrTou64ByBase(&arr, &mut value, 8);
    assert_eq!(1000, value);
}

#[test]
fn u32NumberAppendTou8arrTest() {
    let mut arr = Vec::new();
    u32NumberAppendTou8arr(0, &mut arr);
    assert_eq!(arr, vec![0]);
}
