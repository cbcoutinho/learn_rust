import sys

from cffi import FFI
ffi = FFI()

ffi.cdef("""
    typedef struct {
        double x, y, z;
    } vector_t;
    double length(const vector_t *vec);
""")

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')

# This path is relative to from where the file is being executed.
# C = ffi.dlopen("./target/debug/libraytracer_ffi.so")
C = ffi.dlopen(prefix + "raytracer_ffi" + extension)

vector = ffi.new("vector_t *")
vector.x = 1.0
vector.y = 1.0
vector.z = 1.0

print(C.length(vector))
