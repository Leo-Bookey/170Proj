from math import *
unordered_map = {}
currIter = 0;
# def find_max(igloos,time):
#     if (time >= 1440 or 0 >= len(igloos)):
#         return 0
#     maxProf = 0
#     inclProf = 0
#     for _,v in igloos.items():
#         currActualJob = v
#         if currActualJob[3] >= 0:
#             _ = currActualJob[0]
#             if currActualJob[2] + time <= currActualJob[1]:
#                 inclProf = currActualJob[3]
#             else:
#                 late = currActualJob[2] + time - currActualJob[3]
#                 inclProf = currActualJob[3] * exp(-0.017 * late)
#             currActualJob[3] *= -1

#             for i i
#             inclProf += find_max(igloos, time + currActualJob[2])
#             currActualJob[3] *= -1
#             maxProf = max(maxProf,inclProf)
#     return maxProf

def find_max_1(igloos,time):
    if (time >= 1440 or 0 >= len(igloos)):
        return 0
    maxProf = 0
    inclProf = 0
    for _,v in igloos.items():
        currActualJob = v
        global currIter
        print(currIter)
        currIter += 1
        if currActualJob[3] >= 0 and currActualJob[2] + time <= 1440:
            _ = currActualJob[0]
            if currActualJob[2] + time <= currActualJob[1]:
                inclProf = currActualJob[3]
            else:
                late = currActualJob[2] + time - currActualJob[1]
                inclProf = currActualJob[3] * exp(-0.017 * late)
            currActualJob[3] *= -1
            inclProf += find_max_1(igloos, time + currActualJob[2])
            currActualJob[3] *= -1
            maxProf = max(maxProf,inclProf)
    return maxProf


s = """
1 187 30 99.5
2 30 12 7.751
3 15 55 12.987
4 200 15 55.2
5 1420 20 1.25
"""


s1 = """1 368 50 45
2 939 79 10
3 41 84 44
4 1098 1 45
5 1040 23 95
6 1299 4 3
7 780 94 39
8 312 43 3
9 320 44 50
10 336 24 22
"""
vals = s1.split("\n")
for i in range(len(vals)):
    row = vals[i].split()
    for j in range(len(row)):
        row[j] = float(row[j])
    #print(row)
    if (row != []):
        unordered_map[i] = row

#print(unordered_map)

print(find_max_1(unordered_map,0))
