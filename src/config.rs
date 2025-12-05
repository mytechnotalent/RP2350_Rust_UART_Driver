/*
 * @file config.rs
 * @brief Application configuration constants
 * @author Kevin Thomas
 * @date 2025
 *
 * MIT License
 *
 * Copyright (c) 2025 Kevin Thomas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! FILE: config.rs
//!
//! DESCRIPTION:
//! RP2350 UART Configuration Constants.
//!
//! BRIEF:
//! Defines configuration constants for UART communication.
//! Contains baud rate and buffer size configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 4, 2025
//! UPDATE DATE: December 4, 2025

/// Default UART baud rate.
///
/// # Details
/// Configures the UART communication speed.
/// Standard baud rate for serial communication.
///
/// # Value
/// 115200 baud
pub const UART_BAUD_RATE: u32 = 115200;

/// Minimum allowed UART baud rate.
///
/// # Details
/// Prevents excessively slow communication rates.
///
/// # Value
/// 9600 baud
#[allow(dead_code)]
pub const MIN_UART_BAUD_RATE: u32 = 9600;

/// Maximum allowed UART baud rate.
///
/// # Details
/// Prevents excessively fast communication rates for stability.
///
/// # Value
/// 921600 baud
#[allow(dead_code)]
pub const MAX_UART_BAUD_RATE: u32 = 921600;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uart_baud_rate_default() {
        assert_eq!(UART_BAUD_RATE, 115200);
    }

    #[test]
    fn test_min_baud_rate_less_than_default() {
        assert!(MIN_UART_BAUD_RATE < UART_BAUD_RATE);
    }

    #[test]
    fn test_max_baud_rate_greater_than_default() {
        assert!(MAX_UART_BAUD_RATE > UART_BAUD_RATE);
    }

    #[test]
    fn test_baud_rate_range_valid() {
        assert!(MIN_UART_BAUD_RATE < MAX_UART_BAUD_RATE);
    }
}
