# symbiotic-llm-os

Human and LLM share equal scheduling rights; the kernel arbitrates CPU time between human tasks and model thoughts.

## Attributes
- **Kernel type**: symbiotic
- **LLM subsystem**: fair_arbiter
- **Languages**: rust, c, haskell

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
