use super::{Clim, MenuOption};
use std::fmt::{Display, Debug};
use std::rc::Rc;

#[derive(Clone)]
pub struct Builder<T>
where
    T: Display + Eq + Into<String> + Clone + Debug,
{
    pub options: Vec<MenuOption<T>>,
    pub title: String,
}

impl<T> Builder<T>
where
    T: Display + Eq + Into<String> + Clone + Debug,
{
    pub fn new(title: &str) -> Builder<T> {
        Builder {
            options: vec![],
            title: title.to_owned(),
        }
    }

    pub fn add(&mut self, option: MenuOption<T>) -> &mut Builder<T> {
        self.options.push(option);

        self
    }

    pub fn build(&self) -> Clim<T> {
        Clim::new(self.options.clone(), self.title.clone())
    }
}

#[derive(Clone)]
pub struct OptionBuilder<T>
where
    T: Display + Eq + Into<String> + Clone + Default + Debug,
{
    key: T,
    description: String,
    on_select: Rc<Fn()>,
    is_exit: bool,
}

impl<T> OptionBuilder<T>
where
    T: Display + Eq + Into<String> + Clone + Default + Debug,
{
    pub fn new() -> OptionBuilder<T> {
        OptionBuilder {
            key: T::default(),
            description: String::from(""),
            on_select: Rc::new(|| {}),
            is_exit: false,
        }
    }

    pub fn build(&self) -> MenuOption<T> {
        MenuOption::<T>::new(
            self.key.clone(),
            &self.description.clone(),
            self.on_select.clone(),
            self.is_exit,
        )
    }

    pub fn key<U: Into<T>>(&mut self, key: U) -> &mut Self {
        self.key = key.into();

        self
    }
    pub fn description<U: Into<String>>(&mut self, description: U) -> &mut Self {
        self.description = description.into();

        self
    }
    pub fn on_select<F>(&mut self, on_select: F) -> &mut Self
    where
        F: Fn() -> () + 'static,
    {
        self.on_select = Rc::new(on_select);

        self
    }
    pub fn exits(&mut self) -> &mut Self {
        self.is_exit = true;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_add() {
        let mut builder: Builder<String> = Builder::new("builder test");
        builder
            .add(
                OptionBuilder::new()
                    .key("1")
                    .description("foo bar baz")
                    .on_select(|| {
                        println!("yeeeee");
                    }).build(),
            ).build();

        assert_eq!(builder.title, "builder test");
        assert_eq!(builder.options[0].key, "1".to_string());
    }

}
