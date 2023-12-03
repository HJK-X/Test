# with open("2.txt", 'r') as f:
#     sm = 0
#     for a in f.readlines():
#         a = a.strip()
#         t = int(a[5: a.index(": ")])
#         a = a[a.index(": ")+2:]
#         sl = a.split("; ")
#         fd = True
#         for b in sl:
#             sdf = b.split(", ")
#             # print(sdf)
#             flag = True
#             for i in sdf:
#                 if i[-3:] == "red":
#                     if int(i[:i.index(" ")]) > 12:
#                         flag = False
#                         break
#                 elif i[-4:] == "blue":
#                     if int(i[:i.index(" ")]) > 14:
#                         flag = False
#                         break
#                 elif i[-5:] == "green":
#                     if int(i[:i.index(" ")]) > 13:
#                         flag = False
#                         break

#             if not flag:
#                 fd = False
#                 break

#         if fd:
#             sm += t
#         # print(sm)

# print(sm)

with open("2.txt", 'r') as f:
    sm = 0
    for a in f.readlines():
        a = a.strip()
        t = int(a[5: a.index(": ")])
        a = a[a.index(": ")+2:]
        sl = a.split("; ")
        mr = 0
        mb = 0
        mg = 0
        for b in sl:
            sdf = b.split(", ")
            # print(sdf)
            for i in sdf:
                if i[-3:] == "red":
                    mr = max(mr, int(i[:i.index(" ")]))
                elif i[-4:] == "blue":
                    mb = max(mb, int(i[:i.index(" ")]))
                elif i[-5:] == "green":
                    mg = max(mg, int(i[:i.index(" ")]))

        sm += mr*mb*mg
        # print(sm)

print(sm)

