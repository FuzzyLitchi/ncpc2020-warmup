import sys

digits10limit = 100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

def test(t, a, b):
    a = (pow(t, a) - 1)
    b = (pow(t, b) - 1)

    if a % b != 0:
        return None
    n = a//b
    if n < digits10limit:
        return n
    else:
        return None

for line in sys.stdin:
    ns = line.split(' ')

    t = int(ns[0])
    a = int(ns[1])
    b = int(ns[2])

    print("({}^{}-1)/({}^{}-1)".format(t, a, t, b), end=" ")
    res = test(t, a, b)
    if res == None:
        print("is not an integer with less than 100 digits.")
    else:
        print("{}".format(res))