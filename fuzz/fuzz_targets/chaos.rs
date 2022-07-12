#![no_main]
use bumpalo::Bump;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, arbitrary::Arbitrary)]
enum Op {
    Alloc,
    AllocStr,
    AllocSlice(u8),
    Reset,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut bump = Bump::new();

    for op in ops {
        match op {
            Op::Alloc => {
                bump.alloc(1u8);
            }
            Op::AllocStr => {
                bump.alloc_str("aaa");
            }
            Op::AllocSlice(l) => {
                bump.alloc_slice_fill_copy(l as usize, l);
            }
            Op::Reset => {
                bump.reset();
                assert_eq!(0, bump.allocated_bytes());
            }
        }
    }
});
