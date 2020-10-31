import sys
from math import sqrt, floor, gcd

def g(x, n):
    return (x**2-1) % n

def factor(n):
    x = 2
    y = 2
    d = 1

    while d == 1:
        x = g(x, n)
        y = g(g(y, n), n)
        d = gcd(abs(x - y), n)


    if d == n:
        raise "FAIL!"
    else:
        return (d, key / d)

for line in sys.stdin:
    ns = line.split(' ')

    key = int(ns[0])
    limit = int(ns[1])

    if key == 0 and limit == 0:
        break

    (p, q) = factor(key)

    p = min(p, q)

    if p < limit:
        print("BAD {}".format(p))
    else:
        print("GOOD")

