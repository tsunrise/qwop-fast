# `qwop-fast`: A fast implementation of QWOP simulator

Same API as `https://web.stanford.edu/class/cs168/qwop.py`, but much faster.

## Install

```
pip install qwop-fast
```

## Usage

- You can just follow the API of `https://web.stanford.edu/class/cs168/qwop.py` and just replace `import qwop` with `import qwop_fast`.

    ```python
    import numpy as np
    import qwop_fast
    plan = np.random.uniform(-1, 1, 40)
    qwop_fast.sim(plan) # return a float
    ```

- You can also simulate in batches. Batches will be executed in parallel.

    ```python
    import numpy as np
    import qwop_fast
    plan = np.random.uniform(-1, 1, (100, 40))
    qwop_fast.sim_batch(plan) # return a list of floats
    ```

## Build From Source

1. Install Rust: https://rustup.rs/

2. Install Maturin: 
    ```
    pip install maturin
    ```

3. Build and install the package:
    ```
    maturin develop --release
    ```

