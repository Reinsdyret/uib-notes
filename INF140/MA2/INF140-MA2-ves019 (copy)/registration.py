"""Program for user registration. User will type in username and password and the username and hashed password, using sha512, will be added as a new line in the shadow.txt file. 
Colon seperating the username and password"""

# Hashlib is used to hash password with sha512
import hashlib
# Using getpass to make password input hidden
from getpass import getpass


def makeEntry(username, password:str) -> str:
    """Function making entry for shadow.txt file."""
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

line = makeEntry(username,password)
# Opening shadow file using a context manager, parameter a to append to the file and not overwrite
# NOTE that the file directory must sometimes be changen for it to work. If shadow.txt does not exist it will create it.
with open("shadow.txt","a") as shadowWrite:
    # Writing line to shadow file
    shadowWrite.write(line)
    pass

# Printing congratulations message
print("Congratulations! Your registration is completed!")
