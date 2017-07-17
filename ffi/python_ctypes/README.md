# Simple python-rust ffi example from the ffi omnibus

Python calls rust to calculate the sum of two numbers

After building the rust package using `cargo build`, execute the following line using python3

'''
LD_LIBRARY_PATH=target/debug/ python src/main.py
'''
