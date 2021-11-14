#![no_std]

/*
mod que;

struct Task<'a> (
    &'a dyn FnOnce()
);

enum TaskLink {
    Link(*mut TaskLink),
    Nil
}

struct TaskQue {

}

impl TaskQue {
    fn new() -> TaskQue {
        TaskQue {

        }
    }
}

struct Scheduler {
    que: TaskQue,
}

impl Scheduler {
    fn new() -> Scheduler {
        let que = TaskQue::new();

        Scheduler {
            que
        }
    }

    fn defer<T>(&mut self, f: T) 
        where T: FnOnce()
    {
        self.que.add(f);
    }

    fn defer(&mut self, f: Task) {

    }

    fn repeat<T>(&mut self, f: &mut T)
        where T:  FnMut(&mut bool)
    {
        let mut canceler = false;

        f(&mut canceler);

        if !canceler {
            self.defer(Task::new())
        }
    }

    fn cancel(canceler: &mut bool) {
        *canceler = true;
    }

    fn create_repeat_task() {

    }
}

*/

mod task_que {
    use core::iter::Take;


    /*
    try 1 used a linked list and I could not get it to work without standerd... ;(

    enum Task {
        Defer(&'static dyn FnOnce()),
        Repeat(&'static dyn FnMut())
    }

    struct LinkBody {
        task: Task,
        next: *mut Link
    }
    
    impl LinkBody {
        fn task(self) -> Task {
            self.task
        }
    }

    enum Link {
        Nil,
        Link(LinkBody)
    }

    pub struct TaskQue {
        front: Link,
        back: Link
    }

    impl TaskQue {
        pub fn new() -> TaskQue {
            let front = Link::Nil;
            let back = Link::Nil; 

            TaskQue {
                front,
                back
            }
        }

        pub fn add() {

        }

        pub fn pop(&mut self) -> Option<Task> {
            let front = unsafe {
                self.front
            };
            

            match front {
                Link::Nil => None,
                Link::Link(linkBody) => {
                    // self.front = linkBody.next;

                    Some(linkBody.task())
                }
            }
        }
    }*/


    // Try 2

    //#[derive(Clone)]
    pub enum Task {
        Defer(&'static mut dyn FnOnce()),
        Repeat(&'static mut dyn FnMut()),
    }

    pub struct TaskQue<const MAX_TASKS: usize> {
        start_index: usize,
        end_index: usize,
        list: [Option<Task>; MAX_TASKS],
    }

    impl<const MAX_TASKS: usize> TaskQue<MAX_TASKS> {
        pub fn new() -> TaskQue<MAX_TASKS> {
            let start_index = 0;
            let end_index = 0;
            let list: [_; MAX_TASKS] = [None; MAX_TASKS];

            TaskQue { 
                start_index,
                end_index,
                list
            }
        }

        pub fn add(&mut self, task: Task) {
            assert!((self.start_index - self.end_index) % MAX_TASKS != 1, "TaskQue full");

            self.end_index += 1;

            self.list[self.end_index] = Some(task);
        }

        pub fn pop(&mut self) -> Option<Task> {
            if self.start_index == self.end_index {
                None
            } else {
                let pop_index = self.start_index;
                self.start_index += 1;

                let pop_value = self.list[pop_index];

                self.list[pop_index] = None;

                pop_value
            }
        }
    }
}

use task_que::{TaskQue, Task};

pub struct Scheduler<const MAX_TASKS: usize> {
    task_que: TaskQue<MAX_TASKS>
}

impl<const MAX_TASKS: usize> Scheduler<MAX_TASKS> {
    pub fn new() -> Scheduler<MAX_TASKS> {
        let task_que = TaskQue::new();

        Scheduler {
            task_que
        }
    }

    fn defer<T: FnOnce()>(&mut self, f: T) {

    }

    fn run(&mut self) {
        while let Some(Task) = self.task_que.pop() {
            match Task {
                Task::Defer(f) => {
                    f();
                },
                Task::Repeat(f) => {
                    self.task_que.add(Task::Repeat(f));

                    f();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    use crate::{Scheduler};
    #[test]
    fn example_of_api() {
        let mut schedular = Scheduler::new();

        let x = 1;
        schedular.defer(move || {
            let y = 2;
            assert!(x + y == 3);
        });

        schedular.run();
    }
}
