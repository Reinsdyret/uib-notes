from pwn import *
import sys
import time

for i in range(0, 30):
    print(f"trying sending {i} bytes")
    io = process('./over')
    io.send(cyclic(i))
    
    io.wait_for_close()
    if io.poll() != 0:
        print(f"RIP (maybe) at offset {i}")
        sys.exit(0)

