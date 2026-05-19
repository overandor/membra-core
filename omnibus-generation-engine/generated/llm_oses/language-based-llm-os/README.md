# language-based-llm-os

OS implemented in a linearly-typed language where memory safety proves tensor shape correctness.

## Attributes
- **Kernel type**: language-based
- **LLM subsystem**: shape_prover
- **Languages**: rust, idris, ats

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
