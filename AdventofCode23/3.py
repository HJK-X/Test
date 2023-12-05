# with open("3.txt", 'r') as f:
#     lns = f.readlines()

#     for i in range(len(lns)):
#         lns[i] = lns[i].strip()

#     maze = [[0 for i in range(len(lns[0]))] for j in range(len(lns))]
#     def floodfill(x, y):
#         if lns[x][y].isdigit():
#             maze[x][y] = 1
#         else:
#             return
#         if y-1>=0 and lns[x][y-1].isdigit() and maze[x][y-1] != 1:
#             maze[x][y-1] = 1
#             floodfill(x, y-1)
#         if y+1<len(lns[0]) and lns[x][y+1].isdigit() and maze[x][y+1] != 1:
#             maze[x][y+1] = 1
#             floodfill(x, y+1)
        

#     sm = 0

#     for i in range(len(lns)):
#         for j in range(len(lns[0])):
#             if (not (lns[i][j].isdigit() or lns[i][j] == '.')):
#                 x = [1,1,1,0,0,-1,-1,-1]
#                 y = [1,-1,0,1,-1,0,-1,1]
#                 for d in range(8):
#                     if 0<=i+x[d]<=len(lns) and 0<=j+y[d]<=len(lns[0]):
#                         floodfill(i+x[d], j+y[d])
    
#     # print(maze)
#     for i in range(len(lns)):
#         for j in range(len(lns[0])):
#             if maze[i][j] == 1:
#                 s = lns[i][j]
#                 maze[i][j] = 0
#                 nx = j+1
#                 while nx<len(lns) and maze[i][nx] == 1:
#                     maze[i][nx] = 0
#                     s += lns[i][nx]
#                     nx += 1

#                 # print(s)

#                 sm += int(s)


# print(sm)



with open("3.txt", 'r') as f:
    lns = f.readlines()

    for i in range(len(lns)):
        lns[i] = lns[i].strip()

    maze = [[None for i in range(len(lns[0]))] for j in range(len(lns))]
    def floodfill(x, y, id):
        if lns[x][y].isdigit():
            maze[x][y] = id
        else:
            return
        if y-1>=0 and lns[x][y-1].isdigit() and maze[x][y-1] == None:
            maze[x][y-1] = id
            floodfill(x, y-1, id)
        if y+1<len(lns[0]) and lns[x][y+1].isdigit() and maze[x][y+1] == None:
            maze[x][y+1] = id
            floodfill(x, y+1, id)
        

    sm = 0

    cts = []
    id = -1
    for i in range(len(lns)):
        for j in range(len(lns[0])):
            if lns[i][j] == '*':
                id += 1
                x = [1,1,1,0,0,-1,-1,-1]
                y = [1,-1,0,1,-1,0,-1,1]
                ct = 0
                for d in range(8):
                    nx = i+x[d]
                    ny = j+y[d]
                    if 0<=nx<=len(lns) and 0<=ny<=len(lns[0]):
                        if maze[nx][ny] != id and lns[nx][ny].isdigit():
                            ct += 1
                        floodfill(nx, ny, id)

                cts.append(ct)
    
    # print(maze)
    # print(cts)
    ct = [0 for i in range(len(cts))]
    for i in range(len(lns)):
        for j in range(len(lns[0])):
            if maze[i][j] != None and cts[maze[i][j]] == 2:
                s = lns[i][j]
                nx = j+1
                while nx<len(lns) and maze[i][nx] != None:
                    maze[i][nx] = None
                    s += lns[i][nx]
                    nx += 1

                # print(s)
                if ct[maze[i][j]] == 0:
                    ct[maze[i][j]] = 1
                ct[maze[i][j]] *= int(s)
                # print(s, maze[i][j])

                maze[i][j] = 0

    # print(ct)

    for i in range(len(ct)):
        if ct[i] != 0:
            sm += ct[i]
print(sm)

