from cffi import FFI
ffi = FFI()

ffi.cdef("""
    typedef struct {
        double x, y, z;
    } vector_t;
    double length(const vector_t *vec);
""")

C = ffi.dlopen("./ffi/target/debug/libraytracer_ffi.so")

vector = ffi.new("vector_t *")
vector.x = 1.0
vector.y = 1.0
vector.z = 1.0

print(C.length(vector))
