```bash
#installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


# run project
cargo run

# run test function
cargo test #all

#atau 
cargo test name_function -- --exact  #by function

#atau 
cargo test name_function -- --exact --nocapture #by function

#release project
cargo build --release
```
