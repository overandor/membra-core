# virtualized-llm-os

Type-1 hypervisor hosting para-virtualized LLM guests with direct GPU passthrough.

## Attributes
- **Kernel type**: virtualized
- **LLM subsystem**: gpu_passthrough
- **Languages**: rust, c, cpp

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
