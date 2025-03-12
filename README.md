# VDF playground

A repository implementing different VDF (Verifiable Delay Functions).

The current VDF implemented are:
- [MinRoot](https://eprint.iacr.org/2022/1626.pdf) in [src/lib.rs](./src/lib.rs).


# Execute

```bash
cargo run --release -- minroot-fifth --n 100 -x 4 -y 5
```

# TODO

- [ ] use the field as a parameter
