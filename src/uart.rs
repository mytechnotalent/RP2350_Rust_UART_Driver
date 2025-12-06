/*
 * @file uart.rs
 * @brief UART echo state machine
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

//! FILE: uart.rs
//!
//! DESCRIPTION:
//! RP2350 UART Echo State Machine.
//!
//! BRIEF:
//! Implements UART character echo logic.
//! Provides testable state machine for echo functionality.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 4, 2025
//! UPDATE DATE: December 5, 2025

/// Backspace character code.
const BACKSPACE: u8 = 0x08;

/// Delete character code.
const DELETE: u8 = 0x7F;

/// Backspace erase sequence: backspace, space, backspace.
const BACKSPACE_SEQ: [u8; 3] = [0x08, b' ', 0x08];

/// UART controller with echo tracking.
///
/// # Details
/// Maintains UART echo count for statistics.
/// Provides methods for character processing with backspace support.
///
/// # Fields
/// * `echo_count` - Number of characters echoed
#[derive(Debug)]
pub struct UartController {
    echo_count: u64,
}

impl Default for UartController {
    /// Returns default UartController instance.
    ///
    /// # Details
    /// Delegates to new() for initialization.
    ///
    /// # Returns
    /// * `Self` - New UartController with default values
    fn default() -> Self {
        Self::new()
    }
}

impl UartController {
    /// Creates new UART controller with default settings.
    ///
    /// # Details
    /// Initializes controller with zero echo count.
    /// Ready to receive characters immediately.
    ///
    /// # Returns
    /// * `Self` - New UartController instance
    pub fn new() -> Self {
        Self { echo_count: 0 }
    }

    /// Processes a received character and returns echo response.
    ///
    /// # Details
    /// Handles backspace by returning erase sequence.
    /// Normal characters are echoed as-is.
    ///
    /// # Arguments
    /// * `ch` - The character received
    ///
    /// # Returns
    /// * `&'static [u8]` - Bytes to echo back
    pub fn process_char(&mut self, ch: u8) -> &'static [u8] {
        self.echo_count += 1;
        if ch == BACKSPACE || ch == DELETE {
            &BACKSPACE_SEQ
        } else {
            match ch {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => {
                    static CHARS: [u8; 62] = [
                        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
                        b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
                        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
                        b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
                        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                        b'8', b'9',
                    ];
                    let idx = CHARS.iter().position(|&c| c == ch).unwrap();
                    &CHARS[idx..idx + 1]
                }
                b' ' => b" ",
                b'!' => b"!",
                b'"' => b"\"",
                b'#' => b"#",
                b'$' => b"$",
                b'%' => b"%",
                b'&' => b"&",
                b'\'' => b"\'",
                b'(' => b"(",
                b')' => b")",
                b'*' => b"*",
                b'+' => b"+",
                b',' => b",",
                b'-' => b"-",
                b'.' => b".",
                b'/' => b"/",
                b':' => b":",
                b';' => b";",
                b'<' => b"<",
                b'=' => b"=",
                b'>' => b">",
                b'?' => b"?",
                b'@' => b"@",
                b'[' => b"[",
                b'\\' => b"\\",
                b']' => b"]",
                b'^' => b"^",
                b'_' => b"_",
                b'`' => b"`",
                b'{' => b"{",
                b'|' => b"|",
                b'}' => b"}",
                b'~' => b"~",
                b'\n' => b"\n",
                b'\r' => b"\r",
                b'\t' => b"\t",
                _ => b"",
            }
        }
    }

    /// Returns total echo count.
    ///
    /// # Returns
    /// * `u64` - Number of characters echoed
    #[allow(dead_code)]
    pub fn echo_count(&self) -> u64 {
        self.echo_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== UartController Construction Tests ====================

    #[test]
    fn test_new_controller() {
        let ctrl = UartController::new();
        assert_eq!(ctrl.echo_count(), 0);
    }

    #[test]
    fn test_default_equals_new() {
        let default = UartController::default();
        let new = UartController::new();
        assert_eq!(default.echo_count(), new.echo_count());
    }

    // ==================== Character Processing Tests ====================

    #[test]
    fn test_process_char_returns_same() {
        let mut ctrl = UartController::new();
        assert_eq!(ctrl.process_char(b'A'), b"A");
    }

    #[test]
    fn test_process_char_increments_count() {
        let mut ctrl = UartController::new();
        ctrl.process_char(b'A');
        ctrl.process_char(b'B');
        ctrl.process_char(b'C');
        assert_eq!(ctrl.echo_count(), 3);
    }

    #[test]
    fn test_process_char_special() {
        let mut ctrl = UartController::new();
        assert_eq!(ctrl.process_char(b'\n'), b"\n");
        assert_eq!(ctrl.process_char(b'\r'), b"\r");
    }

    #[test]
    fn test_process_backspace() {
        let mut ctrl = UartController::new();
        assert_eq!(ctrl.process_char(0x08), &[0x08, b' ', 0x08]);
    }

    #[test]
    fn test_process_delete() {
        let mut ctrl = UartController::new();
        assert_eq!(ctrl.process_char(0x7F), &[0x08, b' ', 0x08]);
    }
}
