use std::io::{self, Write};
use termcolor::{ColorSpec, WriteColor};

/// A `Terminal` that just ignores all attempts at formatting. Used
/// to report errors when no ANSI terminfo is available.
pub struct FakeTerminal<W: Write> {
    write: W,
}

impl<W: Write> FakeTerminal<W> {
    pub fn new(write: W) -> FakeTerminal<W> {
        FakeTerminal { write }
    }
}

impl<W: Write> Write for FakeTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write.flush()
    }
}

impl<W: Write> WriteColor for FakeTerminal<W> {
    fn supports_color(&self) -> bool {
        false
    }
    fn set_color(&mut self, _: &ColorSpec) -> io::Result<()> {
        Ok(())
    }
    fn reset(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn is_synchronous(&self) -> bool {
        true
    }
}
