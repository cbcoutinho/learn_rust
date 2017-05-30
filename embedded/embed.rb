require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib './target/debug/libembed.so'
  attach_function :process, [], :void
end

Hello.process

puts 'done!'
