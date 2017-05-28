from cffi import FFI
ffi = FFI()
ffi.cdef("""
    int double(int);
""")

C = ffi.dlopen("./ffi/target/debug/libraytracer_ffi.so")

print(C.double(9))
