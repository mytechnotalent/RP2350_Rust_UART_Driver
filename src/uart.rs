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

/// UART controller with echo tracking.
///
/// # Details
/// Maintains UART echo count for statistics.
/// Provides methods for character processing.
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

    /// Processes a received character and prepares for echo.
    ///
    /// # Details
    /// Increments echo count and returns character for echo.
    ///
    /// # Arguments
    /// * `ch` - The character received
    ///
    /// # Returns
    /// * `u8` - The character to echo back
    pub fn process_char(&mut self, ch: u8) -> u8 {
        self.echo_count += 1;
        ch
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
        assert_eq!(ctrl.process_char(b'A'), b'A');
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
        assert_eq!(ctrl.process_char(b'\n'), b'\n');
        assert_eq!(ctrl.process_char(b'\r'), b'\r');
    }
}
