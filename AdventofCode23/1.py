# def isdigit(a, i):
#     if a[i].isdigit():
#         return int(a[i])

# with open("1.txt", 'r') as f:
#     sm = 0
#     for a in f.readlines():
#         b = -1
#         l = -1
#         for i in range(len(a)):
#             d = isdigit(a, i)
#             if d != None:
#                 if b == -1:
#                     b = d
#                 l = d
        
#         sm += b*10+l

# print(sm)



def isdigit(a, i):
    l = ["one","two","three","four","five","six","seven","eight","nine"]
    if a[i].isdigit():
        return int(a[i])
    
    for j in range(len(l)):
        if i+len(l[j]) <= len(a) and a[i:i+len(l[j])] == l[j]:
            return j+1

with open("1.txt", 'r') as f:
    sm = 0
    for a in f.readlines():
        b = -1
        l = -1
        for i in range(len(a)):
            d = isdigit(a, i)
            if d != None:
                if b == -1:
                    b = d
                l = d
        
        sm += b*10+l

print(sm)

