#![no_std]
#![no_main]

use core::pin::{ pin };
use core::task::{ Context, Waker };
use core::panic::{ PanicInfo };
use core::fmt::{ self, Write };
use rusty_scrapyard_lib::io::{ Send };
use rusty_scrapyard_iomem::{ IOBufMem };
use rusty_scrapyard_lib::ns16550::{ NS16550 };

#[derive(Copy, Clone, Default)]
struct LogChan { }

impl Write for LogChan {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        Ok(())
    }
}

enum Error {
    Fatal,
}

async fn mainloop() -> Result<(), Error> {
    let iomem = IOBufMem::new(0x1000_0000, 0x08);
    let uart = NS16550::new(iomem);

    let msg = "
        hello world!
        hello world!
        hello world!
        hello world!
        hello world!
        hello world!
        hello world!
        hello world!
    ";
    uart.send(msg.as_bytes()).await;

    Ok(())
}

#[unsafe(export_name = "secret_start")]
fn main() {
    let mut ctx = Context::from_waker(Waker::noop());
    let mainloop = pin!(mainloop());
    let _ = mainloop.poll(&mut ctx);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
