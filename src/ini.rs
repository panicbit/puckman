use std::str::Lines;

pub fn parse(file: &str) -> IniParser {
    IniParser::new(file)
}

pub struct IniParser<'l> {
    lines: Lines<'l>,
}

impl<'l> IniParser<'l> {
    fn new(file: &'l str) -> Self {
        Self {
            lines: file.lines(),
        }
    }

    fn next_item(&mut self) -> Option<Item<'l>>{
        loop {
            let line = self.lines.next()?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#'){
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let section = &line[1..line.len() - 1];

                return Some(Item::Section(section));
            }

            let (key, value) = line
                .split_once('=')
                .map(|(key, value)| (key, Some(value)))
                .unwrap_or((line, None));
            let key = key.trim();
            let value = value.map(str::trim);

            let item = match value {
                Some(value) => Item::KeyValue(key, value),
                None => Item::Key(key),
            };

            return Some(item)
        }
    }
}

impl<'l> Iterator for IniParser<'l> {
    type Item = Item<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}

#[derive(Debug, Clone)]
pub enum Item<'a> {
    Section(&'a str),
    Key(&'a str),
    KeyValue(&'a str, &'a str),
}
