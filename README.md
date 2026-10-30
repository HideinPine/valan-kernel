# Valan

Valan is my own x86_64 kernel, written in Rust. It's early, but it already
boots, puts text on the screen and reacts to interrupts, which feels like a
real start.

It gets loaded by **Fern**, a UEFI bootloader I wrote for it, so the two
projects live side by side in this repo.

## Where things stand

What works right now:

- Booting through UEFI with Fern: it loads the kernel ELF, collects the boot
  info, leaves boot services and jumps in
- A text console drawn on the GOP framebuffer with the Spleen font, with
  `print!` and `println!` on top
- A GDT and TSS, with a separate stack for double faults
- An IDT with breakpoint and double fault handlers
- The PIC remapped, with the timer interrupt ticking

What I'm in the middle of:

- Keyboard input (the interrupt fires, the key handling is still rough)
- A test setup that runs inside QEMU and reports back

What isn't there yet: scrolling. Once the text reaches the bottom of the
screen it just stops showing. That's next to fix.

## How it boots

1. OVMF (UEFI firmware in QEMU) starts `BOOTX64.EFI`, which is Fern.
2. Fern reads the kernel ELF from the boot partition and copies its loadable
   segments to the right physical addresses.
3. While UEFI is still around, Fern gathers what the kernel will need: the
   framebuffer (address, size, resolution, stride, pixel format), the ACPI
   RSDP address and where the kernel sits in memory.
4. Fern exits boot services, turns the final UEFI memory map into a plain
   array of regions and patches it into the boot info.
5. Fern jumps to the kernel and hands over a pointer to that boot info:

   ```rust
   pub extern "sysv64" fn _start(bootinfo: *const BootInfo) -> !
   ```

From there the kernel sets up the console, loads the GDT and IDT, and turns
interrupts on.

## What's in the repo

```
Valan/
  boot_info/        small no_std crate shared by Fern and the kernel
  fern_bootloader/  the UEFI bootloader
  kernel/           the keErnel itself
    src/
      main.rs          entry point, panic handler, test hook
      frame_buffer.rs  framebuffer writer, font drawing, print macros
      descriptors/     gdt.rs and idt.rs
```

`boot_info` holds the structs both sides need (`BootInfo`, the memory map,
the framebuffer description). It has no dependencies on purpose. If the kernel
pulled in the `uefi` crate just to read a struct, the two panic handlers would
fight, and I'd rather keep the kernel free of firmware code. The memory map
struct is called `Memento`, after the film.

## Building and running

You'll need:

- Rust nightly (the kernel uses `abi_x86_interrupt` and a custom test
  framework)
- QEMU and OVMF firmware files
- PowerShell, since my run scripts are `.ps1`

Then, from the `kernel` folder:

```powershell
cd kernel
cargo run
```

That builds the kernel, copies it into Fern's `esp/` folder, builds Fern and
starts QEMU. The QEMU and OVMF paths in `fern_bootloader/run.ps1` match my
Windows install, so change them if yours live somewhere else.

For tests, `run.ps1 -Test` in `fern_bootloader` adds QEMU's `isa-debug-exit`
device and writes serial output to a log file. The runner is hooked up in
`.cargo/config.toml`.

## What I want to do next

- Scrolling, and a proper cursor
- Real keyboard input
- Page fault handling and paging
- A physical frame allocator that uses the memory map from Fern, then a heap
- A simple filesystem, starting with a RAM disk
- Userspace, system calls and a scheduler

Further out: an ARM64 port, and one day a phone.

## Related

I'm building a language called **Dron** at the same time, with its own
compiler (`dronc`, also in Rust). Valan is meant to be its home eventually.

## Thanks

- The [Spleen](https://github.com/fcambus/spleen) font by Frederic Cambus
  (BSD 2-Clause), which is what you see on screen.
- [Writing an OS in Rust](https://os.phil-opp.com) and the
  [OSDev wiki](https://wiki.osdev.org), which I keep open in a tab.
