from math import *
unordered_map = {}

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
        if currActualJob[3] >= 0:
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


s1 = """1 1 1 138
2 1 1 583
3 1 1 868
4 1 1 822
5 1 1 783
6 1 1 65
7 1 1 262
8 1 1 121
9 1 1 508
10 1 1 780
11 1 1 461
12 1 1 484
13 1 1 668
14 1 1 389
15 1 1 808
16 1 1 215
17 1 1 97
18 1 1 500
19 1 1 30
20 1 1 915
"""
vals = s1.split("\n")
for i in range(len(vals)):
    row = vals[i].split()
    for j in range(len(row)):
        row[j] = float(row[j])
    #print(row)
    if (row != []):
        unordered_map[i + 1] = row

#print(unordered_map)

print(find_max_1(unordered_map,0))
