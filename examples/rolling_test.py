from ta import rolling

# print(rolling.__dict__)
print(rolling.__all__)

sumer = rolling.Sumer(3)
meaner = rolling.Meaner(3)
maxer = rolling.Maxer(3)
miner = rolling.Miner(3)
deltaer = rolling.Deltaer(3)
pctchanger = rolling.Pctchanger(3)

rollingers = [sumer, meaner, maxer, miner, deltaer, pctchanger]

for rollinger in rollingers:
    for i in range(10):
        print(i, rollinger.update(i))
    print("-" * 20)
