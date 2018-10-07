use super::{Clim, MenuOption };
use std::fmt::Display;

pub struct Builder<T>
where
    T: Display + Eq + Into<String> + Clone
{
    pub options: Vec<MenuOption<T>>,
    pub title: String
}

impl<T> Builder<T>
    where
        T: Display + Eq + Into<String> + Clone
{
    pub fn new(title: &str) -> Builder<T> {
        Builder { options: vec![], title: title.to_owned() }
    }

    pub fn add(&mut self, option: MenuOption<T>) -> &mut Builder<T> {
        self.options.push(option);

        self
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn builder_add() {
        let option = MenuOption {
            key: "1".to_string(),
            description: "foo bar baz".to_string(),
            on_select: Box::new(|| {
                println!("yeeee");
            }),
            is_exit: false,
        };
        let mut builder = Builder::new("builder test");

        builder.add(option);

        assert_eq!(builder.title, "builder test");
        assert_eq!(builder.options[0].key, "1".to_string());


    }

}
