# TGSemu
Reference emulator for the fictional TGS console.
Tested against rust nightly at time of commits.

See the [DCC Summer 15 description](https://github.com/darkscience/dcc/tree/Sum15) for more information.

## Compilation
Simply run:

    cargo build --release

After this you can run `target/release/tgsemu <rom.bin>`.

## Known issues
Due to the crude UI (terminal without proper libs handling it), button input is
far from perfect.
