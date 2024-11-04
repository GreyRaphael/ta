# ta

Technical analysis library by pyo3

## How to develop

```bash
# install maturin or download from github
cargo install --locked maturin
# add ~/.cargo/bin to PATH
maturin --help

# initialize project
mkdir ta && cd ta
maturin init
# choose pyo3

# change Cargo.toml features to 
# features = ["abi3-py38"]

# begin development
source ~/envs/jupy12/bin/activate
maturin develop

# begin release *whl
maturin build --release

# begin publish to pypi
maturin publish
```

## Todo

Overlap
- [] BBANDS - Bollinger Bands
- [X] DEMA - Double Exponential Moving Average
- [X] EMA - Exponential Moving Average
- [] HT_TRENDLINE - Hilbert Transform - Instantaneous Trendline
- [X] KAMA - Kaufman Adaptive Moving Average
- [x] MA - Moving average, **NO NEED**
- [] MAMA - MESA Adaptive Moving Average
- [x] MAVP - Moving average with variable period **NO NEED**
- [x] MIDPOINT - MidPoint over period
- [x] MIDPRICE - Midpoint Price over period
- [] SAR - Parabolic SAR
- [X] SMA - Simple Moving Average
- [X] T3 - Triple Exponential Moving Average (T3)
- [X] TEMA - Triple Exponential Moving Average
- [x] TRIMA - Triangular Moving Average
- [X] WMA - Weighted Moving Average