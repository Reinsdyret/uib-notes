from pwn import *

io = process('./hackme_medium')

io.send(cyclic(8))

io.shutdown('out')
print(io.readall())

