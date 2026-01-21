extern "win64" fn nil0() {}
extern "win64" fn nil<T>(_x: T) {}
extern "win64" fn nil2<T, U>(_x: T, _y: U) {}
extern "win64" fn nil3<T, U, V>(_x: T, _y: U, _z: V) {}
extern "win64" fn nil4<T, U, V, W>(_x: T, _y: U, _z: V, _w: W) {}

extern "win64" fn nil2i<T, U>(x: T, _y: U) -> T {
    x
}

pub trait Nil {
    const NIL: Self;
}

impl Nil for extern "win64" fn() {
    const NIL: Self = nil0;
}

impl<T> Nil for extern "win64" fn(T) {
    const NIL: Self = nil;
}

impl<T, U> Nil for extern "win64" fn(T, U) -> () {
    const NIL: Self = nil2;
}

impl<T, U, V> Nil for extern "win64" fn(T, U, V) {
    const NIL: Self = nil3;
}

impl<T, U, V, W> Nil for extern "win64" fn(T, U, V, W) {
    const NIL: Self = nil4;
}

// the pointer is needed for coherence with the void version
impl<T, U> Nil for extern "win64" fn(*mut T, U) -> *mut T {
    const NIL: Self = nil2i;
}

impl<T> Nil for *mut T {
    const NIL: Self = std::ptr::null_mut();
}
