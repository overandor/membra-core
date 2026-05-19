# flat-address-llm-os

Single global 64-bit address space; LLM KV-cache is memory-mapped across the entire cluster.

## Attributes
- **Kernel type**: flat-address
- **LLM subsystem**: global_kv_cache
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
