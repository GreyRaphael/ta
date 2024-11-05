# indicators

Math Transform Functions
- [x] ACOS - Vector Trigonometric ACos, `real = ACOS(real)`
- [x] ASIN - Vector Trigonometric ASin, `real = ASIN(real)`
- [x] ATAN - Vector Trigonometric ATan, `real = ATAN(real)`
- [x] CEIL - Vector Ceil, `real = CEIL(real)`
- [x] COS - Vector Trigonometric Cos, `real = COS(real)`
- [x] COSH - Vector Trigonometric Cosh, `real = COSH(real)`
- [x] EXP - Vector Arithmetic Exp, `real = EXP(real)`
- [x] FLOOR - Vector Floor, `real = FLOOR(real)`
- [x] LN - Vector Log Natural, `real = LN(real)`
- [x] LOG10 - Vector Log10, `real = LOG10(real)`
- [x] SIN - Vector Trigonometric Sin, `real = SIN(real)`
- [x] SINH - Vector Trigonometric Sinh, `real = SINH(real)`
- [x] SQRT - Vector Square Root, `real = SQRT(real)`
- [x] TAN - Vector Trigonometric Tan, `real = TAN(real)`
- [x] TANH - Vector Trigonometric Tanh, `real = TANH(real)`

Math Operator Functions
- [x] ADD - Vector Arithmetic Add, `real = ADD(real0, real1)`
- [x] DIV - Vector Arithmetic Div, `real = DIV(real0, real1)`
- [x] MAX - Highest value over a specified period, `real = MAX(real, timeperiod=30)`
- [x] MAXINDEX - Index of highest value over a specified period, `integer = MAXINDEX(real, timeperiod=30)`
- [x] MIN - Lowest value over a specified period, `real = MIN(real, timeperiod=30)`
- [x] MININDEX - Index of lowest value over a specified period, `integer = MININDEX(real, timeperiod=30)`
- [x] MINMAX - Lowest and highest values over a specified period, `min, max = MINMAX(real, timeperiod=30)`
- [x] MINMAXINDEX - Indexes of lowest and highest values over a specified period, `minidx, maxidx = MINMAXINDEX(real, timeperiod=30)`
- [x] MULT - Vector Arithmetic Mult, `real = MULT(real0, real1)`
- [x] SUB - Vector Arithmetic Subtraction, `real = SUB(real0, real1)`
- [x] SUM - Summation, `real = SUM(real, timeperiod=30)`

Statistic Functions
- [] BETA - Beta, `real = BETA(real0, real1, timeperiod=5)`
- [] CORREL - Pearson's Correlation Coefficient (r), `real = CORREL(real0, real1, timeperiod=30)`
- [] LINEARREG - Linear Regression, `real = LINEARREG(real, timeperiod=14)`
- [] LINEARREG_ANGLE - Linear Regression Angle, `real = LINEARREG_ANGLE(real, timeperiod=14)`
- [] LINEARREG_INTERCEPT - Linear Regression Intercept, `real = LINEARREG_INTERCEPT(real, timeperiod=14)`
- [] LINEARREG_SLOPE - Linear Regression Slope, `real = LINEARREG_SLOPE(real, timeperiod=14)`
- [] STDDEV - Standard Deviation, `real = STDDEV(real, timeperiod=5, nbdev=1)`
- [] TSF - Time Series Forecast, `real = TSF(real, timeperiod=14)`
- [] VAR - Variance, `real = VAR(real, timeperiod=5, nbdev=1)`