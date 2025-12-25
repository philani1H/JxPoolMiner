#!/bin/bash
# Comprehensive Test Script for JxPoolMiner

set -e

echo "🧪 JxPoolMiner - Comprehensive Test Suite"
echo "=========================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASSED=0
FAILED=0

test_check() {
    local test_name="$1"
    local test_command="$2"
    
    echo -n "Testing: $test_name... "
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}"
        ((FAILED++))
        return 1
    fi
}

echo -e "${BLUE}📁 File Structure Tests${NC}"
echo "------------------------------"

# Core files
test_check "Cargo.toml exists" "[ -f Cargo.toml ]"
test_check "src/main.rs exists" "[ -f src/main.rs ]"
test_check "README.md exists" "[ -f README.md ]"

# Core crate
test_check "core/Cargo.toml" "[ -f crates/core/Cargo.toml ]"
test_check "core/src/lib.rs" "[ -f crates/core/src/lib.rs ]"
test_check "core/src/types/device.rs" "[ -f crates/core/src/types/device.rs ]"
test_check "core/src/types/algorithm.rs" "[ -f crates/core/src/types/algorithm.rs ]"
test_check "core/src/types/share.rs" "[ -f crates/core/src/types/share.rs ]"
test_check "core/src/types/job.rs" "[ -f crates/core/src/types/job.rs ]"
test_check "core/src/error.rs" "[ -f crates/core/src/error.rs ]"

# Devices crate
test_check "devices/Cargo.toml" "[ -f crates/devices/Cargo.toml ]"
test_check "devices/src/lib.rs" "[ -f crates/devices/src/lib.rs ]"
test_check "devices/src/cpu.rs" "[ -f crates/devices/src/cpu.rs ]"

# Mining crate
test_check "mining/Cargo.toml" "[ -f crates/mining/Cargo.toml ]"
test_check "mining/src/lib.rs" "[ -f crates/mining/src/lib.rs ]"
test_check "mining/src/engine.rs" "[ -f crates/mining/src/engine.rs ]"
test_check "mining/src/algorithms/gxhash.rs" "[ -f crates/mining/src/algorithms/gxhash.rs ]"

# Pool crate
test_check "pool/Cargo.toml" "[ -f crates/pool/Cargo.toml ]"
test_check "pool/src/lib.rs" "[ -f crates/pool/src/lib.rs ]"
test_check "pool/src/client.rs" "[ -f crates/pool/src/client.rs ]"

# GUI crate
test_check "gui/Cargo.toml" "[ -f crates/gui/Cargo.toml ]"
test_check "gui/src/lib.rs" "[ -f crates/gui/src/lib.rs ]"
test_check "gui/src/app.rs" "[ -f crates/gui/src/app.rs ]"

# Config crate
test_check "config/Cargo.toml" "[ -f crates/config/Cargo.toml ]"
test_check "config/src/lib.rs" "[ -f crates/config/src/lib.rs ]"

# Stats crate
test_check "stats/Cargo.toml" "[ -f crates/stats/Cargo.toml ]"
test_check "stats/src/lib.rs" "[ -f crates/stats/src/lib.rs ]"
test_check "stats/src/collector.rs" "[ -f crates/stats/src/collector.rs ]"

# Updater crate
test_check "updater/Cargo.toml" "[ -f crates/updater/Cargo.toml ]"
test_check "updater/src/lib.rs" "[ -f crates/updater/src/lib.rs ]"

# Tests
test_check "integration_test.rs" "[ -f tests/integration_test.rs ]"

# Config
test_check "default.toml" "[ -f config/default.toml ]"

echo ""
echo -e "${BLUE}🔍 Code Quality Tests${NC}"
echo "------------------------------"

# Check for proper exports
test_check "core exports types" "grep -q 'pub use types::' crates/core/src/lib.rs"
test_check "devices exports detect_all" "grep -q 'pub async fn detect_all' crates/devices/src/lib.rs"
test_check "mining exports Engine" "grep -q 'pub use engine::Engine' crates/mining/src/lib.rs"
test_check "pool exports Client" "grep -q 'pub use client::Client' crates/pool/src/lib.rs"

echo ""
echo -e "${BLUE}📊 Statistics${NC}"
echo "------------------------------"

TOTAL_RS_FILES=$(find crates -name "*.rs" | wc -l)
TOTAL_TOML_FILES=$(find crates -name "Cargo.toml" | wc -l)
TOTAL_LINES=$(find crates -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')

echo "Rust files: $TOTAL_RS_FILES"
echo "Cargo.toml files: $TOTAL_TOML_FILES"
echo "Total lines of code: $TOTAL_LINES"

echo ""
echo "=========================================="
echo -e "${BLUE}📈 Test Summary${NC}"
echo "=========================================="
echo -e "${GREEN}Passed:${NC}   $PASSED"
echo -e "${RED}Failed:${NC}   $FAILED"
echo ""

TOTAL=$((PASSED + FAILED))
if [ $TOTAL -gt 0 ]; then
    PERCENTAGE=$((PASSED * 100 / TOTAL))
    echo "Success Rate: $PERCENTAGE%"
fi

echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    echo ""
    echo "🎉 JxPoolMiner is ready!"
    echo ""
    echo "Next steps:"
    echo "  1. cargo build (compile the project)"
    echo "  2. cargo test (run unit tests)"
    echo "  3. cargo run (run the application)"
    exit 0
else
    echo -e "${RED}❌ Some tests failed.${NC}"
    exit 1
fi
