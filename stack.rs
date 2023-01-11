use std::option::Option;
use std::boxed::Box;

/// Implementation of a linked Stack (LIFO)
struct Stack<E: Copy>{
    top: Option<Box<Node<E>>>
}

/// Node structure having a value and the last node
struct Node<E: Copy>{
    prev: Option<Box<Node<E>>>,
    val: E
}


impl <E: Copy> Default for Stack<E>{
    /// Default Stack is an empty stack
    fn default() -> Stack<E>{
        Stack{ 
            top: Option::None
        }
    }
}

impl <E: Copy> Stack<E>{
    /// Pushes an element on top of the stack 
    fn push(&mut self, elem: E) {
        match self.top {
            Option::None => {
                let node: Node<E> = Node{prev:None, val:elem};
                self.top = Option::Some(Box::new(node));
            }
            Option::Some(_) => {
                let node: Node<E> = Node{prev:self.top.take(), val:elem};
                self.top = Option::Some(Box::new(node));
            }
        }
    }

    /// Returns the value on top of the stack, without removing it
    fn top(&self) -> Option<E> {
        match &self.top {
            Option::None => {
                None
            }
            Option::Some(prev_node) => {
                Some(prev_node.val)
            }
        }
    }

    /// Removes the last item on the stack and returns it
    fn pop(&mut self) -> Option<E> {
        match self.top.take() {
            Option::None => {
                None
            }
            Option::Some(prev_node) => {
                self.top = prev_node.prev;
                Some(prev_node.val)
            }
        }
    }
}

fn main() {
    //just some tests
    let mut stack: Stack<i32> = Stack::default();
    let num_array = [1,2,3,4,56,71];
    for i in num_array{
        stack.push(i)
    }
    loop {
        if stack.top().is_none() {
            break;
        }
        else{
            println!("{}", stack.pop().unwrap());
        }
    }
}
