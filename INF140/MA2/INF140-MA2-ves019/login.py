"""Script for login in on user presumably saved in the shadow.txt file.
The script will input username and password of the wished login and search the shadow file for corresponding results.
If the username and hashed password match entries in the shadow file the script outputs "You're successfully logged in!" 
If there are no matches the script will output "Obs! The provided username and password do no match" """

# Hashlib is used to hash password with sha512
import hashlib
# Using getpass to make password input hidden
from getpass import getpass


# Using function from registration to make sample entry
def makeEntry(username, password:str) -> str:
    """Function making string consisting of username:hashedPassword. Hashing scheme is sha512"""
    # The bytes function makes the string into bytes so that hashlib can hash it
    password = bytes(password, "utf-8")

    # Choose hashing scheme
    crypt = hashlib.sha512()
    # Input what is to be hashed
    crypt.update(password)
    # Get hashed password
    password = crypt.hexdigest()

    # Make line to be appended into shadow.txt
    entry = f"{username}:{password}\n"

    return entry


# Get username
username = input("Please provide your username:\n")
# The getpass function hides user input in terminal
password = getpass("Please choose a password:\n")

# Make entry
entry = makeEntry(username,password)
# Set default output to be not logged in
output = "Obs! The provided username and password do no match"

# Opening shadow file with read permissions only (r parameter)
with open("shadow.txt","r") as shadowRead:
    # Looping over lines in shadowfile
    for line in shadowRead:
        # Test if the provided username and password match the line in the shadow file
        if line == entry:
            output = "You’re successfully logged in!"
            break


print(output)
