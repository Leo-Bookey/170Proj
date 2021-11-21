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


s1 = """1 237 23 61
2 757 51 29
3 132 56 79
4 185 41 69
5 330 27 23
6 622 25 65
7 1212 12 55
8 305 14 38
9 1403 21 40
10 41 40 77
11 534 54 33
12 405 46 81
13 642 45 37
14 1091 53 9
15 50 35 47
16 1299 36 82
17 1372 19 36
18 793 42 43
19 1360 21 57
20 955 37 57
21 1332 7 7
22 1090 23 95
23 594 49 75
24 1059 27 97
25 1330 58 5
26 1012 59 93
27 571 56 72
28 1188 12 26
29 284 13 87
30 1122 45 72
31 537 21 62
32 878 9 55
33 523 1 1
34 780 6 14
35 76 8 45
36 82 25 69
37 1108 18 65
38 1254 33 70
39 640 20 58
40 1190 37 64
41 480 57 47
42 1209 27 23
43 914 24 17
44 1257 2 20
45 315 52 91
46 500 7 73
47 100 50 50
48 47 55 4
49 580 48 60
50 227 8 76
51 286 28 19
52 206 48 51
53 204 16 62
54 668 47 33
55 874 31 10
56 1439 44 11
57 400 6 65
58 353 45 94
59 511 32 30
60 67 2 55
61 401 23 97
62 1294 19 27
63 862 20 44
64 791 29 17
65 1353 39 95
66 973 56 62
67 860 30 58
68 1349 57 1
69 659 15 95
70 770 48 99
71 1185 17 87
72 1326 29 70
73 1006 47 67
74 1035 4 5
75 530 47 76
76 813 8 9
77 596 38 90
78 836 43 66
79 828 40 11
80 234 49 94
81 547 44 45
82 1044 56 84
83 1012 27 40
84 84 37 24
85 209 46 31
86 973 58 57
87 1437 12 35
88 234 21 55
89 42 33 83
90 220 32 51
91 5 43 85
92 973 31 14
93 508 52 70
94 574 47 64
95 318 57 12
96 1009 19 18
97 1240 2 25
98 288 6 7
99 628 14 98
100 98 19 48
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



