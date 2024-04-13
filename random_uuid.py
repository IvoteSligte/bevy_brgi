import random
import sys


def random_uuid():
    return random.randrange(2**64)


if __name__ == "__main__":
    num = 1
    if len(sys.argv) > 1:
        num = int(sys.argv[1])

    for i in range(num):
        print(random_uuid())
