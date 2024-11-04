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