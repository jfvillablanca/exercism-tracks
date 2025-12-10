use std::io::{BufReader, BufWriter, Read, Result, Write};

pub struct ReadStats<R> {
    reader: BufReader<R>,
    bytes_through: usize,
    reads: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self { reader: BufReader::new(wrapped), bytes_through: 0, reads: 0 }
    }

    pub fn get_ref(&self) -> &R {
        self.reader.get_ref()
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_through = self.reader.read(buf)?;
        self.bytes_through = self.bytes_through.saturating_add(bytes_through);
        self.reads = self.reads.saturating_add(1);
        Ok(bytes_through)
    }
}

#[derive(Debug)]
pub struct WriteStats<W: Write> {
    writer: BufWriter<W>,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self { writer: BufWriter::new(wrapped), bytes_through: 0, writes: 0 }
    }

    pub fn get_ref(&self) -> &W {
        self.writer.get_ref()
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_through = self.writer.get_mut().write(buf)?;
        self.bytes_through = self.bytes_through.saturating_add(bytes_through);
        self.writes = self.writes.saturating_add(1);
        Ok(bytes_through)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
