# from ctypes import cdll
#
# lib = cdll.LoadLibrary("target/debug/libembed.so")
#
# lib.process()
#
# print("done!")

import cffi

ffi = cffi.FFI()
lib = ffi.dlopen("./target/debug/libembed.so")

# print(lib)
print('Loaded lib {0}'.format(lib))
# <cffi.api.FFILibrary_./libtreble.dylib object at 0x107f440d0>

# ffi.cdef('int treble(int);')

# print("math from rust!", lib.treble(10))
# print()
lib.process()
