
# Serial wire viewer

The idea here is pretty simple. Use the ARM debug components
found in many ARM embedded cores and forward the trace
data to the lognplot GUI.

# Usage

Example usage:

    $ cd swviewer
    $ cargo run --release -- some_elf_firmware.elf -F 48000000

This will load the firmware file, scan it for DWARF symbols and
next connect to both st-link and lognplot. The cpu frequency
must be given for proper results.
