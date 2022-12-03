import re


def duplicate_letter(string1: str, string2: str):
    list = []
    for letter1 in string1:
        for letter2 in string2:
            if letter1 == letter2:
                list.append(letter1)
    return set(list)


def findDuplicateItem(rucksack: str):
    compartmentsize: int = int(len(rucksack)/2)
    left = rucksack[0:compartmentsize]
    right = rucksack[compartmentsize:compartmentsize*2]
    return next(iter(duplicate_letter(left, right)))


def char_value(letter: str):
    if letter.isupper():
        return ord(letter) - 65 + 26
    else:
        return ord(letter) - 97


file = open("input3.txt", "r")
lines = file.readlines()
for jj in range(len(lines)):
    lines[jj] = re.sub('\n', '', lines[jj])


priority = 0
for line in lines:
    char = findDuplicateItem(line)
    value = char_value(char) + 1  # easy fix - dont know where broken
    print(char + "   " + str(value))
    priority += value

print(priority)

badge_prio = 0
for ii in range(len(lines)//3):  # python 3 muss man zwingen eine int division zu machen
    for letters in duplicate_letter(lines[ii*3], lines[ii*3 + 1]):
        for letter in letters:
            for char in lines[ii*3+2]:
                if letter == char:
                    print(letter)
                    # easy fix - dont know where broken - second time
                    badge_prio += char_value(str(letter)) + 1
                    break

print(badge_prio)
