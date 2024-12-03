f = open("day2.txt", "r")

numbers = []

for x in f:
    x = x.rstrip()
    numbers.append(x.split(' '))

pos_control = [1,2,3]
neg_control = [-1,-2,-3]

def check(rep):
    flag = False
    all_same = 0
    for i in range(len(rep)-1):

        diff = int(rep[i]) - int(rep[i+1])
        if diff in pos_control and all_same == 0:
            flag = True
        
        if diff in pos_control and flag == True:
            all_same += 1
        if diff in neg_control and flag == False:
            all_same += 1


    if all_same == len(rep) -1:
        return 1
    else:
        return 0

safe = 0
for report in numbers:
    safe += check(report)
    
print('First: ',safe)


def check2(rep):
    pos = 0
    pos2 = 0
    neg = 0
    neg2 = 0
    oob = 0
    loop = len(rep)
    for i in range(loop-1):

        diff = int(rep[i]) - int(rep[i+1])
        if i != loop - 2:
            diff2 = int(rep[i]) - int(rep[i+2])
        
        if diff in pos_control:
            pos += 1
        if diff2 in pos_control:
            pos2 += 1

        if diff in neg_control:
            neg += 1
        if diff2 in neg_control:
            neg2 += 1

        if diff not in pos_control and diff not in neg_control:
            oob +=1
        diff2 = 0

    if pos == loop - 1:
        return 1
    elif neg == loop - 1:
        return 1

    elif pos == loop - 2 and oob < 2 and pos2 > 0:
        return 1
    elif neg == loop - 2 and oob < 2 and neg2 > 0:
        return 1
    else:
        return 0

safe = 0
for report in numbers:
    safe += check2(report)
    
print('Damp: ', safe)