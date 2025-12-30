macro_rules! assert_offset {
    ($T:ty, $field:ident, $expected:literal) => {
        const _: () = {
            if ::core::mem::offset_of!($T, $field) != $expected {
                panic!("offset mismatch");
            }
        };
    };
}

macro_rules! assert_size {
    ($T:ty, $expected:literal) => {
        const _: () = {
            _ = ::core::mem::transmute::<$T, [u8; $expected]>;
        };
    };
}
