use itertools::Itertools;
use tap::Pipe;

use super::{Case, GetInput};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Ref<T> = Rc<RefCell<T>>;

pub enum Entry {
    Dir(Ref<Directory>),
    File(File),
}

#[derive(PartialEq, Eq, Clone)]
pub struct File {
    pub size: usize,
}

#[derive(Default)]
pub struct Directory {
    pub parent: Option<Ref<Directory>>,
    pub children: HashMap<String, Entry>,
}

impl GetInput for Case {
    type Input = Ref<Directory>;

    const NAME: &'static str = "day07";

    fn text_to_input(content: &str) -> Self::Input {
        let root: Ref<Directory> = Default::default();
        let mut cwd = Rc::clone(&root);

        content
            .lines()
            .map(|l| l.split_whitespace().collect_vec())
            .for_each(|words| match (words[0], words[1], words.get(2)) {
                ("$", "cd", Some(&"/")) => {
                    cwd = root.clone();
                }
                ("$", "cd", Some(&"..")) => {
                    let dir = cwd.borrow().clone_parent().unwrap();
                    cwd = dir;
                }
                ("$", "cd", Some(name)) => {
                    let dir = cwd
                        .borrow()
                        .children
                        .get(*name)
                        .and_then(Entry::to_dir)
                        .expect("Cd into non-existing direcory");

                    cwd = dir;
                }
                ("$", "ls", _) => {}
                ("dir", name, _) => cwd
                    .borrow_mut()
                    .children
                    .insert(name.into(), Entry::new_dir(&cwd))
                    .pipe(|_| ()),

                (size, name, _) => cwd
                    .borrow_mut()
                    .children
                    .insert(name.into(), Entry::new_file(size))
                    .pipe(|_| ()),
            });

        root
    }
}

impl Entry {
    fn new_dir(cwd: &Rc<RefCell<Directory>>) -> Entry {
        Entry::Dir(
            Directory {
                parent: Some(cwd.clone()),
                children: Default::default(),
            }
            .pipe(RefCell::new)
            .pipe(Rc::new),
        )
    }
    fn new_file(size: &str) -> Entry {
        Entry::File(File {
            size: size.parse().expect("Failed to parse size"),
        })
    }

    pub fn to_dir(&self) -> Option<Ref<Directory>> {
        if let Self::Dir(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }
}

impl Directory {
    pub fn size(&self) -> usize {
        self.children
            .iter()
            .map(|(_, entry)| match entry {
                Entry::Dir(dir) => dir.borrow().size(),
                Entry::File(File { size, .. }) => size.to_owned(),
            })
            .sum()
    }

    pub fn clone_parent(&self) -> Option<Ref<Directory>> {
        self.parent.as_ref().map(Clone::clone)
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dir(value) => value.clone().take().fmt(f),
            Self::File(value) => value.fmt(f),
        }
    }
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.size.pretty_fmt())
    }
}

impl std::fmt::Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("children", &self.children)
            .finish()
    }
}

pub trait PrettyFormat<T> {
    fn pretty_fmt(&self) -> String;
}

impl PrettyFormat<usize> for usize {
    fn pretty_fmt(&self) -> String {
        let mut s = String::new();
        let i_str = self.to_string();
        let a = i_str.chars().rev().enumerate();
        for (idx, val) in a {
            if idx != 0 && idx % 3 == 0 {
                s.insert(0, ',');
            }
            s.insert(0, val);
        }
        s
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let root = output.borrow();
    let children = root
        .children
        .iter()
        .map(|(name, _)| name)
        .sorted()
        .collect_vec();
    assert_eq!(children, vec!["a", "b.txt", "c.dat", "d"]);
    let sum = root.size();

    assert_eq!(sum, 48381165);
}
