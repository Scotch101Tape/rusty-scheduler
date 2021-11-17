use std::cell::RefCell;
use std::collections::VecDeque;

enum Task {
    Repeat(Box<dyn FnMut()>),
    Defer(Box<dyn FnOnce()>)
}

struct TaskQue (VecDeque<Task>);

impl TaskQue {
    fn new() -> TaskQue {
        TaskQue(VecDeque::new())
    }

    fn add_task(&mut self, task: Task) {
        self.0.push_back(task);
    }

    fn pop_task(&mut self) -> Option<Task> {
        self.0.pop_front()
    }
}

pub struct Scheduler {
    que: RefCell<TaskQue>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let que = RefCell::new(TaskQue::new());

        Scheduler {
            que
        }
    }

    pub fn defer<T>(&self, f: T)
    where T: 'static + FnOnce() {
        let mut que = self.que.borrow_mut();

        (*que).add_task(Task::Defer(
            Box::new(f)
        ))
    }

    pub fn repeat<T>(&self, f: T)
    where T: 'static + FnMut() {
        let mut que = self.que.borrow_mut();

        (*que).add_task(Task::Repeat(
            Box::new(f)
        ))
    }

    pub fn run(&self) {
        loop {
            let mut que = self.que.borrow_mut();
            let task = (*que).pop_task();

            drop(que);

            if let Some(task) = task {
                match task {
                    Task::Repeat(mut f) => {
                        f();

                        let mut que = self.que.borrow_mut();
                        (*que).add_task(Task::Repeat(f));
                    },
                    Task::Defer(f) => {                        
                        f();
                    }
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Scheduler};
    #[test]
    fn example_of_api() {
        let mut schedular = Scheduler::new();

        let x = 1;
        schedular.defer(move || {
            let y = 2;
            assert_eq!(x + y, 3);
        });

        schedular.defer(|| {
            assert_eq!(1, 1);
        });

        schedular.run();
    }
}
