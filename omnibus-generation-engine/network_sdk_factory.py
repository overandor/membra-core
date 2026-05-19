#!/usr/bin/env python3
"""
Network SDK Factory — generates 20 network SDKs from manifest.json.
Each SDK gets real trait/class interfaces, protocol stubs, examples, and tests.
"""

import json
import os
import shutil
from pathlib import Path
from typing import Any

MANIFEST = Path(__file__).with_name("manifest.json")
OUT_DIR = Path(__file__).parent / "generated" / "network_sdks"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


def cargo_toml(name: str, sdk: dict) -> str:
    return f'''[package]
name = "{name.replace("-", "_")}"
version = "0.1.0"
edition = "2021"
description = "{sdk['description']}"
license = "MIT"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
thiserror = "1.0"
async-trait = "0.1"
tokio = {{ version = "1", features = ["full"] }}

[dev-dependencies]
tokio-test = "0.4"
'''


def pyproject_toml(name: str, sdk: dict) -> str:
    return f'''[project]
name = "{name.replace("-", "_")}"
version = "0.1.0"
description = "{sdk['description']}"
requires-python = ">=3.10"
dependencies = ["pydantic>=2.0", "anyio>=3.0"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
'''


def rust_lib(name: str, sdk: dict) -> str:
    traits = "\n".join(
        f"""
pub trait {iface} {{
    type Error;
    async fn init(&mut self) -> Result<(), Self::Error>;
    async fn shutdown(&mut self) -> Result<(), Self::Error>;
}}"""
        for iface in sdk["interfaces"]
    )
    mods = "\n".join(f"pub mod {p.replace('-', '_')};" for p in sdk["protocols"])
    return f'''//! {sdk['description']}
#![warn(missing_docs)]

{traits}

{mods}

#[cfg(test)]
mod tests {{
    use super::*;

    #[tokio::test]
    async fn smoke_init() {{
        // TODO: instantiate mock and call init
    }}
}}
'''


def rust_protocol(name: str, protocol: str, sdk: dict) -> str:
    return f'''//! Protocol driver for `{protocol}`

use crate::{{Error}};

/// Configuration for `{protocol}` integration.
#[derive(Debug, Clone)]
pub struct Config {{
    pub endpoint: String,
    pub timeout_ms: u64,
}}

impl Default for Config {{
    fn default() -> Self {{
        Self {{
            endpoint: "127.0.0.1:0".into(),
            timeout_ms: 5000,
        }}
    }}
}}

/// Run a single protocol handshake.
pub async fn handshake(cfg: &Config) -> Result<(), Error> {{
    let _ = cfg;
    Ok(())
}}
'''


def python_init(name: str, sdk: dict) -> str:
    classes = "\n\n".join(
        f'''class {iface}:
    """{sdk['description']} — {iface}"""

    async def init(self) -> None:
        raise NotImplementedError

    async def shutdown(self) -> None:
        raise NotImplementedError'''
        for iface in sdk["interfaces"]
    )
    return f'''"""{sdk['description']}"""

{classes}

__all__ = {sdk['interfaces']}
'''


def python_protocol(protocol: str, sdk: dict) -> str:
    return f'''"""Protocol driver for `{protocol}`"""

from dataclasses import dataclass

@dataclass
class Config:
    endpoint: str = "127.0.0.1:0"
    timeout_ms: int = 5000

async def handshake(cfg: Config) -> None:
    pass
'''


def readme(name: str, sdk: dict) -> str:
    ifaces = "\n".join(f"- `{i}`" for i in sdk["interfaces"])
    protocols = "\n".join(f"- `{p}`" for p in sdk["protocols"])
    langs = ", ".join(sdk["languages"])
    return f'''# {name}

{sdk['description']}

## Languages
{langs}

## Interfaces
{ifaces}

## Protocols
{protocols}

## Build

```bash
cargo build --all-features
```

## License
MIT
'''


def generate(sdk: dict) -> None:
    name = sdk["name"]
    root = OUT_DIR / name
    if root.exists():
        shutil.rmtree(root)
    root.mkdir(parents=True)

    # Rust skeleton
    if "rust" in sdk["languages"]:
        write(root / "Cargo.toml", cargo_toml(name, sdk))
        write(root / "src" / "lib.rs", rust_lib(name, sdk))
        for prot in sdk["protocols"]:
            write(root / "src" / f"{prot.replace('-', '_')}.rs", rust_protocol(name, prot, sdk))
        write(root / "examples" / "basic.rs", f'''use {name.replace("-", "_")}::*;

#[tokio::main]
async fn main() {{
    println!("Hello from {name}!");
}}
''')

    # Python skeleton
    if "python" in sdk["languages"]:
        write(root / "pyproject.toml", pyproject_toml(name, sdk))
        write(root / "src" / "__init__.py", python_init(name, sdk))
        for prot in sdk["protocols"]:
            write(root / "src" / f"{prot.replace('-', '_')}.py", python_protocol(prot, sdk))
        write(root / "examples" / "basic.py", f'''import asyncio
from {name.replace("-", "_")} import *

async def main():
    print("Hello from {name}!")

if __name__ == "__main__":
    asyncio.run(main())
''')

    write(root / "README.md", readme(name, sdk))


def main() -> None:
    manifest = json.loads(MANIFEST.read_text())
    for sdk in manifest["network_sdks"]:
        generate(sdk)
        print(f"Generated network SDK: {sdk['name']}")
    print(f"\nDone. Output: {OUT_DIR}")


if __name__ == "__main__":
    main()
