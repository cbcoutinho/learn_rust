import sys, os
from ctypes import cdll

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = cdll.LoadLibrary(os.path.join('target', 'debug', prefix + "embed" + extension))

print('Loaded lib {0}'.format(lib))

lib.process()
