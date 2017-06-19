mod vector;

#[no_mangle]
pub extern "C" fn length(ptr: *const vector::Vector3) -> f64 {
    let vec = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    vec.length()
}
