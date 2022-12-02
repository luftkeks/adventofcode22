def calcWin(opponent: str, player: str):
    if opponent == "A":
        if player == "X":
            return 3
        if player == "Y":
            return 6
        if player == "Z":
            return 0
    if opponent == "B":
        if player == "X":
            return 0
        if player == "Y":
            return 3
        if player == "Z":
            return 6
    if opponent == "C":
        if player == "X":
            return 6
        if player == "Y":
            return 0
        if player == "Z":
            return 3

def calcScore(opponent: str, player: str):
    if player == "X":
        return 1 + calcWin(opponent, player)
    if player == "Y":
        return 2 + calcWin(opponent, player)
    if player == "Z":
        return 3 + calcWin(opponent, player)
    print("error")

def calcPart2(opponent: str, player: str):
    if opponent == "A":
        if player == "X":
            return 0 + 3
        if player == "Y":
            return 3 + 1
        if player == "Z":
            return 6 + 2
    if opponent == "B":
        if player == "X":
            return 0 + 1
        if player == "Y":
            return 3 + 2
        if player == "Z":
            return 6 + 3
    if opponent == "C":
        if player == "X":
            return 0 + 2
        if player == "Y":
            return 3 + 3
        if player == "Z":
            return 6 + 1


file = open("input2.txt","r")
lines = file.readlines()

score = 0
score2 = 0
for line in lines:
    opponent,player = line[0], line[2]
    score += calcScore(opponent, player)
    score2 += calcPart2(opponent, player)

print("part1:" + str(score))
print("part2:" + str(score2))