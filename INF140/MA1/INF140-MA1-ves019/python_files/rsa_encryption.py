"""
Program used in mandatory assigmenet in INF140 from university of Bergen for doing RSA encryption given the values p,q,e and M
Program authour: Lars Møen Haukland
"""

def calculate_n(p,q):
    return p*q

def calculate_phi_of_n(p,q):
    return (p-1) * (q-1)

def calculate_d(e,phi_n):
    # Modulo inverse solution found on https://stackoverflow.com/questions/4798654/modular-multiplicative-inverse-function-in-python 
    return pow(e,-1,phi_n)

def calculate_c(M,e,n):
    return (M**e) % n

def calculate_m(c,d,n):
    return (c**d) % n

M = int(input("M: "))
p = int(input("p: "))
q = int(input("q: "))
e = int(input("e: "))

n = calculate_n(p,q)
phi_n = calculate_phi_of_n(p,q)
d = calculate_d(e,phi_n)
C = calculate_c(M,e,n)
decrypted_M = calculate_m(C,d,n)

print(f"d is {d}")
print(f"Encrypted M is C: {C}")
print(f"Decrypted C is M: {decrypted_M}")