# reactive-llm-os

Reactive streams at kernel level; LLM outputs propagate as observable hot streams.

## Attributes
- **Kernel type**: reactive
- **LLM subsystem**: observable_output
- **Languages**: rust, cpp

## Architecture

```
bootloader/   → minimal stub or GRUB multiboot
kernel/       → kernel main, scheduler, memory manager
llm_subsystem/→ token inference, KV-cache, model loader
drivers/      → UART, framebuffer, NVMe, GPU
```

## Build

```bash
make
```

## Run in QEMU

```bash
make run ARCH=x86_64
```

## License
MIT
