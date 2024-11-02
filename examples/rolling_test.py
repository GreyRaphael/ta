from ta import rolling

sumer = rolling.Sumer(3)
for i in range(10):
    print(i, sumer.update(i))
