extern crate console;
extern crate failure;
pub mod inputs;

use console::Term;
use failure::Error;
use inputs::{Input, LineInput};
use std::convert::Into;
use std::fmt::Display;

pub struct Clim<T>
where
    T: Display + Eq,
{
    menu_options: Vec<MenuOption<T>>,
}

impl<T> Clim<T>
where
    T: Display + Eq,
{
    pub fn new<U: Into<Vec<MenuOption<T>>>>(menu_options: U) -> Clim<T> {
        Clim {
            menu_options: menu_options.into(),
        }
    }

    pub fn init(&self) -> Result<(), Error> {
        let term = Term::stderr();

        loop {

            for item in self.menu_options {
                let result = term.write_line(&format!("{} {}", menu_option.key, menu_option.description))?;
                let mut line = LineInput::new(term);
                line.get_from_terminal()?;
                break;
            }
        }

        Ok(())
    }
}

pub struct MenuOption<T>
where
    T: Display + Eq,
{
    key: T,
    description: String,
    on_select: Box<Fn()>,
    is_exit: bool,
}

impl<T> MenuOption<T>
where
    T: Display + Eq,
{
    fn new(key: T, description: String, on_select: Box<Fn()>, is_exit: bool) -> MenuOption<T> {
        MenuOption {
            key,
            description,
            on_select,
            is_exit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_menu_option() {
        let menu_option = MenuOption {
            key: "1".to_string(),
            description: "foo bar baz".to_string(),
            on_select: Box::new(|| {
                println!("yeeee");
            }),
            is_exit: false,
        };

        let clim = Clim::new(vec![(menu_option)]);

        (clim.menu_options.get(0).unwrap().on_select)();
    }

    #[test]
    fn clim_init() {
        let menu_option = vec![MenuOption {
            key: "1".to_string(),
            description: "foo bar baz".to_string(),
            on_select: Box::new(|| {
                println!("yeeee");
            }),
            is_exit: false,
        }];

        let clim = Clim::new(menu_option).init();

        println!("{:?}", clim.unwrap());
    }
}
