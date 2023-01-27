use std::path::PathBuf;

#[derive(Debug)]
pub struct Node {
    pub parent: Option<usize>,
    pub data: Contents,
    pub size: usize,
}

#[derive(Debug)]
pub enum Contents {
    Dir { name: String, children: Vec<usize> },
    File { name: String },
}

impl Contents {
    pub fn new_dir(name: impl Into<String>) -> Self {
        Contents::Dir {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn new_file(name: impl Into<String>) -> Self {
        Contents::File { name: name.into() }
    }

    pub fn children(&self) -> &[usize] {
        if let Contents::Dir { children, .. } = self {
            children
        } else {
            unreachable!();
        }
    }

    pub fn push_child(&mut self, i: usize) {
        if let Contents::Dir { children, .. } = self {
            children.push(i)
        } else {
            unreachable!();
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Contents::Dir { name, .. } => name,
            Contents::File { name, .. } => name,
        }
    }

    pub fn cd(&self, name: &str, arena: &[Node]) -> usize {
        *self
            .children()
            .iter()
            .find(|i| arena[**i].data.name() == name)
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Filesystem {
    pub pointer: usize,
    pub arena: Vec<Node>,
}

impl Filesystem {
    pub fn load(file: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let data = std::fs::read_to_string(file.into())?;

        let root = Node {
            parent: None,
            data: Contents::Dir {
                name: "/".into(),
                children: Vec::new(),
            },
            size: 0,
        };

        let mut arena = vec![root];
        let mut pointer = 0;

        let mut lines = data.lines().peekable().into_iter();
        while let Some(line) = lines.next() {
            let mut words = line.split_whitespace();
            match words.next().unwrap() {
                "$" => match words.next().unwrap() {
                    "cd" => match words.next().unwrap() {
                        "/" => pointer = 0,
                        ".." => pointer = arena[pointer].parent.unwrap(),
                        name => pointer = arena[pointer].data.cd(name, &arena),
                    },
                    "ls" => {
                        while !matches!(
                            lines.peek().and_then(|l| l.split_whitespace().next()),
                            Some("$")
                        ) && lines.peek().is_some()
                        {
                            let line = lines.next().unwrap();
                            let mut words = line.split_whitespace();
                            let node = match words.next().unwrap() {
                                "dir" => Node {
                                    parent: Some(pointer),
                                    data: Contents::new_dir(words.next().unwrap()),
                                    size: 0,
                                },
                                size => Node {
                                    parent: Some(pointer),
                                    data: Contents::new_file(words.next().unwrap()),
                                    size: size.parse().unwrap(),
                                },
                            };
                            arena.push(node);
                            let index = arena.len() - 1;
                            arena[pointer].data.push_child(index);
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        let mut fs = Filesystem { arena, pointer };
        fs.calculate_size();
        Ok(fs)
    }

    pub fn calculate_size(&mut self) {
        Self::size(&mut self.arena, 0);
    }

    pub fn size(arena: &mut [Node], pointer: usize) -> usize {
        let children: Vec<_> = arena[pointer].data.children().iter().copied().collect();

        children
            .iter()
            .map(|child| match arena[*child].data {
                Contents::File { .. } => arena[*child].size,
                _ => {
                    let s = Self::size(arena, *child);
                    arena[*child].size = s;
                    s
                }
            })
            .sum()
    }

    pub fn dirs_up_to(&self, greater_than: usize) -> impl Iterator<Item = usize> + '_ {
        self.arena.iter().filter_map(move |item| {
            let size = item.size;
            if size <= greater_than && matches!(item.data, Contents::Dir { .. }) {
                Some(size)
            } else {
                None
            }
        })
    }
}
