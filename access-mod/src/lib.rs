mod outermost {
    pub fn middle_func() {}

    fn middle_secret_func() {}

    pub mod inner {
        pub fn inner_func() {
            ::outermost::middle_secret_func();
        }
        pub fn secret_func() {}
    }
}

fn try_me() {
    outermost::middle_func();
    outermost::inner::inner_func();
    outermost::inner::secret_func();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
