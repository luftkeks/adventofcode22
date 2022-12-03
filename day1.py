file = open("input.txt", "r")
lines = file.readlines()
number = 0
numbers = []
for line in lines:
    if line != "\n":
        number += int(line)
    else:
        numbers.append(number)
        number = 0

print(max(numbers))
numbers = sorted(numbers, reverse=True)
print(numbers)
print(numbers[0]+numbers[1]+numbers[2])
