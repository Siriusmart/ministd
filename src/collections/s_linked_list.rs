use crate::rc::Rc;

#[derive(Clone)]
pub struct SLinkedList<T> {
    first: Option<Rc<SLinkedListNode<T>>>,
}

struct SLinkedListNode<T> {
    next: Option<Rc<SLinkedListNode<T>>>,
    payload: T,
}

impl<T> SLinkedList<T> {
    pub fn new() -> Self {
        Self { first: None }
    }

    pub fn len(&self) -> usize {
        let mut node = &self.first;
        let mut len = 0;

        while let Some(next_node) = node {
            len += 1;
            node = &next_node.next;
        }

        len
    }

    pub fn cons(&self, payload: T) -> Self {
        Self {
            first: Some(Rc::new(SLinkedListNode {
                next: self.first.clone(),
                payload,
            })),
        }
    }

    pub fn decons(&self) -> Option<(&T, SLinkedList<T>)> {
        let first = self.first.as_ref()?;
        Some((
            &first.payload,
            Self {
                first: first.next.clone(),
            },
        ))
    }
}
