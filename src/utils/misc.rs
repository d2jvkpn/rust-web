use std::{io, thread};

#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let caller = std::panic::Location::caller();
        let name = type_name_of(f);
        let list: Vec<&str> = name.split("::").collect();
        // println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" { length - 3 } else { length - 2 };

        format!("{}:{}:{}", caller.file(), caller.line(), list[idx]).as_str()
    }};
}

pub fn number_of_threads() -> io::Result<usize> {
    Ok(thread::available_parallelism()?.get())
}

// return (cpus, threads)
pub fn number_of_cpus() -> (usize, usize) {
    (num_cpus::get_physical(), num_cpus::get())
}

pub fn update_option<T: std::cmp::PartialEq>(a: &mut Option<T>, b: &mut Option<T>) -> bool {
    //if b.is_none() {
    //    return false;
    //}
    //*a = b.take();
    //true

    let val = match b {
        None => return false,
        Some(v) => v,
    };

    if let Some(v) = a {
        if v == val {
            return false;
        }
    };

    *a = b.take();
    true
}

pub fn update_value<T: std::cmp::PartialEq>(a: &mut T, b: &mut Option<T>) -> bool {
    //if b.is_none() {
    //    return false;
    //}
    //*a = b.take().unwrap();
    // true

    let val = match b {
        None => return false,
        Some(v) => v,
    };

    if a == val {
        return false;
    }

    *a = b.take().unwrap();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_number_of_cpus() {
        let threads = number_of_threads().unwrap();
        let cpus = number_of_cpus();

        println!("cpus: {cpus:?}");
        assert_eq!(threads, cpus.1);
    }

    #[test]
    fn t_update_option() {
        let mut a = Some("A");
        let mut b = Some("B");

        update_option(&mut a, &mut b);
        assert_eq!(a, Some("B"));
        assert_eq!(b, None);

        let mut a = Some("A");
        let mut b = None;
        update_option(&mut a, &mut b);
        assert_eq!(a, Some("A"));
        assert_eq!(b, None);

        let mut a = None;
        let mut b = Some("B");
        update_option(&mut a, &mut b);
        assert_eq!(a, Some("B"));
        assert_eq!(b, None);
    }
}
