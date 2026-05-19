# self-hosting-llm-os

OS can rewrite its own kernel source via LLM and hot-reload without reboot.

## Attributes
- **Kernel type**: self-hosting
- **LLM subsystem**: kernel_codegen
- **Languages**: rust, c, lisp

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
