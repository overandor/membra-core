# mobile-llm-os

Battery-aware scheduler quantizing models to NPU and falling back to CPU on thermal throttling.

## Attributes
- **Kernel type**: mobile
- **LLM subsystem**: thermal_quantizer
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
