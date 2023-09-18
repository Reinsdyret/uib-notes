"""
Program for hashing message M using toy tetragraph hash method or TTH for short.
Program authour: Lars Møen Haukland
"""

import collections

alfabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']

M = input("Message: ")
M = M.lower()
blocks = len(M) // 16
runningTotal = [0,0,0,0]

# Remove all characters that is not in alfabet
i = 0
G = M
while i < len(M):
    if M[i] not in alfabet:
        G = G.replace(M[i],"")
    i += 1
M = G

def block_hash(block,runnTotal):
    colOne = 0
    colTwo = 1
    colThree = 2
    colFour = 3
    while colFour <= len(block) - 1:
        runnTotal[0] += alfabet.index(block[colOne])
        runnTotal[1] += alfabet.index(block[colTwo])
        runnTotal[2] += alfabet.index(block[colThree])
        runnTotal[3] += alfabet.index(block[colFour])
        colOne += 4
        colTwo += 4
        colThree += 4
        colFour += 4
    
    for i in range(0,len(runnTotal)):
        runnTotal[i] %= 26
    return runnTotal


def rotate_block(block):
    row1 = block[0:4]
    row2 = block[4:8]
    row3 = block[8:12]
    row4 = block[12:16]
    rows = [row1,row2,row3,row4]
    for i in range(len(rows)-1):
        a_list = collections.deque(rows[i])
        a_list.rotate(-1*(i+1))
        rows[i] = list(a_list)
    rows[3] = list(rows[3][::-1])
    return [rows[0] + rows[1] + rows[2] + rows[3]]

def two_dimensional_list_to_string(arr):
    """Take in a two dimensional list and turn into a string"""
    output = ""
    for row in range(len(arr)):
        for col in range(len(arr[row])):
            output += arr[row][col]
    return output


for i in range(1,blocks+1):
    block = M[((i-1)*16):(i*16)]
    runningTotal = block_hash(block,runningTotal)
    block = two_dimensional_list_to_string(rotate_block(block))
    runningTotal = block_hash(block,runningTotal)

# Print Hash from runningTotal
hashVariable = ""
for i in range(len(runningTotal)):
    hashVariable += alfabet[runningTotal[i]]
hashVariable = hashVariable.upper()
print(hashVariable)
