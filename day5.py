blub = []
#blub.append(["Z", "N"])
#blub.append(["M", "C", "D"])
#blub.append(["P"])
blub.append(["S","T","H","F","W","R"])
blub.append(["S","G","D","Q","W"])
blub.append(["B","T","W"])
blub.append(["D","R","W","T","N","Q","Z","J"])
blub.append(["F","B","H","G","L","V","T","Z"])
blub.append(["L","P","T","C","V","B","S","G"])
blub.append(["Z","B","R","T","W","G","P"])
blub.append(["N","G","M","T","C","J","R"])
blub.append(["L","G","B","W"])

for element in blub:
    print(element)

file = open("input.txt","r")
lines = file.readlines()

for line in lines:
    elements = line.split(" ")
    for ii in reversed(range(int(elements[1]))): # to get part one you need to switch this an the following line a bit - here the reverse needs to be removed
        blub[int(elements[5])-1].append(blub[int(elements[3])-1].pop(-ii-1)) # here the things in pop needs to be removed
    print("=============================================================")
    for element in blub:
        print(element)
