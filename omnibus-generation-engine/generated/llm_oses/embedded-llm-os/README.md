# embedded-llm-os

Sub-MB kernel for MCUs; runs TinyML models with on-device fine-tuning via federated delta.

## Attributes
- **Kernel type**: embedded
- **LLM subsystem**: tinyml_federator
- **Languages**: c, rust, zephyr

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
