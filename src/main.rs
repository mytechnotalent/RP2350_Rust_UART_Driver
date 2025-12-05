/*
 * @file main.rs
 * @brief Microcontroller entry point
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

//! FILE: main.rs
//!
//! DESCRIPTION:
//! RP2350 Embedded Rust Embassy UART Echo Application.
//!
//! BRIEF:
//! Main application entry point for RP2350 UART echo driver using Embassy.
//! Implements async UART character echo on GPIO 0 (TX) and GPIO 1 (RX).
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 4, 2025
//! UPDATE DATE: December 5, 2025

#![no_std]
#![no_main]

mod config;
mod uart;

use config::UART_BAUD_RATE;
use embassy_executor::Spawner;
use embassy_rp::uart::{Config, Uart};
use embassy_rp::{bind_interrupts, peripherals::UART0, uart::InterruptHandler};
use panic_halt as _;
use uart::UartController;

bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<UART0>;
});

/// Main application entry point.
///
/// # Details
/// Initializes Embassy runtime and runs the main UART echo loop.
/// Uses UartController for state management.
///
/// # Arguments
/// * `_spawner` - Embassy task spawner (reserved for future async tasks).
///
/// # Returns
/// * `()` - Never returns (infinite loop).
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut config = Config::default();
    config.baudrate = UART_BAUD_RATE;
    let mut uart = Uart::new(
        p.UART0, p.PIN_0, p.PIN_1, Irqs, p.DMA_CH0, p.DMA_CH1, config,
    );
    let mut controller = UartController::new();
    let mut buf = [0u8; 1];
    loop {
        if uart.read(&mut buf).await.is_ok() {
            let echo_char = controller.process_char(buf[0]);
            let _ = uart.write(&[echo_char]).await;
        }
    }
}
