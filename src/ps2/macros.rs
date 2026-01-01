macro_rules! exports {
    (
        struct $dll:ident;

        $(
            #[func @ $func_addr:literal]
            fn $func:ident(
                $($arg:ident : $arg_ty:ty),*$(,)?
            ) $(-> $ret:ty)? ;
        )*

        $(
            #[var @ $var_addr:literal]
            static $var:ident : $var_ty:ty;
        )*

        $(
            #[array @ $arr_addr:literal]
            static $arr:ident : [$arr_ty:ty ; $len:literal];
        )*

        $(
            #[callback @ $cb_addr:literal]
            fn $cb:ident(
                $($cb_arg:ident : $cb_arg_ty:ty),*$(,)?
            ) $(-> $cb_ret:ty)? ;
        )*
    ) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $dll {
            $( pub $func: extern "win64" fn($($arg_ty),*) $(-> $ret)?, )*
            $( pub $var: *mut $var_ty, )*
            $( pub $arr: *mut [$arr_ty; $len], )*
            $( pub $cb: *mut extern "win64" fn($($cb_arg_ty),*) $(-> $cb_ret)?, )*
        }

        impl Nil for $dll {
            const NIL: Self = Self {
                $( $func: Nil::NIL, )*
                $( $var: Nil::NIL, )*
                $( $arr: Nil::NIL, )*
                $( $cb: Nil::NIL, )*
            };
        }

        impl $dll {
            unsafe fn from_module(module: HMODULE) -> Self {
                unsafe {
                    let base = module.0;
                    Self {
                        $($func : std::mem::transmute(base.offset($func_addr)),)*
                        $($var : base.offset($var_addr).cast::<$var_ty>(),)*
                        $($arr : base.offset($arr_addr).cast::<[$arr_ty ; $len]>(),)*
                        $($cb : base.offset($cb_addr).cast(),)*
                    }
                }
            }
        }

        $(
            #[inline]
            pub fn $func( $($arg : $arg_ty),* ) $(-> $ret)? {
                unsafe { (DLL.$func)($($arg),*) }
            }
        )*

        $(
            #[inline]
            pub fn $var() -> $var_ty {
                unsafe { *DLL.$var }
            }
        )*

        $(
            #[inline]
            pub fn $arr(i: impl TryInto<usize>) -> *mut $arr_ty {
                // TODO: decide how this should handle conversion failure
                // most/all of these arrays have indices that fit in 0..=127,
                // so anything negative or large is almost certainly a bogus index

                let i = i.try_into().unwrap_or(0xDEAD_DEAD_DEAD_DEAD);
                unsafe { &raw mut (*DLL.$arr)[i] }
            }
        )*

        pub mod cb {
            use super::*;

            $(
                #[inline]
                pub fn $cb( $($cb_arg : $cb_arg_ty),* ) $(-> $cb_ret)? {
                    unsafe {
                        let f = DLL.$cb;
                        // TODO: check for null pointer, like the original code does?
                        (*f)($($cb_arg),*)
                    }
                }
            )*
        }
    }
}
