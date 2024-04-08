use once_cell::sync::Lazy;
use quick_xml::{events::Event, Reader};

#[derive(Debug)]
pub struct Sura {
    pub name: String,
    pub bismillah: Option<String>,
    pub aya: Vec<String>,
}

#[derive(Debug)]
pub struct Quran {
    pub sura: Vec<Sura>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranSubset(Vec<Vec<usize>>);

impl QuranSubset {
    pub fn sura_iter<'a>(&'a self) -> impl Iterator<Item = (usize, &'static Sura, &'a [usize])> {
        self.0
            .iter()
            .enumerate()
            .zip(QURAN.sura.iter())
            .filter(|x| !x.0 .1.is_empty())
            .map(|x| (x.0 .0 + 1, x.1, &**x.0 .1))
    }
}

impl Quran {
    fn load() -> Self {
        let mut r = Self { sura: vec![] };
        let text = include_str!("../quran-simple.xml");
        let mut reader = Reader::from_str(text);
        reader.trim_text(true);

        let mut txt = Vec::new();
        let mut buf = Vec::new();
        let mut current_sura = &mut Sura {
            name: "invalid".to_owned(),
            bismillah: None,
            aya: vec![],
        };

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(e) | Event::Empty(e)) => match e.name().as_ref() {
                    b"sura" => {
                        r.sura.push(Sura {
                            name: attr(&e, "name"),
                            bismillah: None,
                            aya: vec![],
                        });
                        current_sura = r.sura.last_mut().unwrap();
                    }
                    b"aya" => {
                        if let Some(bismillah) = attr_opt(&e, "bismillah") {
                            current_sura.bismillah = Some(bismillah);
                        }
                        current_sura.aya.push(attr(&e, "text"))
                    }
                    _ => (),
                },
                Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        r
    }

    pub fn get_aya(&self, sura: usize, aya: usize) -> Option<&str> {
        let sura = sura.checked_sub(1)?;
        let aya = aya.checked_sub(1)?;
        let r = self.sura.get(sura)?.aya.get(aya)?;
        Some(&*r)
    }

    pub fn filter(&self, f: impl Fn(usize, usize, &str) -> bool) -> QuranSubset {
        let mut r = QuranSubset(vec![]);
        for (sura_index, sura) in self.sura.iter().enumerate() {
            let mut s = vec![];
            for (aya_index, aya) in sura.aya.iter().enumerate() {
                if f(sura_index + 1, aya_index + 1, &aya) {
                    s.push(aya_index);
                }
            }
            r.0.push(s);
        }
        r
    }
}

fn attr_opt(e: &quick_xml::events::BytesStart<'_>, qname: &str) -> Option<String> {
    String::from_utf8(
        e.attributes()
            .find(|a| a.as_ref().unwrap().key.into_inner() == qname.as_bytes())?
            .unwrap()
            .value
            .to_vec(),
    )
    .ok()
}

fn attr(e: &quick_xml::events::BytesStart<'_>, qname: &str) -> String {
    attr_opt(e, qname).unwrap()
}

pub static QURAN: Lazy<Quran> = Lazy::new(Quran::load);
