#!/usr/bin/env python3
"""Test JxPoolMiner structure"""

import os
from pathlib import Path

GREEN = '\033[0;32m'
RED = '\033[0;31m'
BLUE = '\033[0;34m'
NC = '\033[0m'

passed = 0
failed = 0

def test_file(name, path):
    global passed, failed
    print(f"Testing: {name}... ", end='')
    if Path(path).exists():
        print(f"{GREEN}✅ PASS{NC}")
        passed += 1
        return True
    else:
        print(f"{RED}❌ FAIL{NC}")
        failed += 1
        return False

print(f"{BLUE}🧪 JxPoolMiner - Structure Test{NC}")
print("=" * 50)
print()

print(f"{BLUE}📁 Core Files{NC}")
test_file("Cargo.toml", "Cargo.toml")
test_file("src/main.rs", "src/main.rs")
test_file("README.md", "README.md")

print()
print(f"{BLUE}📦 Core Crate{NC}")
test_file("core/Cargo.toml", "crates/core/Cargo.toml")
test_file("core/lib.rs", "crates/core/src/lib.rs")
test_file("core/device.rs", "crates/core/src/types/device.rs")
test_file("core/algorithm.rs", "crates/core/src/types/algorithm.rs")
test_file("core/share.rs", "crates/core/src/types/share.rs")
test_file("core/job.rs", "crates/core/src/types/job.rs")
test_file("core/error.rs", "crates/core/src/error.rs")

print()
print(f"{BLUE}🔧 Devices Crate{NC}")
test_file("devices/Cargo.toml", "crates/devices/Cargo.toml")
test_file("devices/lib.rs", "crates/devices/src/lib.rs")
test_file("devices/cpu.rs", "crates/devices/src/cpu.rs")

print()
print(f"{BLUE}⛏️  Mining Crate{NC}")
test_file("mining/Cargo.toml", "crates/mining/Cargo.toml")
test_file("mining/lib.rs", "crates/mining/src/lib.rs")
test_file("mining/engine.rs", "crates/mining/src/engine.rs")
test_file("mining/gxhash.rs", "crates/mining/src/algorithms/gxhash.rs")

print()
print(f"{BLUE}🌐 Pool Crate{NC}")
test_file("pool/Cargo.toml", "crates/pool/Cargo.toml")
test_file("pool/lib.rs", "crates/pool/src/lib.rs")
test_file("pool/client.rs", "crates/pool/src/client.rs")

print()
print(f"{BLUE}🖥️  GUI Crate{NC}")
test_file("gui/Cargo.toml", "crates/gui/Cargo.toml")
test_file("gui/lib.rs", "crates/gui/src/lib.rs")
test_file("gui/app.rs", "crates/gui/src/app.rs")

print()
print(f"{BLUE}⚙️  Config Crate{NC}")
test_file("config/Cargo.toml", "crates/config/Cargo.toml")
test_file("config/lib.rs", "crates/config/src/lib.rs")

print()
print(f"{BLUE}📊 Stats Crate{NC}")
test_file("stats/Cargo.toml", "crates/stats/Cargo.toml")
test_file("stats/lib.rs", "crates/stats/src/lib.rs")
test_file("stats/collector.rs", "crates/stats/src/collector.rs")

print()
print(f"{BLUE}🔄 Updater Crate{NC}")
test_file("updater/Cargo.toml", "crates/updater/Cargo.toml")
test_file("updater/lib.rs", "crates/updater/src/lib.rs")

print()
print(f"{BLUE}🧪 Tests{NC}")
test_file("integration_test.rs", "tests/integration_test.rs")

print()
print(f"{BLUE}📝 Config{NC}")
test_file("default.toml", "config/default.toml")

print()
print("=" * 50)
print(f"{BLUE}📈 Summary{NC}")
print("=" * 50)
print(f"{GREEN}Passed:{NC} {passed}")
print(f"{RED}Failed:{NC} {failed}")

total = passed + failed
if total > 0:
    percentage = (passed * 100) // total
    print(f"Success Rate: {percentage}%")

# Count files
rs_files = len(list(Path("crates").rglob("*.rs")))
toml_files = len(list(Path("crates").rglob("Cargo.toml")))

print()
print(f"{BLUE}📊 Statistics{NC}")
print(f"Rust files: {rs_files}")
print(f"Cargo.toml files: {toml_files}")

print()
if failed == 0:
    print(f"{GREEN}✅ All tests passed!{NC}")
    print()
    print("🎉 JxPoolMiner structure is complete!")
    print()
    print("Next steps:")
    print("  1. cargo build (requires Rust)")
    print("  2. cargo test")
    print("  3. cargo run")
else:
    print(f"{RED}❌ Some tests failed{NC}")
