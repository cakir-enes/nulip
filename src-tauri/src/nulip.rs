use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::{
  cell::RefCell,
  collections::{BTreeMap, HashSet},
};
<<<<<<< HEAD

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
=======
>>>>>>> 6951630f9b7ba9a08c60b3d9375b34ac47d536c4

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
  id: String,
  created_at: DateTime<Utc>,
  content: String,
}

impl Block {
  fn new<'a>(content: &'a str) -> Self {
    Self {
      id: "keke".to_string(),
      created_at: Utc::now(),
      content: content.to_string(),
    }
  }
}

<<<<<<< HEAD
=======
#[derive(Debug, Deserialize, Serialize)]
pub struct Thread {
  id: String,
  created_at: DateTime<Utc>,
  name: String,
  tags: HashSet<String>,
  blocks: Vec<Block>,
}

impl Thread {}

// #[derive(Deserialize, Serialize)]
pub struct Stream {
  tags: HashSet<String>,
  created_at: DateTime<Utc>,
  threads: Vec<Rc<RefCell<Thread>>>,
}
// #[derive(Deserialize, Serialize)]
pub struct Nulip {
  streams: Vec<Stream>,
  threads: BTreeMap<String, Rc<RefCell<Thread>>>,
  path: String,
}

>>>>>>> 6951630f9b7ba9a08c60b3d9375b34ac47d536c4
struct AppendBlock<'a> {
  thread_id: &'a str,
  content: &'a str,
  tags: Vec<&'a str>,
}
struct EditTags<'a> {
  tags: Vec<&'a str>,
  thread_id: &'a str,
}

<<<<<<< HEAD
struct CreateThread<'a> {
  name: &'a str,
}
=======
>>>>>>> 6951630f9b7ba9a08c60b3d9375b34ac47d536c4
#[derive(Serialize, Deserialize)]
pub struct NewStream {
  pub tags: Vec<String>,
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
      let content =
        std::fs::read_to_string(std::path::Path::new(folder).join(".index")).unwrap_or_default();
      content
        .split("/n")
        .map(|tags| {
          tags
            .split(",")
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
    match toml::from_str(&str) {
      Ok(v) => Some(v),
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
      req
        .tags
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

  pub fn create_thread(&mut self, thread_name: &str, content: &str) -> String {
    let id = "asd";
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

    id.to_string()
  }

  pub fn append_block(&mut self, thread_id: &str, content: &str) -> Option<()> {
    let mut thread = self.threads.get_mut(thread_id)?.borrow_mut();
    let block = Block::new(content);
    thread.blocks.push(block);
    Some(())
  }

  pub fn edit_thread(&mut self, thread_id: &str, block_id: &str, new_content: &str) -> Option<()> {
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

<<<<<<< HEAD
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
=======
#[test]
fn check() {
  let mut tags = HashSet::new();
  tags.insert("tag1".to_string());
  tags.insert("tag2".to_string());
  // let mut t = Tags::new("tag1");
  // t.tags.insert("t2".to_string());
  let thread = Thread {
    blocks: vec![
      Block {
        id: "ID1".to_string(),
        content: "content this shta[[blocks]] babe".to_string(),
        created_at: Utc::now(),
      },
      Block {
        id: "ID2".to_string(),
        content: "asdasdcontent this shta[[blocks]] babe".to_string(),
        created_at: Utc::now(),
      },
    ],
    created_at: Utc::now(),
    id: "thread1".to_string(),
    name: "THREAD 1".to_string(),
    tags: tags,
  };
  let encoded = toml::to_string_pretty(&thread).unwrap();
  let decoded: Thread = toml::from_str(&encoded).unwrap();
  println!("{}", encoded);
  println!("REENCODED");
  let reencoded = toml::to_string(&decoded).unwrap();
  println!("{}", reencoded);
>>>>>>> 6951630f9b7ba9a08c60b3d9375b34ac47d536c4
}
