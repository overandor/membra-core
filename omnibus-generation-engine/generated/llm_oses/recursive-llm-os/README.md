# recursive-llm-os

OS runs a smaller OS inside itself to validate speculative LLM outputs before committing to host state.

## Attributes
- **Kernel type**: recursive
- **LLM subsystem**: speculative_vm
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
