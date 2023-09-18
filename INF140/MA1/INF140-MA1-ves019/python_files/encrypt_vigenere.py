"""
Program for encrypting using vigenere encryption with running key
Program authour: Lars Møen Haukland
"""

M = input("Message: ")
K = input("Key: ")

alfabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']

# Lowering all characters to make it easier
M = M.lower()
K = K.lower()

decryptOrEncrypt = int(input("0 for encrypt and 1 for decrypt: "))

def vigenere(M,k):
    keyCount = 0
    output = ""
    for letter in M:
        # In case of spaces
        if letter not in alfabet:
            #output += letter
            continue
        # Using algebraic expression for vigenere (Ci = Ek(Mi) = (Mi + Ki) mod 26)
        # Found on wikipedia page for vigenere encryption, sourced as source 1 in sources list
        output += alfabet[(alfabet.index(letter)  + alfabet.index(k[keyCount]))  % 26]
        keyCount += 1
        # Repeating key
        if keyCount >= len(k):
            keyCount = 0
    return output


def decrypt_vigenere(M,k):
    keyCount = 0
    output = ""
    for letter in M:
        # In case of spaces
        if letter not in alfabet:
            #output += letter
            continue
        # Using algebraic expression for vigenere (Ci = Ek(Mi) = (Mi + Ki) mod 26)
        # Found on wikipedia page for vigenere encryption, sourced as source 1 in sources list
        output += alfabet[(alfabet.index(letter)  - alfabet.index(k[keyCount]))  % 26]
        keyCount += 1
        # Repeating key
        if keyCount >= len(k):
            keyCount = 0
    return output

if decryptOrEncrypt:
    print(decrypt_vigenere(M,K))
else:
    print(vigenere(M,K))
