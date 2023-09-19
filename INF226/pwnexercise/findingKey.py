from pwn import *

context.log_level = 'debug'

e = ELF('/home/yral/UiB/INF226/oblig1/pwnexercise')

print(e.functions.main)

print(e.symbols)
for (k,v) in e.symbols.items():
    print(f'{hex(v)}  {k}')


