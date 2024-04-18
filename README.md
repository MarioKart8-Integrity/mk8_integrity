# Mario Kart 8 Integrity tools

An application which verifies if the game files loaded by CEMU are legit.
The tool will also check if any external program is trying to modify CEMU behavior (slowing down, impossible input macros, ...).
The tool will be available for any OS distribution (Windows, Linux, MacOS).
Language used: `rust`.

## Usage

```bash
cargo run 'path/to/game'
```

## To do

### Basic features

- Making sure the path to the game files are the ones used by CEMU.
  - Preventing the user to specify the wrong path and cheating the system.
- Verify the checksum of some game files (keeping the file names private for now).
- Being able to access CEMU live information and check if the game runs in 60fps.

### Advanced features

- Scanning for any memory modifications.
  - Preventing external softwares to change the game stats.
- Checking for `ptrace` permissions.
- Checking for impossible input combos.
- Replicating some of MOSS features, see more --> [MOSS](https://nohope.eu/).
  - Random screenshots of the monitors.
  - PID and names of opened/closed programs while the tool is running.
  - ...and more.
