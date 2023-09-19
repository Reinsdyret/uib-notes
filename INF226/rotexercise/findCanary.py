from pwn import *

for i in range(1,30):
    print(f"trying sending {i} bytes")
    io = process('./hackme_medium')
    io.send(cyclic(i))

    io.shutdown('out')
    
    io.wait_for_close()
    if io.poll() != 0:
        print(f"canary (maybe) at offset {i}")
        sys.exit(0)
