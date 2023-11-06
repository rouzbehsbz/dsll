use std::{fmt::Debug, sync::{Arc, Mutex}};

type SafeNode<T> = Arc<Mutex<Option<Node<T>>>>;

#[derive(Clone, Debug)]
pub struct Node<T>
where T: Ord + Clone + Debug + Default
{
    pub value: T,
    pub next: SafeNode<T>,
    pub prev: SafeNode<T>,
    pub is_helper: bool
}

impl <T> Node<T>
where T: Ord + Clone + Debug + Default
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: Arc::new(Mutex::new(None)),
            prev: Arc::new(Mutex::new(None)),
            is_helper: false
        }
    }

    pub fn new_helper_node() -> Self {
        Self {
            value: T::default(),
            next: Arc::new(Mutex::new(None)),
            prev: Arc::new(Mutex::new(None)),
            is_helper: true
        }
    }
}

pub struct DoublySortedLinkedList<T>
where T: Ord + Clone + Debug + Default
{
    pub head: SafeNode<T>,
    pub tail: SafeNode<T>,
}

impl <T> DoublySortedLinkedList<T>
where T: Ord + Clone + Debug + Default
{
    pub fn new() -> Self {
        let helper_node = Node::<T>::new_helper_node();

        let head = Arc::new(Mutex::new(Some(helper_node.clone())));
        let tail = Arc::new(Mutex::new(Some(helper_node.clone())));

        head.lock().unwrap().as_mut().unwrap().next = tail.clone();
        tail.lock().unwrap().as_mut().unwrap().prev = head.clone();

        Self {
            head,
            tail
        }
    }

    pub fn insert(&self, value: T) {
        let mut current_node = self.head.clone();

        loop {
            let mut guarded_current_node = current_node.lock().unwrap();
            let next_node = guarded_current_node.as_ref().unwrap().next.clone();
            let mut guarded_next_node = next_node.lock().unwrap();

            if guarded_next_node.as_ref().unwrap().is_helper || value < guarded_next_node.as_ref().unwrap().value {
                let mut new_node = Node::new(value.clone());

                new_node.next = next_node.clone();
                new_node.prev = current_node.clone();

                let safe_new_node = Arc::new(Mutex::new(Some(new_node)));

                guarded_next_node.as_mut().unwrap().prev = safe_new_node.clone();
                guarded_current_node.as_mut().unwrap().next = safe_new_node.clone();

                break;
            } else {
                drop(guarded_current_node);

                current_node = next_node.clone();
            }
        }
    }

    pub fn print(&self) {
        let mut current_node = self.head.clone();

        loop {
            let guarded_current_node = current_node.lock().unwrap();

            if guarded_current_node.is_none() {
                break;
            }

            if !guarded_current_node.as_ref().unwrap().is_helper {
                println!("{:?}", guarded_current_node.as_ref().unwrap().value);
            }

            let next_node = guarded_current_node.as_ref().unwrap().next.clone();

            drop(guarded_current_node);
    
            current_node = next_node;   
        }
    }

    pub fn is_sorted(&self) -> bool {
        let mut current_node = self.head.clone();
        let mut prev_node_value: Option<T> = None;
        let mut is_sorted = true;

        loop {
            let guarded_current_node = current_node.lock().unwrap();

            if guarded_current_node.is_none() {
                break;
            }

            if !guarded_current_node.as_ref().unwrap().is_helper {
                match prev_node_value {
                    Some(prev_node_value) => {
                        if prev_node_value > guarded_current_node.as_ref().unwrap().value {
                            is_sorted = false;
                            
                            break;
                        }
                    },
                    None => {}
                }

                prev_node_value = Some(guarded_current_node.as_ref().unwrap().value.clone());
            }

            let next_node = guarded_current_node.as_ref().unwrap().next.clone();

            drop(guarded_current_node);
    
            current_node = next_node;   
        }

        is_sorted
    }
}