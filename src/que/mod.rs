// Crappy code, lets go

pub mod que {
    enum Link<T: ?Sized> {
        Value(Box<T>, Link<T>),
        Nil
    }

    pub struct Que<T: ?Sized> {
        front: Link<T>,
        back: Link<T>
    }

    impl<T: ?Sized> Que<T> {
        pub fn new() -> Que<T> {
            let front = Link::Nil;
            let back = Link::Nil;

            Que {
                front, 
                back
            }
        }

        pub fn add(&mut self, value: &mut T) {
            let mut new_link = Link::Value(value as *mut T, &mut Link::Nil);

            match self.back {
                Link::Nil => {
                    self.front = new_link;
                    self.back = new_link;
                },
                Link::Value(_, next) => {
                    next = new_link;
                }
            }
        }
    }

    struct Box<'a, T: ?Sized> {
        pointer: *mut T
    }

    impl<'a, T: ?Sized> Box<'a, T> {
        fn new(value: T) {
            Box 
        }

        fn deref(&mut self) -> &'a mut T {
            unsafe {
                &mut *(self.pointer);
            }
        }
    }
}