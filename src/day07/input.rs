use itertools::Itertools;
use tap::Pipe;

use super::{Case, GetInput};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Ref<T> = Rc<RefCell<T>>;

pub enum Entry {
    Dir(Ref<Directory>),
    File(File),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Default)]
pub struct Directory {
    pub _name: String,
    pub parent: Option<Ref<Directory>>,
    pub childern: HashMap<String, Entry>,
}

impl Directory {
    pub fn size(&self) -> usize {
        self.childern
            .iter()
            .map(|(_, entry)| match entry {
                Entry::Dir(dir) => dir.borrow().size(),
                Entry::File(File { size, .. }) => size.to_owned(),
            })
            .sum()
    }
}

//
impl GetInput for Case {
    type Input = Ref<Directory>;

    const NAME: &'static str = "day07";

    fn text_to_input(content: &str) -> Self::Input {
        let root: Ref<Directory> = Default::default();
        let mut cwd = Rc::clone(&root);

        content.lines().map(|l| l.split_whitespace().collect_vec()).for_each(|words| {
            match (words[0], words[1]) {
                ("$", "ls") => {}
                ("$", "cd") => match words[2]  {
                    "/" =>  cwd = root.clone(),
                    ".." =>   {
                        let Some(parent) = cwd.borrow().parent.as_ref().map(Rc::clone) else { unreachable!() };
                        cwd = parent.clone();
                    },

                    name =>  {
                        let cwd_borrow = cwd.borrow();
                        let newcwd = cwd_borrow.childern.get(name).map(|v| {
                            if let Entry::Dir(newcwd) = v { newcwd.clone() } else { unreachable!() }
                        }).expect("Cd into non-existing direcory");
                        drop(cwd_borrow);

                        cwd = newcwd;
                    }
                }
                ("dir", name) => cwd.borrow_mut().childern.insert(name.into(), Entry::Dir(Directory {
                        _name: name.into(),
                        parent: Some(cwd.clone()),
                        childern: Default::default(),
                    }
                    .pipe(RefCell::new)
                    .pipe(Rc::new)))
                    .pipe(|_| ()),

                (size, name) => {
                    let file = File {
                        name: name.into(),
                        size: size.parse().expect("Failed to parse size"),
                    };
                    cwd.borrow_mut().childern.insert(name.into(), Entry::File(file));
                }
            }
        });

        root
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let root = output.borrow();
    let childern = root
        .childern
        .iter()
        .map(|(name, _)| name)
        .sorted()
        .collect_vec();
    assert_eq!(childern, vec!["a", "b.txt", "c.dat", "d"]);
    let sum = root.size();

    assert_eq!(sum, 48381165);
}
