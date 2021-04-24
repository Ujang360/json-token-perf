# Serde JSON Tokenization vs Deserialization

## Optimized Run with glibc

```bash
❯ RUSTFLAGS="-C link-args=-s -C target-cpu=native" cargo run --release
First ID in Tokenization: "75f24eaa-5bef-4ed2-a0aa-b4a091acfb08"
Tokenization Duration: 119904737 ns
Tokenization per Item: 1199 ns
First ID in Deserialization: 75f24eaa-5bef-4ed2-a0aa-b4a091acfb08
Deserialization Duration: 104159349 ns
Deserialization per Item: 1041 ns
```

## Optimized Run with glibc and mimalloc

```bash
❯ RUSTFLAGS="-C link-args=-s -C target-cpu=native" cargo run --release --features=altalloc
First ID in Tokenization: "9c279d61-e3b5-4129-b330-dcbcedca3de8"
Tokenization Duration: 121022611 ns
Tokenization per Item: 1210 ns
First ID in Deserialization: 9c279d61-e3b5-4129-b330-dcbcedca3de8
Deserialization Duration: 110495770 ns
Deserialization per Item: 1104 ns
```

## Optimized Run with libc-musl

```bash
❯ RUSTFLAGS="-C link-args=-s -C target-cpu=native" cargo run --release --target=x86_64-unknown-linux-musl
First ID in Tokenization: "9b372f86-41e8-4666-a803-ed972f68e8ec"
Tokenization Duration: 178724684 ns
Tokenization per Item: 1787 ns
First ID in Deserialization: 9b372f86-41e8-4666-a803-ed972f68e8ec
Deserialization Duration: 109466307 ns
Deserialization per Item: 1094 ns
```

## Optimized Run with libc-musl and mimalloc

```bash
❯ RUSTFLAGS="-C link-args=-s -C target-cpu=native" cargo run --release --target=x86_64-unknown-linux-musl --features=altalloc
First ID in Tokenization: "90831e12-dbbe-4e4e-b543-82f56ade1817"
Tokenization Duration: 175867665 ns
Tokenization per Item: 1758 ns
First ID in Deserialization: 90831e12-dbbe-4e4e-b543-82f56ade1817
Deserialization Duration: 115727900 ns
Deserialization per Item: 1157 ns
```
