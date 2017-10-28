use sodium::gc::Gc;
use sodium::gc::GcCtx;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[test]
pub fn gc_loop() {
    let count = Rc::new(RefCell::new(0));
    let mut gc_ctx = GcCtx::new();
    struct A {
        count: Weak<RefCell<i32>>,
        x: Cell<Option<Gc<A>>>
    }
    impl A {
        fn new(x: Option<Gc<A>>, count: &Rc<RefCell<i32>>) -> A {
            {
                let mut c = count.borrow_mut();
                *c = *c + 1;
            }
            A {
                count: Rc::downgrade(&count),
                x: Cell::new(x)
            }
        }
    }
    impl Drop for A {
        fn drop(&mut self) {
            let count = self.count.upgrade().unwrap();
            let mut c = count.borrow_mut();
            *c = *c - 1;
        }
    }
    {
        let mut a = gc_ctx.new_gc(A::new(None, &count));
        let mut b = gc_ctx.new_gc(A::new(Some(a.clone()), &count));
        b.add_child(&a);
        let mut c = gc_ctx.new_gc(A::new(Some(b.clone()), &count));
        c.add_child(&b);
        a.x.set(Some(c.clone()));
        a.add_child(&c);
    }
    assert_eq!(0, *count.borrow());
}