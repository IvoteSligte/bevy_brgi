import random
import sys

num = 1
if len(sys.argv) > 1:
    num = int(sys.argv[1])

for i in range(num):
    print(random.randrange(2**64))

