# Phase 10 - Issue Log

## [2026-02-01] FeiQ Communication Issue

### User Report

**Environment**:

- Machine A: 192.168.0.23 (macOS) - Running our FeiQ implementation
- Machine B: 192.168.0.21 (unknown OS) - Running FeiQ client
- **Issue**: No communication between the two instances

### Diagnosis Performed

1. **Machine A (192.168.0.23) Status**: ✅ Working
   - UDP socket bound to 0.0.0.0:2425
   - Broadcasting to 192.168.0.255:2425
   - Receiving loopback messages (self)
   - **Not receiving** messages from 192.168.0.21

2. **Network Configuration**: ✅ Correct
   - Subnet: 192.168.0.0/24
   - Broadcast address: 192.168.0.255
   - Multiple interfaces present (192.168.0.23, 10.x.x.x, 10.x.x.x)

3. **Firewall Status**: ⚠️ **ENABLED**
   ```
   Firewall is enabled. (State = 1)
   ```

   - **Likely blocking incoming UDP packets**

### Root Cause Analysis

**Primary Suspect**: macOS Firewall is blocking UDP broadcast packets

**Evidence**:

- Socket successfully bound and listening
- Sending works (loopback received)
- No external messages received
- Firewall is enabled

**Secondary Possibilities**:

1. Machine B (192.168.0.21) not actually running FeiQ
2. Machine B has firewall enabled
3. Protocol incompatibility (if using official FeiQ client)
4. Network routing issues

### Recommended Solutions

#### Solution 1: Disable Firewall (Testing)

```bash
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off
```

#### Solution 2: Add Firewall Exception

- System Preferences → Security & Privacy → Firewall
- Add 飞秋通讯.app
- Allow incoming connections

#### Solution 3: Verify Machine B

- Confirm FeiQ is actually running on 192.168.0.21
- Check firewall on Machine B
- Verify same subnet (192.168.0.0/24)

### Status

**BLOCKER**: Cannot test Phase 10 features properly until basic communication works

**Impact**: Medium - Can still develop UI features, but integration testing blocked

**Next Steps**:

1. User to test with firewall disabled
2. Document results
3. If fixed, update troubleshooting guide
4. If not fixed, investigate further (network capture, etc.)

### Related Tasks

This affects:

- Task 1.1.2: Search backend (needs working communication for testing)
- Task 3.1: Feature testing (integration tests require multiple instances)
- Task 2.2: Performance optimization (needs real-world testing)

---

_Last Updated: 2026-02-01_
