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
//! Contains baud rate and control character configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 4, 2025
//! UPDATE DATE: December 5, 2025

/// Default UART baud rate.
///
/// # Details
/// Configures the UART communication speed.
/// Standard baud rate for serial communication.
///
/// # Value
/// 115200 baud
#[allow(dead_code)]
pub const UART_BAUD_RATE: u32 = 115200;

/// Backspace character code.
///
/// # Details
/// Typical backspace control sent by terminals.
///
/// # Value
/// 0x08
pub const BACKSPACE: u8 = 0x08;

/// Delete character code.
///
/// # Details
/// Some terminals send DEL (0x7F) for backspace.
///
/// # Value
/// 0x7F
pub const DELETE: u8 = 0x7F;

/// Backspace erase sequence: backspace, space, backspace.
///
/// # Details
/// When echoed to a terminal this sequence erases the previous character.
///
/// # Value
/// [0x08, b' ', 0x08]
pub const BACKSPACE_SEQ: [u8; 3] = [0x08, b' ', 0x08];

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== UART Configuration Tests ====================

    #[test]
    fn test_uart_baud_rate_default() {
        assert_eq!(UART_BAUD_RATE, 115200);
    }

    #[test]
    fn test_backspace_value() {
        assert_eq!(BACKSPACE, 0x08);
    }

    #[test]
    fn test_delete_value() {
        assert_eq!(DELETE, 0x7F);
    }

    #[test]
    fn test_backspace_seq_length() {
        assert_eq!(BACKSPACE_SEQ.len(), 3);
    }

    #[test]
    fn test_backspace_seq_first_byte() {
        assert_eq!(BACKSPACE_SEQ[0], 0x08);
    }

    #[test]
    fn test_backspace_seq_middle_byte() {
        assert_eq!(BACKSPACE_SEQ[1], b' ');
    }

    #[test]
    fn test_backspace_seq_last_byte() {
        assert_eq!(BACKSPACE_SEQ[2], 0x08);
    }

    #[test]
    fn test_backspace_not_delete() {
        assert_ne!(BACKSPACE, DELETE);
    }

    #[test]
    fn test_backspace_seq_contains_backspace() {
        assert!(BACKSPACE_SEQ.contains(&BACKSPACE));
    }

    #[test]
    fn test_backspace_seq_contains_space() {
        assert!(BACKSPACE_SEQ.contains(&b' '));
    }

    #[test]
    fn test_backspace_seq_full() {
        assert_eq!(BACKSPACE_SEQ, [0x08, b' ', 0x08]);
    }
}
