struct MyModule {}

impl MyModule {
    fn abs(&self, n: i32) -> i32 {
        if n < 0 {
            -n
        } else {
            n
        }
    }

    fn factorial(&self, n: i32) -> i32 {
        fn go(n: i32, acc: i32) -> i32 {
            if n < 1 {
                acc
            } else {
                go(n - 1, n * acc)
            }
        }
        go(n, 1)
    }

    fn fib(&self, n: i32) -> i32 {
        let mut m = n;

        let mut left = 0;
        let mut right = 1;

        fn _calc(x: i32, y: i32) -> (i32, i32) {
            (y, x + y)
        }

        while m > 0 {
            let lr = _calc(left, right);
            left = lr.0;
            right = lr.1;

            m -= 1
        }

        left
    }

    fn is_sorted<A, F>(&self, _as: &Vec<A>, ordered: F) -> bool
        where
            F: Fn(&A, &A) -> bool,
    {
        // actually iterator.as_sorted_by
        _as.windows(2)
            .all(|a| ordered(&a[0], &a[1]))
    }
}

#[cfg(test)]
mod my_module_test {
    use crate::my_module::MyModule;

    #[test]
    fn abs() {
        let my_mod = MyModule {};
        assert_eq!(10, my_mod.abs(10));
        assert_eq!(10, my_mod.abs(-10));
    }

    #[test]
    fn factorial() {
        let my_mod = MyModule {};
        assert_eq!(1, my_mod.factorial(1));
        assert_eq!(24, my_mod.factorial(4));
        // panicked at attempt to multiply with overflow
        // assert_eq!(1, my_mod.factorial(1000));
    }

    #[test]
    fn fib() {
        let my_mod = MyModule {};
        assert_eq!(0, my_mod.fib(0));
        assert_eq!(13, my_mod.fib(7));
    }

    #[test]
    fn is_sorted_i32() {
        let my_mod = MyModule {};
        let sorted = vec![0, 1, 3, 5, 6];
        let unsorted = vec![0, 1, 3, 2, 4];

        fn ordered(x: &i32, y: &i32) -> bool {
            x < y
        }
        assert_eq!(true, my_mod.is_sorted(&sorted, ordered));
        assert_eq!(false, my_mod.is_sorted(&unsorted, ordered));
    }

    #[test]
    fn windows() {
        let i = [0, 1, 2, 3];
        let a = i.windows(2).all(
            |a| {
                println!("{:?}", a);
                true
            });
        assert_eq!(true, a)
    }
}
