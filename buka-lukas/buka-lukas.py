import sys

data = []
for line in sys.stdin:
    data.append(line.split()[0])

a = int(data[0])
operator = str(data[1])
b = int(data[2])
if (operator == "*"):
    print(str(a*b))
if (operator == "+"):
    print(str(a+b))