file = open("input.txt","r")
lines = file.readlines()

def findMarker(line: str, singular:int) -> int:
    for ii in range(singular, len(line)):
        charList = []
        for jj in range(singular):
            charList.append(line[ii-jj])
        charSet = set(charList)
        if len(charSet) == singular:
            return ii+1

for line in lines: # this is because i have multiple test cases
    print("Part 1:" + str(findMarker(line, 4)))
    print("Part 2:" + str(findMarker(line, 14)))
