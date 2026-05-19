# deterministic-llm-os

Fully reproducible kernel state; same prompt always produces identical token sequence and side effects.

## Attributes
- **Kernel type**: deterministic
- **LLM subsystem**: reproducible_engine
- **Languages**: rust, c

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
