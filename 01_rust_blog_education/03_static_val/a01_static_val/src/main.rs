static BYTES: [u8; 3] = [1, 2, 3];
static mut MUT_BYTES: [u8; 3] = [1, 2, 3];

fn main() {
    // MUT_BYTES[0] = 99;
    unsafe {
        MUT_BYTES[0] = 99;
        assert_eq!(99, MUT_BYTES[0]);
    }

    unsafe {
        dbg!(MUT_BYTES);
    }
}
