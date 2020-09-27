use chrono::prelude;
use chrono::DateTime;
use chrono::Utc;
use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

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

pub struct Thread {
  id: String,
  created_at: DateTime<Utc>,
  name: String,
  blocks: Vec<Block>,
  tags: std::collections::HashSet<String>,
}

impl Thread {}

pub struct Stream {
  tags: HashSet<String>,
  created_at: DateTime<Utc>,
  threads: Vec<std::rc::Rc<RefCell<Thread>>>,
}
use std::cell::RefCell;

pub struct Nulip {
  streams: Vec<Stream>,
  threads: BTreeMap<String, Rc<RefCell<Thread>>>,
  path: String,
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
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct NewStream {
  pub tags: Vec<String>,
}

impl Nulip {
  pub fn new(folder: &str) -> Nulip {
    Self {
      streams: vec![],
      path: folder.to_string(),
      threads: BTreeMap::new(),
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

    let stream = Stream::new(&req.tags, tagged_threads);

    self.streams.push(stream);
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
  fn new(tags: &Vec<String>, threads: Vec<Rc<RefCell<Thread>>>) -> Self {
    Self {
      created_at: Utc::now(),
      threads: threads,
      tags: tags.iter().map(|t| t.to_string()).collect(),
    }
  }
}
