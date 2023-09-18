alfabet = "abcdefghijklmnopqrstuvwxyzæøå"
where = {}

for i,letter in enumerate(alfabet):
    where[letter] = i


def encrypt(t,n):
    encrypted = ""
    t = t.lower()
    for i in range(len(t)):
        if t[i] not in where:
            encrypted += t[i]
            continue

        newKey = (where[t[i]] + n) % len(alfabet)
        encrypted += alfabet[newKey]
    return encrypted

t = input('What to encrypt: ')
n = int(input('Key: '))

print(encrypt(t,n))
