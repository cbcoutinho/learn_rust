import sys, os
import cffi

ffi = cffi.FFI()

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ffi.dlopen(os.path.join('target', 'debug', prefix + "embed" + extension))

ffi.cdef('void process();')

print('Loaded lib {0}'.format(lib))

lib.process()
