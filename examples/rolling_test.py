from ta import rolling

sumer = rolling.Sumer(3)
meaner = rolling.Meaner(3)

rollingers = [sumer, meaner]

for rollinger in rollingers:
    for i in range(10):
        print(i, rollinger.update(i))
    print("-" * 20)
