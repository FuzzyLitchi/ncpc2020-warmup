import sys

data_one = []
for line in sys.stdin:
    data_one.append(line.split()[0])

def getIndex(x, y, size):
    if(x >= size or y >= size or x < 0 or y < 0):
        return 'B'
    return (list(data[y])[x])


def getNeighbors(x, y, size):
    neigh = []
    if ((x,y,x+1,y) not in visited):
        neigh.append(getIndex(x+1,y,size))
    if ((x,y,x-1,y) not in visited):
        neigh.append(getIndex(x-1,y,size))
    if ((x,y,x,y+1) not in visited):
        neigh.append(getIndex(x,y+1,size))
    if ((x,y,x,y-1) not in visited):
        neigh.append(getIndex(x,y-1,size))
    return neigh

visited = []
size = int(data_one[0])
data = data_one[1:]
count = 0
for x in range(0,size):
    for y in range(0, size):
        if (getIndex(x,y,size) == 'B'):
            continue
        neigh = getNeighbors(x,y,size)
        count = count + neigh.count('C')
        visited.append((x+1,y,x,y))
        visited.append((x-1,y,x,y))
        visited.append((x,y+1,x,y))
        visited.append((x,y-1,x,y))

print(count%10007)