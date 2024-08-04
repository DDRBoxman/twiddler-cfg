# Twiddler-cfg

Tool for converting and massaging twiddler configs!

Convert a v5 to v6 file and make sure that shift works
```
./twiddler_cfg --generate-caps 4 ./configs/backspice2_v5.cfg ./backspicev2_v6.cfg
```

Help
```
./twiddler_cfg --help

Convert Twiddler v5 configs to Twiddler v6 configs

Usage: twiddler_cfg [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>
  <OUTPUT>

Options:
  -g, --generate-caps <GENERATE_CAPS>  Generate upper case versions of chords with shift, 1 2 3 or 4 for the thumb key that should act as shift
  -h, --help                           Print help
  -V, --version                        Print version
```

### Roadmap
- [x] Read v5 configs
- [x] Read v6 configs
- [x] Write v6 configs
- [ ] Read v7 configs
- [ ] Write v7 configs
- [x] Chord mappings
- [x] v5 global config (mouse accel, mouse clicks, etc.)
- [x] v6 global config (mouse accel, mouse clicks, etc.)
- [ ] Ensure output has default system chords
- [ ] Read CSV
- [ ] Write CSV
- [x] Read dido text format
- [ ] Write dido format
- [x] Ability to autogenerate shift chords for uppercase letters


### Development stuff
Run directly from cargo
```
cargo run -- ./configs/backspice2_v5.cfg ./test.cfg
```

Coolhand
```
cargo run -- --generate-caps 4 ./configs/CoolHand.txt ./coolhand_v6_caps.cfg
```

Run tests logging output
```
cargo test -- --nocapture
```
