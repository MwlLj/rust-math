pub fn u64Tou8arr(v: u64, arr: &mut [u8]) {
    let mut value = v;
    let mut i = 0;
    while value > 0 {
        arr[i] = (value % 256) as u8;
        value /= 256;
        i += 1;
    }
}

pub fn u8arrTou64(arr: &[u8], v: &mut u64) {
    for (index, item) in arr.iter().enumerate() {
        *v += *item as u64 * 256_u64.pow(index as u32) as u64;
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
