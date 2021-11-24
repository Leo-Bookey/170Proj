f= open("a.txt", "r+")
import math
allVals = []
for x in f:
    a = x.split()
    allVals.append(int(a[3]))


allVals.sort(reverse = True)
#print(allVals)

total = 0
for i in range(len(allVals)):
    total += allVals[i] * math.exp(-0.017 * i)

print(total)
f.close()
