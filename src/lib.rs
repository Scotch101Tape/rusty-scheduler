#![no_std]

struct Task {

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn example_of_api() {
        let scheuler = Scheduler::new();

        scheuler.defer(Task::new_task(|| {

        }));

        scheuler.defer(Task::new_loop(|| {

        }));

        scheuler.run();
    }
}
