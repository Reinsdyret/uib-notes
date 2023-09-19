from pwn import *

context.log_level = 'debug'

io = remote('inf226.puffling.no', 6001)
print(io.recvline())
io.sendline('infA')

io.recvline()
math = io.recvuntil(' =')
math = math[0:-2]
print(math)
answer = safeeval.expr(math)
io.sendline(str(answer))
for i in range(1023):
    math = io.recvuntil(' =')
    math = math[2:-2]
    answer = safeeval.expr(math)
    io.sendline(str(answer))

io.recvline()
