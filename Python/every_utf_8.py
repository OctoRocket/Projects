x = 1
for i in range(150000):
    try:
        print(chr(x))
    except:
        pass
    x = x + 1

