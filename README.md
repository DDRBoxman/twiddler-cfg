# Twiddler-cfg

Tool for converting and massaging twiddler configs!

Convert a v5 to v6 file
```
./twiddler_cfg ./configs/backspice2_v5.cfg ./backspicev2_v6.cfg
```

Help
```
./twiddler_cfg --help
```

### Roadmap
- [x] Read v5 configs
- [x] Read v6 configs
- [x] Write v6 configs
- [x] Chord mappings
- [x] v5 global config (mouse accel, mouse clicks, etc.)
- [x] v6 global config (mouse accel, mouse clicks, etc.)
- [ ] Ensure output has default system chords
- [ ] Read CSV
- [ ] Write CSV
- [x] Read dido text format
- [ ] Write dido format
- [ ] Ability to autogenerate shift chords for uppercase letters


### Development stuff
Run directly from cargo
```
cargo run -- ./configs/backspice2_v5.cfg ./test.cfg
```

Run tests logging output
```
cargo test -- --nocapture
```
