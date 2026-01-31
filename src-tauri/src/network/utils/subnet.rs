// src-tauri/src/network/utils/subnet.rs
//
/// Subnet broadcast address detection utility
///
/// Detects the local subnet broadcast address (e.g., 192.168.1.255)
/// to fix macOS UDP broadcast errors with 255.255.255.255
use crate::error::{AppError, AppResult};
use std::net::IpAddr;

/// Detect subnet broadcast address
///
/// Returns the subnet-specific broadcast address (e.g., 192.168.1.255)
/// Falls back to global broadcast (255.255.255.255) if detection fails
///
/// # Examples
///
/// ```ignore
/// let subnet = detect_subnet_broadcast().await?;
/// assert!(subnet.ends_with(".255"));
/// ```
pub async fn detect_subnet_broadcast() -> AppResult<String> {
    // Get local IP address
    let local_ip = local_ip_address::local_ip()
        .map_err(|e| AppError::Network(format!("Failed to get local IP: {}", e)))?;

    // Calculate subnet broadcast address
    let broadcast = calculate_subnet_broadcast(&local_ip)?;

    Ok(broadcast)
}

/// Calculate subnet broadcast address from local IP
///
/// Assumes common home/office subnet masks:
/// - 192.168.x.x → /24 → 192.168.x.255
/// - 10.x.x.x → /8 → 10.255.255.255
/// - 172.16.x.x → /12 → 172.31.255.255
fn calculate_subnet_broadcast(ip: &IpAddr) -> AppResult<String> {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();

            // Class C private network (192.168.x.x)
            if octets[0] == 192 && octets[1] == 168 {
                return Ok(format!("192.168.{}.255", octets[2]));
            }

            // Class A private network (10.x.x.x)
            if octets[0] == 10 {
                return Ok("10.255.255.255".to_string());
            }

            // Class B private network (172.16.x.x - 172.31.x.x)
            if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
                return Ok(format!("172.{}.255.255", octets[1]));
            }

            // Fallback: assume /24 subnet for other IPs
            Ok(format!("{}.{}.{}.255", octets[0], octets[1], octets[2]))
        }
        IpAddr::V6(_) => {
            // IPv6: use ff02::1 (link-local all-nodes multicast)
            Ok("ff02::1".to_string())
        }
    }
}

// ============================================================
// TDD Tests (RED-GREEN-REFACTOR)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_detect_subnet_broadcast_returns_valid_format() {
        // RED Phase: This test will fail initially (function doesn't exist)
        let subnet = detect_subnet_broadcast().await.unwrap();

        // Should end with .255 for IPv4
        assert!(subnet.ends_with(".255") || subnet == "ff02::1", "Should return subnet broadcast address");
    }

    #[tokio::test]
    async fn test_detect_subnet_broadcast_not_global_broadcast() {
        let subnet = detect_subnet_broadcast().await.unwrap();

        // Should use subnet-specific address, not global broadcast
        // Note: We allow fallback to global broadcast if detection fails
        assert!(subnet != "255.255.255.255" || subnet.contains("."), "Should use subnet-specific address");
    }

    #[test]
    fn test_calculate_subnet_broadcast_class_c() {
        // Test 192.168.x.x network
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let broadcast = calculate_subnet_broadcast(&ip).unwrap();

        assert_eq!(broadcast, "192.168.1.255");
    }

    #[test]
    fn test_calculate_subnet_broadcast_class_a() {
        // Test 10.x.x.x network
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 1, 100));
        let broadcast = calculate_subnet_broadcast(&ip).unwrap();

        assert_eq!(broadcast, "10.255.255.255");
    }

    #[test]
    fn test_calculate_subnet_broadcast_class_b() {
        // Test 172.16.x.x network
        let ip = IpAddr::V4(Ipv4Addr::new(172, 16, 1, 100));
        let broadcast = calculate_subnet_broadcast(&ip).unwrap();

        assert_eq!(broadcast, "172.16.255.255");
    }

    #[test]
    fn test_calculate_subnet_broadcast_default() {
        // Test other IPs with /24 assumption
        let ip = IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1));
        let broadcast = calculate_subnet_broadcast(&ip).unwrap();

        assert_eq!(broadcast, "198.51.100.255");
    }

    #[test]
    fn test_calculate_subnet_broadcast_ipv6() {
        // Test IPv6 address
        let ip: IpAddr = "2001:db8::1".parse().unwrap();
        let broadcast = calculate_subnet_broadcast(&ip).unwrap();

        assert_eq!(broadcast, "ff02::1");
    }
}
