from ta import cum

print(cum.__all__)

counter = cum.Counter()
sumer = cum.Sumer()
meaner = cum.Meaner()
maxer = cum.Maxer()
miner = cum.Miner()
# deltaer = cum.Deltaer()
# pctchanger = cum.Pctchanger()
# quantiler = cum.Quantiler(0.3)

cumers = [counter, sumer, meaner, maxer, miner]
# cumers = [sumer, meaner, maxer, miner, deltaer, pctchanger, quantiler]
# cumers = [deltaer, pctchanger]

for cumer in cumers:
    for i in range(10):
        # print(i, cumer.update(i), cumer.get(0), cum.get(2))
        print(i, cumer.update(i))
    print("-" * 20)
