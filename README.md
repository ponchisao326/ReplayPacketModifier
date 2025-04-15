# ReplayPacketModifier

**ReplayPacketModifier** is a command-line application written in Rust that modifies Minecraft replay files (in `.mcpr` format) by filtering out specified packets from the internal `recording.tmcpr` file. The application reads a list of packet IDs provided by the user (e.g., `0x65,0x03,0x00,0x40`) and produces a new replay file with those packets removed.

---

## Usage

You can run the pre-built executable directly from the Releases. For example, from a command prompt:

```bash
ReplayPacketModifier.exe --input "replay.mcpr" --output "modified_replay.mcpr" --codes "0x65,0x03,0x00,0x40"
```

For further details on available options, use the `--help` flag:

```bash
ReplayPacketModifier.exe --help
```

This will display additional information on how to use the application.

---

## Build Instructions

To build **ReplayPacketModifier** from source, make sure you have [Rust](https://rust-lang.org) installed and then run the following command in the project directory:

```bash
cargo build --release
```

After building, you will find the executable in the `target/release` directory.

---

## Project Structure

```
src/
├── main.rs       // Entry point; handles CLI and orchestration.
├── args.rs       // Defines the command-line arguments using clap.
├── utils.rs      // Utility functions, such as parsing packet codes and reading/writing integers.
└── process.rs    // Contains the logic to process the .mcpr file and filter the packets from recording.tmcpr.
```

---

## Dependencies

- [clap](https://crates.io/crates/clap) (v4.5.36) for command-line parsing.
- [zip](https://crates.io/crates/zip) (v2.6.1) for reading and writing `.mcpr` files (ZIP format).

---

## License

This project is released under the terms of the MIT License.

---

## Additional Information

- **Filtering Packets**:  
  The application will remove packets from the replay file if their packet ID (as a VarInt) matches any of the IDs provided via the `--codes` parameter. For example, if you run with `--codes "0x65,0x03,0x00,0x40"`, packets with IDs `0x65` (101), `0x03` (3), `0x00` (0), and `0x40` (64) will be filtered out.

- **Replay File Format**:  
  The application assumes that inside the `.mcpr` file, the `recording.tmcpr` entry follows this binary format:

  ```c
  uint32_t timestamp;
  uint32_t packet_size;
  uint8_t packet_data[];
  ```

  This is taken into account during the reading and writing process.