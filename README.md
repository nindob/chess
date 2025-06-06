<div align="center">
<h1>chess</h1>
A chess TUI implementation in rust
</div>

![board](./assets/demo.gif)

[GitHub CI](https://github.com/nindob/chess/actions/workflows/build_and_test.yml/badge.svg)
![GHCR Push](https://github.com/nindob/chess/actions/workflows/docker_push.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

### Demo

**With docker**

```bash
docker run --rm -it ghcr.io/nindob/chess:main
```

**With Cargo**
```
git clone git@github.com:nindob/chess.git
cd chess
cargo build --release
./target/release/chess
```
