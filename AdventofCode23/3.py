def floodfill(maze):
    mp = [[0 for j in range(len(maze[0]))] for i in range(len(maze))]
    for i in range(len(maze)):
        for j in range(len(maze[0])):
            if not (mp[i][j].isdigit() or mp[i][j] == '.'):
                x = [1,1,1,0,0,-1,-1,-1]
                y = [1,-1,0,1,-1,-1,0,1]
                for d in range(4):
                    if 0<=i+x[d]<=len(maze) and 0<=j+y[d]<=len(maze[0]):
                        mp[i+x[d]][j+y[d]] = 1

with open("3.txt", 'r') as f:
    lns = f.readlines()

    for a in lns:
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

