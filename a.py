f= open("a.txt", "r+")
v = ""

from random import seed
from random import randint
seed(1)
for x in f:
    a = x.split()
    #print(len(a))
    #
    v += "\nJob j%s = {%s, %s, %s, %s};" % (a[0], a[0], a[1], a[2], a[3])
    v = v + "\nigloo.insert(make_pair(%s, j%s));" % (a[0], a[0])
    #b = str(randint(1,60))
    #c = str(randint(1, 1440))
    #v += "\n%s %s %s %s" % (a[0], 1, 1, randint(1,1000))


f.write(v + "\n")

f.close()
