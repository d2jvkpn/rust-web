pub fn update_option_field<T>(a: &mut Option<T>, b: &mut Option<T>) -> bool {
    if b.is_none() {
        return false;
    }
    *a = b.take();
    true
}

pub fn update_from_option<T>(a: &mut T, b: &mut Option<T>) -> bool {
    if b.is_none() {
        return false;
    }
    *a = b.take().unwrap();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_update_option_field() {
        let mut a = Some("A");
        let mut b = Some("B");

        update_option_field(&mut a, &mut b);
        assert_eq!(a, Some("B"));
        assert_eq!(b, None);

        let mut a = Some("A");
        let mut b = None;
        update_option_field(&mut a, &mut b);
        assert_eq!(a, Some("A"));
        assert_eq!(b, None);

        let mut a = None;
        let mut b = Some("B");
        update_option_field(&mut a, &mut b);
        assert_eq!(a, Some("B"));
        assert_eq!(b, None);
    }
}
