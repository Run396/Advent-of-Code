f = open("day1.txt", "r")

list1 = []
list2 = []

for x in f:
    x = x.rstrip()
    numbers = x.split('   ')

    list1.append(numbers[0])
    list2.append(numbers[1])

list1.sort()
list2.sort()

num = 0

for x in range(len(list1)):
    num += abs(int(list1[x]) - int(list2[x]))
d1 = {}
d2 = {}

for x in range(len(list1)):
    if list1[x] in d1.keys():
        freq = d1.get(list1[x])
        freq += 1
        d1.update({list1[x]:freq})
    else:
        d1[list1[x]] = 1
    if list2[x] in d2.keys():
        freq = d2.get(list2[x])
        freq += 1
        d2.update({list2[x]:freq})
    else:
        d2[list2[x]] = 1

acc = 0

for key, freq in d1.items():
    if key in d2.keys():
        acc += int(key) * freq * d2[key]

print(acc)