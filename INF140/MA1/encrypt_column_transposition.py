"""
Encrypting a message M with key K using transposition (column) encryption
Program authour: Lars Møen Haukland
"""

M = input("Message: ")
K = input("Key: ")

alfabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']

# Lowering all characters to make it easier
M = M.lower()
K = K.lower()

columns = [[None]] * len(K)
print (columns)

for i in range(0,len(M)):
    if M[i] not in alfabet:
        continue
    print(((i+1) // len(K)))
    columns[((i+1) // len(K))].append(M[i])

print(columns)