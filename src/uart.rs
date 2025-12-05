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
//! UPDATE DATE: December 4, 2025

use crate::config::{MAX_UART_BAUD_RATE, MIN_UART_BAUD_RATE, UART_BAUD_RATE};

/// UART state enumeration.
///
/// # Details
/// Represents the current state of the UART.
/// Used for state tracking and transitions.
///
/// # Variants
/// * `Idle` - UART is waiting for input
/// * `Receiving` - UART is receiving a character
/// * `Echoing` - UART is echoing the character back
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UartState {
    Idle,
    Receiving,
    Echoing,
}

/// UART controller with state tracking.
///
/// # Details
/// Maintains UART state and baud rate configuration.
/// Provides methods for state transitions and queries.
///
/// # Fields
/// * `state` - Current UART state
/// * `baud_rate` - UART baud rate
/// * `echo_count` - Number of characters echoed
/// * `last_char` - Last character received
#[derive(Debug)]
pub struct UartController {
    state: UartState,
    baud_rate: u32,
    echo_count: u64,
    last_char: Option<u8>,
}

/// Default implementation for UartController.
impl Default for UartController {
    fn default() -> Self {
        Self {
            state: UartState::Idle,
            baud_rate: UART_BAUD_RATE,
            echo_count: 0,
            last_char: None,
        }
    }
}

/// Public methods for UartController
impl UartController {
    /// Creates new UART controller with default settings.
    ///
    /// # Details
    /// Initializes controller with UART idle using Default trait.
    /// Ready to receive characters immediately.
    ///
    /// # Returns
    /// * `Self` - New UartController instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates new UART controller with custom baud rate.
    ///
    /// # Details
    /// Initializes controller with specified baud rate, clamped to valid range.
    /// UART starts in idle state.
    ///
    /// # Arguments
    /// * `baud_rate` - Desired UART baud rate
    ///
    /// # Returns
    /// * `Self` - New UartController with configured baud rate
    #[allow(dead_code)]
    pub fn with_baud_rate(baud_rate: u32) -> Self {
        Self {
            state: UartState::Idle,
            baud_rate: clamp_baud_rate(baud_rate),
            echo_count: 0,
            last_char: None,
        }
    }

    /// Processes a received character and prepares for echo.
    ///
    /// # Details
    /// Transitions UART from Idle to Receiving to Echoing.
    /// Stores the character for echo and increments counter.
    ///
    /// # Arguments
    /// * `ch` - The character received
    ///
    /// # Returns
    /// * `u8` - The character to echo back
    pub fn process_char(&mut self, ch: u8) -> u8 {
        self.state = UartState::Receiving;
        self.last_char = Some(ch);
        self.state = UartState::Echoing;
        self.echo_count += 1;
        self.state = UartState::Idle;
        ch
    }

    /// Returns current UART state.
    ///
    /// # Returns
    /// * `UartState` - Current UART state
    #[allow(dead_code)]
    pub fn state(&self) -> UartState {
        self.state
    }

    /// Returns current UART baud rate.
    ///
    /// # Returns
    /// * `u32` - Baud rate
    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    /// Returns total echo count.
    ///
    /// # Returns
    /// * `u64` - Number of characters echoed
    #[allow(dead_code)]
    pub fn echo_count(&self) -> u64 {
        self.echo_count
    }

    /// Returns the last character received.
    ///
    /// # Returns
    /// * `Option<u8>` - Last character or None
    #[allow(dead_code)]
    pub fn last_char(&self) -> Option<u8> {
        self.last_char
    }

    /// Checks if UART is currently idle.
    ///
    /// # Returns
    /// * `bool` - true if UART is idle
    #[allow(dead_code)]
    pub fn is_idle(&self) -> bool {
        self.state == UartState::Idle
    }

    /// Sets new UART baud rate, clamped to valid range.
    ///
    /// # Arguments
    /// * `baud_rate` - New baud rate
    #[allow(dead_code)]
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = clamp_baud_rate(baud_rate);
    }
}

/// Clamps baud rate value to valid range.
///
/// # Details
/// Ensures baud rate falls within MIN_UART_BAUD_RATE and MAX_UART_BAUD_RATE.
///
/// # Arguments
/// * `baud_rate` - Baud rate to clamp
///
/// # Returns
/// * `u32` - Clamped baud rate value
#[allow(dead_code)]
fn clamp_baud_rate(baud_rate: u32) -> u32 {
    baud_rate.clamp(MIN_UART_BAUD_RATE, MAX_UART_BAUD_RATE)
}

/// Converts character to its echo representation.
///
/// # Details
/// Returns the same character for echo.
/// Can be modified for custom echo behavior.
///
/// # Arguments
/// * `ch` - Character to convert
///
/// # Returns
/// * `u8` - Character for echo
pub fn char_to_echo(ch: u8) -> u8 {
    ch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let ctrl = UartController::new();
        assert_eq!(ctrl.state(), UartState::Idle);
        assert_eq!(ctrl.baud_rate(), UART_BAUD_RATE);
        assert_eq!(ctrl.echo_count(), 0);
    }

    #[test]
    fn test_with_baud_rate() {
        let ctrl = UartController::with_baud_rate(57600);
        assert_eq!(ctrl.baud_rate(), 57600);
    }

    #[test]
    fn test_with_baud_rate_clamps_low() {
        let ctrl = UartController::with_baud_rate(100);
        assert_eq!(ctrl.baud_rate(), MIN_UART_BAUD_RATE);
    }

    #[test]
    fn test_with_baud_rate_clamps_high() {
        let ctrl = UartController::with_baud_rate(10000000);
        assert_eq!(ctrl.baud_rate(), MAX_UART_BAUD_RATE);
    }

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
    fn test_process_char_stores_last() {
        let mut ctrl = UartController::new();
        ctrl.process_char(b'X');
        assert_eq!(ctrl.last_char(), Some(b'X'));
    }

    #[test]
    fn test_is_idle_true_initially() {
        let ctrl = UartController::new();
        assert!(ctrl.is_idle());
    }

    #[test]
    fn test_is_idle_after_process() {
        let mut ctrl = UartController::new();
        ctrl.process_char(b'A');
        assert!(ctrl.is_idle());
    }

    #[test]
    fn test_set_baud_rate() {
        let mut ctrl = UartController::new();
        ctrl.set_baud_rate(38400);
        assert_eq!(ctrl.baud_rate(), 38400);
    }

    #[test]
    fn test_set_baud_rate_clamps() {
        let mut ctrl = UartController::new();
        ctrl.set_baud_rate(100);
        assert_eq!(ctrl.baud_rate(), MIN_UART_BAUD_RATE);
    }

    #[test]
    fn test_char_to_echo() {
        assert_eq!(char_to_echo(b'A'), b'A');
    }

    #[test]
    fn test_char_to_echo_special() {
        assert_eq!(char_to_echo(b'\n'), b'\n');
    }

    #[test]
    fn test_clamp_baud_rate_within_range() {
        assert_eq!(clamp_baud_rate(115200), 115200);
    }

    #[test]
    fn test_clamp_baud_rate_below_min() {
        assert_eq!(clamp_baud_rate(100), MIN_UART_BAUD_RATE);
    }

    #[test]
    fn test_clamp_baud_rate_above_max() {
        assert_eq!(clamp_baud_rate(10000000), MAX_UART_BAUD_RATE);
    }
}
