use std::io::{Read, Write};

use serde_json::{Map, Number, Value};

pub fn new_string_consumer<W>(w: W) -> impl StringVecConsumer
where
    W: Write,
{
    StringsWriter { w }
}

struct StringsWriter<W> {
    w: W,
}
impl<W> StringVecConsumer for StringsWriter<W>
where
    W: Write,
{
    fn consume(&mut self, v: Vec<String>) -> Result<u32, String> {
        serde_json::to_writer(&mut self.w, &v)
            .map_err(|e| format!("Unable to write json: {}", e))?;
        writeln!(self.w).map_err(|e| format!("Unable to write: {}", e))?;
        Ok(v.len() as u32)
    }
}

pub trait StringVecConsumer {
    fn consume(&mut self, v: Vec<String>) -> Result<u32, String>;

    fn consume_reader<R>(&mut self, r: R) -> Result<u32, String>
    where
        R: Read,
        Self: Sized,
    {
        let v: Value =
            serde_json::from_reader(r).map_err(|e| format!("Unable to parse json: {}", e))?;
        self.consume_value(vec![], v)
    }

    fn consume_value(&mut self, ancestor: Vec<String>, v: Value) -> Result<u32, String>
    where
        Self: Sized,
    {
        match v {
            Value::Null => Ok(0),
            Value::Bool(b) => self.consume_bool(b, ancestor),
            Value::Number(n) => self.consume_number(n, ancestor),
            Value::String(s) => self.consume_string(s, ancestor),
            Value::Array(v) => self.consume_vec(ancestor, v),
            Value::Object(m) => self.consume_map(ancestor, m),
        }
    }

    fn consume_vec(&mut self, ancestor: Vec<String>, v: Vec<Value>) -> Result<u32, String>
    where
        Self: Sized,
    {
        v.into_iter().enumerate().try_fold(0, |tot, (ix, item)| {
            let key: String = StringItem::from(ix).into();
            let mut dup = ancestor.clone();
            dup.push(key);
            match item {
                Value::Null => Ok(tot),
                Value::Bool(b) => self.consume_bool(b, dup).map(|cnt| cnt + tot),
                Value::Number(n) => self.consume_number(n, dup).map(|cnt| cnt + tot),
                Value::String(s) => self.consume_string(s, dup).map(|cnt| cnt + tot),
                Value::Array(v) => self.consume_vec(dup, v).map(|cnt| cnt + tot),
                Value::Object(m) => self.consume_map(dup, m).map(|cnt| cnt + tot),
            }
        })
    }

    fn consume_map(&mut self, ancestor: Vec<String>, m: Map<String, Value>) -> Result<u32, String>
    where
        Self: Sized,
    {
        m.into_iter().try_fold(0, |tot, (key, val)| {
            let mut dup = ancestor.clone();
            dup.push(key);
            match val {
                Value::Null => Ok(tot),
                Value::Bool(b) => self.consume_bool(b, dup).map(|cnt| cnt + tot),
                Value::Number(n) => self.consume_number(n, dup).map(|cnt| cnt + tot),
                Value::String(s) => self.consume_string(s, dup).map(|cnt| cnt + tot),
                Value::Array(v) => self.consume_vec(dup, v).map(|cnt| cnt + tot),
                Value::Object(m) => self.consume_map(dup, m).map(|cnt| cnt + tot),
            }
        })
    }

    fn consume_source<C>(&mut self, s: C, ancestor: Vec<String>) -> Result<u32, String>
    where
        C: StringVecSource<Self>,
        Self: Sized,
    {
        s.to_consumer(ancestor, self)
    }

    fn consume_item(&mut self, i: StringItem, ancestor: Vec<String>) -> Result<u32, String>
    where
        Self: Sized,
    {
        self.consume_source(i, ancestor)
    }

    fn consume_bool(&mut self, b: bool, ancestor: Vec<String>) -> Result<u32, String>
    where
        Self: Sized,
    {
        let item = StringItem::from(b);
        self.consume_item(item, ancestor)
    }

    fn consume_number(&mut self, n: Number, ancestor: Vec<String>) -> Result<u32, String>
    where
        Self: Sized,
    {
        let item = StringItem::from(n);
        self.consume_item(item, ancestor)
    }

    fn consume_string(&mut self, s: String, ancestor: Vec<String>) -> Result<u32, String>
    where
        Self: Sized,
    {
        let item = StringItem::from(s);
        self.consume_item(item, ancestor)
    }
}

pub trait StringVecSource<C>
where
    C: StringVecConsumer,
{
    fn to_consumer(self, ancestor: Vec<String>, c: &mut C) -> Result<u32, String>;
}

pub struct StringItem {
    raw: String,
}
impl<C> StringVecSource<C> for StringItem
where
    C: StringVecConsumer,
{
    fn to_consumer(self, mut ancestor: Vec<String>, c: &mut C) -> Result<u32, String> {
        ancestor.push(self.raw);
        c.consume(ancestor)
    }
}

impl From<StringItem> for String {
    fn from(s: StringItem) -> Self {
        s.raw
    }
}
impl From<usize> for StringItem {
    fn from(u: usize) -> Self {
        let raw: String = u.to_string();
        Self { raw }
    }
}

impl From<bool> for StringItem {
    fn from(b: bool) -> Self {
        let raw = b.to_string();
        Self { raw }
    }
}

impl From<Number> for StringItem {
    fn from(n: Number) -> Self {
        let raw: String = n.to_string();
        Self { raw }
    }
}

impl From<String> for StringItem {
    fn from(raw: String) -> Self {
        Self { raw }
    }
}
