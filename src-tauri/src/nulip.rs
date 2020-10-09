use anyhow::{Context, Result};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
};

// #[derive(Deserialize, Serialize)]
pub struct Nulip {
    streams: Vec<Stream>,
    threads: BTreeMap<String, Rc<RefCell<Thread>>>,
    path: String,
}

pub struct Stream {
    tags: HashSet<String>,
    created_at: DateTime<Utc>,
    threads: Vec<Rc<RefCell<Thread>>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Thread {
    id: String,
    created_at: DateTime<Utc>,
    name: String,
    tags: HashSet<String>,
    blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    id: String,
    created_at: DateTime<Utc>,
    content: String,
}

impl Block {
    fn new<'a>(content: &'a str) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            created_at: Utc::now(),
            content: content.to_string(),
        }
    }

    pub fn tags(&self) -> Vec<String> {
        self.content
            .split_whitespace()
            .filter(|s| s.starts_with("#"))
            .map(|s| s.to_string())
            .collect()
    }
}

struct AppendBlock<'a> {
    thread_id: &'a str,
    content: &'a str,
    tags: Vec<&'a str>,
}
struct EditTags<'a> {
    tags: Vec<&'a str>,
    thread_id: &'a str,
}

struct CreateThread<'a> {
    name: &'a str,
}
#[derive(Serialize, Deserialize)]
pub struct NewStream {
    pub tags: Vec<String>,
}

impl<'a> Thread {
    pub fn dump(&self, path: &'a str) -> Result<()> {
        let encoded = toml::to_string(self)
            .with_context(|| format!("Failed encoding thread {}", self.name))?;
        let _ = std::fs::write(path, encoded)?;
        Ok(())
    }

    pub fn tag_with(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
    }
}

impl Nulip {
    pub fn new(folder: &str) -> Nulip {
        let paths = std::fs::read_dir(folder);
        if paths.is_err() {
            return Self {
                streams: vec![],
                path: folder.to_string(),
                threads: BTreeMap::new(),
            };
        }

        let threads: Vec<Rc<RefCell<Thread>>> = paths
            .unwrap()
            .filter_map(Result::ok)
            .map(|d| d.path())
            .map(|d| std::fs::read_to_string(d))
            .filter_map(Result::ok)
            .filter_map(|s| Nulip::to_thread(s))
            .map(|s| Rc::from(RefCell::from(s)))
            .collect();

        let mut streams: Vec<Stream> = (|| {
            let content = std::fs::read_to_string(std::path::Path::new(folder).join(".index"))
                .unwrap_or_default();
            content
                .split("/n")
                .map(|tags| {
                    tags.split(",")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .map(|tags| Stream::new(tags, vec![]))
                .collect()
        })();

        let mut map = BTreeMap::new();
        threads.iter().for_each(|tbox| {
            let t = tbox.borrow();
            map.insert(t.id.clone(), tbox.clone());
            for stream in streams.iter_mut() {
                if t.tags.is_subset(&stream.tags) {
                    stream.threads.push(tbox.clone())
                }
            }
        });

        Self {
            streams,
            threads: map,
            path: folder.to_string(),
        }
    }

    fn to_thread(str: String) -> Option<Thread> {
        println!("THREEAD ! {}", str);
        match toml::from_str(&str) {
            Ok(v) => {
                println!("THREEAD ! {:?}", &v);
                Some(v)
            }
            Err(_) => None,
        }
    }

    pub fn get_stream(&self, stream_id: &str) -> Option<&Stream> {
        self.streams.iter().find(|s| s.tags.is_empty())
    }

    pub fn threads<F: Fn(&Block)>(&self, stream_id: &str, f: F) {
        if let Some(s) = self.get_stream(stream_id) {
            s.threads
                .iter()
                .for_each(|t| t.borrow().blocks.iter().for_each(|b| f(b)));
        }
    }

    pub fn create_stream(&mut self, req: NewStream) {
        let tagged_with = |thread: &Thread| {
            req.tags
                .iter()
                .find(|&tag| thread.tags.contains(tag))
                .is_some()
        };

        let tagged_threads = self
            .threads
            .iter()
            .map(|thread_entry| thread_entry.1)
            .filter(|thread| tagged_with(&thread.borrow()))
            .map(|thread| thread.clone())
            .collect();

        let stream = Stream::new(req.tags, tagged_threads);

        self.streams.push(stream);
    }

    pub fn create_thread(&mut self, thread_name: &str, content: &str) -> Result<String> {
        let id = ulid::Ulid::new().to_string();
        let tags: HashSet<String> = content
            .split("\\s+")
            .filter(|w| w.starts_with("#"))
            .map(|t| t.to_string())
            .collect();
        let thread = Thread {
            name: thread_name.to_string(),
            created_at: Utc::now(),
            id: id.to_string(),
            blocks: vec![Block::new(content)],
            tags,
        };
        thread.dump(&format!("{}/{}.toml", self.path, thread_name));
        Ok(id.to_string())
    }

    pub fn append_block(&mut self, thread_id: &str, content: &str) -> Result<()> {
        if let Some(thr) = self.threads.get_mut(thread_id) {
            let mut thread = thr.borrow_mut();
            let block = Block::new(content);
            thread.tag_with(block.tags());
            thread.blocks.push(block);
            thread.dump(&format!("{}/{}.toml", self.path, thread.name))
        } else {
            Err(anyhow::Error::msg("Couldn't append block"))
        }
    }

    pub fn edit_thread(
        &mut self,
        thread_id: &str,
        block_id: &str,
        new_content: &str,
    ) -> Option<()> {
        let mut thread = self.threads.get_mut(thread_id)?.borrow_mut();
        let mut block = thread.blocks.iter_mut().find(|b| b.id == block_id)?;
        block.content = new_content.to_string();
        Some(())
    }
}

impl Stream {
    fn new(tags: Vec<String>, threads: Vec<Rc<RefCell<Thread>>>) -> Self {
        Self {
            created_at: Utc::now(),
            threads: threads,
            tags: tags.iter().map(|t| t.to_string()).collect(),
        }
    }
}

fn main() {
    use serde::{Deserialize, Serialize};
    use toml;

    struct Entry {
        foo: String,
        bar: String,
    }

    let toml_string = r#"
  [[entry]]
  foo = "a0"
  bar = "b0"

  [[entry]]
  foo = "a1"
  bar = "b1"
  "#;
    let v: toml::Value = toml::from_str(&toml_string).unwrap();
    for v in v.as_table() {
        // let x:  = v["asd"];
        println!("{:?}", v)
    }
}

// #[test]
// fn new_thr() {
//     let mut t = Nulip::new("/home/ec/tulip/src-tauri/notes");
//     match t.create_thread("This is a new thread", "Lorem ipsum #kek #mek") {
//         Ok(id) => println!("HERE ID {}", id),
//         Err(e) => println!("SHIT WENT WRONG{}", e),
//     }
// }

#[test]
fn append_thr() {
    // let mut t = Nulip::new("/home/ec/tulip/src-tauri/notes");
    // t.append_block(
    //     "01EM7ECV4435MDT0SH1Y7Y1ZS0",
    //     "block3 [[blocks]]  mmmyeah yeah beybii",
    // )
    // .unwrap()
    let b = Block::new(
        r"
kek meh   #tag1     #tag2

#tag3
",
    );
    println!("TAGS: {:?}", b.tags())
}
