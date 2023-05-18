# `qwop-fast`: A fast implementation of QWOP simulator

Same API as `https://web.stanford.edu/class/cs168/qwop.py`, but much faster.

## Install

1. Install Rust: https://rustup.rs/

2. Install Maturin: 
    ```
    pip install maturin
    ```

3. Build and install the package:
    ```
    maturin develop --release
    ```

4. Use it:
    ```python
    import numpy as np
    import qwop_fast
    plan = np.random.uniform(-1, 1, 40)
    qwop_fast.simulate(plan)
    ```
