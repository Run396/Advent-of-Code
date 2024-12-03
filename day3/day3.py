import re

f = open("input.txt", "r")

text = []
num = []
acc = 0

for x in f:
    text += re.findall(r"mul\(\d+,\d+\)",x)

for x in text:
    num = re.findall(r"\d+",x)
    acc += int(num[0])*int(num[1])

print(acc)
