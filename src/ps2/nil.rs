extern "win64" fn nil2i<T, U>(x: T, _: U) -> T {
    x
}

pub trait Nil {
    const NIL: Self;
}

macro_rules! impl_nil {
    ($($T:ident)*) => {
        impl_nil!("C" $($T)*);
        impl_nil!([unsafe] "C" $($T)*);
        impl_nil!("win64" $($T)*);
        impl_nil!([unsafe] "win64" $($T)*);
    };

    ($([$unsafe:tt])? $abi:literal $($T:ident)*) => {
        impl<$($T),*> Nil for $($unsafe)? extern $abi fn($($T),*) {
            const NIL: Self = {
                $($unsafe)? extern $abi fn nil <$($T),*> ($(_: $T),*) {}
                nil
            };
        }
    };
}

impl_nil!();
impl_nil!(A);
impl_nil!(A B);
impl_nil!(A B C);
impl_nil!(A B C D);
impl_nil!(A B C D E);
impl_nil!(A B C D E F);
impl_nil!(A B C D E F G);
impl_nil!(A B C D E F G H);

// the pointer is needed for coherence with the void version
impl<T, U> Nil for extern "win64" fn(*mut T, U) -> *mut T {
    const NIL: Self = nil2i;
}

impl<T> Nil for *mut T {
    const NIL: Self = std::ptr::null_mut();
}
