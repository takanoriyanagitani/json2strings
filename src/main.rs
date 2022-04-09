use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

mod json2strings;

use crate::json2strings::{new_string_consumer, StringVecConsumer};

fn on_rw<R, W>(r: &mut R, w: &mut W) -> Result<(), String>
where
    R: Read,
    W: Write,
{
    let mut consumer = new_string_consumer(BufWriter::new(w.by_ref()));
    consumer.consume_reader(BufReader::new(r))?;
    drop(consumer);

    w.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

fn sub() -> Result<(), String> {
    let i = stdin();
    let mut il = i.lock();

    let o = stdout();
    let mut ol = o.lock();

    on_rw(&mut il, &mut ol)
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
