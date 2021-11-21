f= open("a.txt", "r+")
for x in f:
    a = x.split()
    print(len(a))
    v = "Job j%s = {%s, %s, %s, %s}" % (a[0], a[0], a[1], a[2], a[3]) 
    v = v + "\nigloo.insert(make_pair(%s, j%s));" % (a[0], a[0])
    f.write(v + "\n")
f.close()

