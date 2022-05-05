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
        _as.windows(2).all(|a| ordered(&a[0], &a[1]))
    }

    fn closure<T, R, F>(&self, value: T, double: F) -> R
    where
        F: Fn(T) -> R,
    {
        double(value)
    }

    fn return_closure(&self) -> fn(i32) -> i32 {
        |x: i32| return x * 2
    }

    fn return_closure_string(&self) -> fn(&str) -> String {
        |s: &str| return format!("{}{}", s, s)
    }

    fn compose<A, B, C, F, G>(&self, f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C,
        G: Fn(A) -> B,
    {
        move |a: A| f(g(a))
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
        let a = i.windows(2).all(|a| {
            println!("{:?}", a);
            true
        });
        assert_eq!(true, a)
    }

    #[test]
    fn closure() {
        let my_mod = MyModule {};
        assert_eq!(20, my_mod.closure(10, |x: i32| { return x * 2 }));
        assert_eq!(
            "strstr",
            my_mod.closure("str", |x: &str| { return format!("{}{}", x, x) })
        )
    }

    #[test]
    fn return_closure() {
        let my_mod = MyModule {};
        let c = my_mod.return_closure();
        assert_eq!(2, c(1));
        assert_eq!(4, c(2));
    }

    #[test]
    fn return_closure_string() {
        let my_mod = MyModule {};
        let c = my_mod.return_closure_string();
        assert_eq!("strstr", c("str"));
    }

    #[test]
    fn compose() {
        let my_mod = MyModule {};
        let double = |i: i32| i * 2;
        let triple = |i: i32| i * 3;
        assert_eq!(12, my_mod.compose(double, triple)(2));

        let str_double = |s: &str| format!("{}{}", s, s);
        let trim: fn(String) -> String = |s: String| {
            let len = s.len();
            if len < 3 {
                String::from("")
            } else {
                s[1..(len - 1)].to_string()
            }
        };
        assert_eq!("", my_mod.compose(trim, str_double)(""));
        assert_eq!("", my_mod.compose(trim, str_double)("a"));
        assert_eq!("oofo", my_mod.compose(trim, str_double)("foo"))
    }
}
