from ta import indicator

print(indicator.__all__)

ultosc = indicator.ULTOSC(7, 14, 28)
for i in range(100):
    print(i, ultosc.update(i + 10, i + 7, i + 8, i + 3))
