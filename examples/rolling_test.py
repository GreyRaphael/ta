from ta import rolling

sumer = rolling.Sumer(3)
meaner = rolling.Meaner(3)
maxer = rolling.Maxer(3)
miner = rolling.Miner(3)

rollingers = [sumer, meaner, maxer, miner]

for rollinger in rollingers:
    for i in range(10):
        print(i, rollinger.update(i))
    print("-" * 20)
