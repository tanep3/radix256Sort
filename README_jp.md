# radix256Sort

[English](README.md) | [技術解説](TECHNICAL_DETAILS_jp.md)

Rustで書かれた、`u32` 整数向けの超高速かつ安定な Radix Sort（基数ソート）実装です。Pythonバインディングも提供しています。
CPUキャッシュ効率の最適化と、内部ループでのアロケーションをゼロにする戦略により、高いパフォーマンスを実現しています。

## 特徴

- **高速**: 大規模データセットにおいて、標準ライブラリのソート (`std::slice::sort`) や Pythonの `list.sort` / `numpy.sort` を凌駕する性能（Rustネイティブ比較時）。
- **安定**: 同じ値の要素の相対的な順序を保持します。
- **安全**: `unsafe` ブロックを使用しない純粋な Rust 実装です。
- **シンプル**: 32ビット整数に最適化された、固定256基数・4パスのアルゴリズムです。

## 導入手順 (Getting Started)

### Crates.io からのインストール

`Cargo.toml` に以下を追加してください:

```toml
[dependencies]
radix256_sort = "0.1.0"
```

または `cargo add` を使用:

```bash
cargo add radix256_sort
```

### ソースからビルド

#### 前提条件
- Rust (最新の安定版)
- Python 3.7以上 (Pythonバインディング用)

#### 1. リポジトリのクローン
```bash
git clone https://github.com/tanep3/radix256Sort.git
cd radix256Sort
```

### 2. Rust: テストとベンチマークの実行
ユニットテストの実行:
```bash
cargo test --workspace
```

マイクロベンチマーク (Criterion) の実行:
```bash
cargo bench -p radix256_sort
```
*結果は `target/criterion/report/index.html` にグラフィカルなレポートとして生成されます。*

マクロベンチマーク (1億件) の実行:
```bash
cargo run --release -p rust_bench
```

### 3. Python: ビルドとベンチマークの実行
仮想環境の使用を推奨します。

```bash
# 仮想環境の作成と有効化
python3 -m venv .venv
source .venv/bin/activate

# ビルドツールのインストール
pip install maturin numpy

# ライブラリのビルドとインストール
cd radix256_sort_py
maturin develop --release
cd ..

# ベンチマークの実行
python benchmarks/python_bench/bench.py
```

## 使い方

### Rust

```rust
use radix256_sort::radix256_sort_vec;

let mut data = vec![5, 2, 9, 1, 5];
let sorted = radix256_sort_vec(data);
assert_eq!(sorted, vec![1, 2, 5, 5, 9]);
```

インプレース（破壊的）ソート:

```rust
use radix256_sort::radix256_sort_inplace;

let mut data = vec![5, 2, 9, 1, 5];
radix256_sort_inplace(&mut data);
assert_eq!(data, vec![1, 2, 5, 5, 9]);
```

### Python

```python
import radix256_sort_py

data = [5, 2, 9, 1, 5]
sorted_data = radix256_sort_py.radix256_sort(data)
print(sorted_data) # [1, 2, 5, 5, 9]
```

## ベンチマーク

1億個 (100M) のランダムな `u32` 整数に対する計測結果です。

> [!NOTE]
> 以下の数値は開発環境での参考値です。環境により性能は変動します。


### 凡例 (Legend)
- **`radix256_sort_vec`**: 本ライブラリ (バッファ版) - **最速**
- **`radix256_sort_inplace`**: 本ライブラリ (インプレース版)
- **`std_sort`**: Rust標準 (安定ソート) - 比較対象
- **`std_sort_unstable`**: Rust標準 (不安定ソート) - 参考比較

### Rust

| アルゴリズム | 時間 (秒) | 高速化率 |
| :--- | :--- | :--- |
| `std::slice::sort` | 2.99s | 1.0x |
| **`radix256_sort_vec`** | **0.84s** | **3.56x** |

### Python

| アルゴリズム | 時間 (秒) | 高速化率 (vs list) |
| :--- | :--- | :--- |
| `list.sort()` | 76.89s | 1.0x |
| **`radix256_sort`** | **7.61s** | **10.1x** |
| `numpy.sort()` | 5.27s | 14.6x |

### 考察

ベンチマーク結果は、`radix256_sort` がRustおよびPythonの両環境において、標準実装を大きく上回る性能を持つことを示しています。

- **Rust**: 高度に最適化された標準の `std::slice::sort` (pdqsort) に対して **3.5倍** の高速化を達成しました。これは、汎用的な比較ソートに対し、キャッシュ効率の良い固定パス方式が有効であることを証明しています。
- **Python**: `list.sort` に対して **10倍** の高速化を実現しました。純粋なPython環境での数値計算において強力な選択肢となります。`numpy.sort` (5.27s) の方が高速ですが、これは NumPy への依存が必要です。`radix256_sort` は標準リストに対して NumPy に迫る性能 (7.61s) を提供しており、その差の多くは Pythonリストから Rustベクタへの変換コスト (O(N)) によるものです。

なぜこれほど高速なのか、詳細な技術解説については [技術解説](TECHNICAL_DETAILS_jp.md) をご覧ください。

## ライセンス

Apache License 2.0

Copyright (c) 2025 Tane Channel Technology

