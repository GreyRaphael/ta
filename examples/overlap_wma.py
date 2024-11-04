from ta import indicator

wmaer = indicator.WMA(5)
price_list = [172.72, 176.08, 178.67, 171.37, 172.38]
for p in price_list:
    print(p, wmaer.update(p))
weights = [1, 2, 3, 4, 5]

sum1 = 0
for w, x in zip(weights, price_list):
    print(w, x, w * x)
    sum1 += w * x
print(f"result={sum1/15.0}")
