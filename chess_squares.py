for rank in range(1, 8 + 1):
    line = ""
    for file in range(ord('a'), ord('h') + 1):
        line = line + '"' + chr(file) + str(rank) + '", '
    print(line)
