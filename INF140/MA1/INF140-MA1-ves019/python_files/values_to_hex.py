"""
Given a set of values this program returns the hexadecimal value of each, values shall be space seperated
Program authour: Lars Møen Haukland
"""

values = input("Values: ").split(" ")

hex_values = []
for value in values:
    hex_values.append(hex(int(value)))
print(hex_values)
