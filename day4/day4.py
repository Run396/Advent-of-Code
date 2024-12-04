f = open("input.txt", "r")

def check(x,m,a,s):
    if (x == 'X') and (m == 'M') and (a == 'A') and (s == 'S'):
        return 1
    else:
        return 0

matrix = []
acc = 0
no = 0

for line in f:
    line = line.rstrip()
    matrix.append(line)

for i in range(len(line)): #Collumn
    for j in range(len(line)): #Row
        try:
            acc += check(matrix[i][j],matrix[i][j+1],matrix[i][j+2],matrix[i][j+3]) # Horizontal = 3
            acc += check(matrix[i][j+3],matrix[i][j+2],matrix[i][j+1],matrix[i][j]) # Reverse = 2
        except:
            no += 1
        try:
            acc += check(matrix[i][j],matrix[i+1][j],matrix[i+2][j],matrix[i+3][j]) # Vertical = 1
            acc += check(matrix[i+3][j],matrix[i+2][j],matrix[i+1][j],matrix[i][j]) # Reverse = 2
        except:
            no += 1
        try:
            acc += check(matrix[i][j],matrix[i+1][j+1],matrix[i+2][j+2],matrix[i+3][j+3]) # Diagonal (l->r) = 1
            acc += check(matrix[i+3][j+3],matrix[i+2][j+2],matrix[i+1][j+1],matrix[i][j]) # Reverse = 4
        except:
            no += 1
        try:
            acc += check(matrix[i][j+3],matrix[i+1][j+2],matrix[i+2][j+1],matrix[i+3][j]) # Diagonal (r->l) = 1
            acc += check(matrix[i+3][j],matrix[i+2][j+1],matrix[i+1][j+2],matrix[i][j+3]) # Reverse = 4
        except:
            no += 1

print('x-mas: ',acc)


cross = 0
def check2(tl,tr,bl,br):
    if ((tl == "M" and br == "S") or (tl == "S" and br == "M")) and ((tr == "M" and bl == "S") or (tr == "S" and bl == "M")):
        return 1
    else:
        return 0

for i in range(len(line)-1):
    for j in range(len(line)-1):
        if matrix[i+1][j+1] == 'A':
            try:
                cross += check2(matrix[i][j],matrix[i][j+2],matrix[i+2][j],matrix[i+2][j+2])
            except:
                no += 1

print('cross: ', cross)