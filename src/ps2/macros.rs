macro_rules! export_functions {
    (
        $var:ident : $struct:ident;
        $(
            #[addr = $addr:literal]
            fn $func:ident(
                $($arg:ident : $aty:ty),*$(,)?
            ) $(-> $ret:ty)? ;
        )*
    ) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $struct {
            $(
                pub $func: extern "win64" fn($($aty),*) $(-> $ret)?,
            )*
        }

        impl Nil for $struct {
            const NIL: Self = Self {
                $($func: Nil::NIL),*
            };
        }

        impl $struct {
            unsafe fn from_module(module: HMODULE) -> Self {
                unsafe {
                    let base = module.0;
                    Self {
                        $($func : std::mem::transmute(base.offset($addr)),)*
                    }
                }
            }
        }

        $(
            pub fn $func( $($arg : $aty),* ) $(-> $ret)? {
                unsafe { (DLL.functions.$func)($($arg),*) }
            }
        )*
    }
}

macro_rules! export_variables {
    (
        $group:ident : $struct:ident;
        $(
            #[addr = $addr:literal]
            static $var:ident : $ty:ty;
        )*
    ) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $struct { $( pub $var: *mut $ty),* }

        impl Nil for $struct {
            const NIL: Self = Self { $($var: Nil::NIL),* };
        }

        impl $struct {
            unsafe fn from_module(module: HMODULE) -> Self {
                unsafe {
                    let base = module.0;
                    Self {
                        $($var : base.offset($addr).cast::<$ty>(),)*
                    }
                }
            }
        }

        $(
            pub fn $var() -> $ty {
                unsafe { *DLL.variables.$var }
            }
        )*
    }
}
