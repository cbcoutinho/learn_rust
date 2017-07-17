# Python3 - ctypes version
# from ctypes import cdll
#
# lib = cdll.LoadLibrary("./target/debug/libembed.so")
#
# lib.process()
#
# print("done!")

# Python3 - cffi version
import cffi

ffi = cffi.FFI()
lib = ffi.dlopen("./target/debug/libembed.so")

ffi.cdef('void process();')

lib.process()
