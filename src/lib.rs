extern crate console;
extern crate failure;

use std::fmt::Display;
use std::convert::Into;
use failure::Error;
use console::Term;

pub struct Clim<'a, T>
where T: Display + 'a
{
    menu_options: Vec<Box<MenuOption<'a, T>>>,
}

impl<'a, T> Clim<'a, T>
    where T: Display + 'a
{
    fn new(menu_options: Vec<Box<MenuOption<'a, T>>>, ) -> Clim<T> {
        Clim { menu_options }
    }

    fn init(&'a self) -> Result<(), Error> {
        let term = Term::stderr();

        loop {
            self.menu_options.iter()
                .for_each(|menu_option: &'a Box<MenuOption<'a, T>> | {
                    term.write_line(&format!("{} {}", menu_option.key, menu_option.description));
                });

            break;
        }

        Ok(())
    }
}

pub struct MenuOption<'a, T>
where T: Display + 'a
{
    key: T,
    description: String,
    on_select: Box<Fn()>,
    sub_menu: Option<&'a Clim<'a, T>>,
    is_exit: bool
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_menu_option() {
        let menu_option = MenuOption{ key: "1", description: "foo bar baz".to_string(),
            on_select: Box::new(|| {
                println!("yeeee");
        }),
            sub_menu: None,
            is_exit: false
        };

        let clim = Clim::new(vec![Box::new(menu_option)]);

        (clim.menu_options.get(0).unwrap().on_select)();
    }

    #[test]
    fn clim_init() {
        let menu_option: Vec<Box<MenuOption<String>>> = vec!(Box::new(MenuOption{
            key: "1".to_string(),
            description: "foo bar baz".to_string(),
            on_select: Box::new(|| {
                println!("yeeee");
            }),
            sub_menu: None,
            is_exit: false
        }),
            Box::new(MenuOption {
                key: "2".to_string(),
                description: "second option".to_string(),
                on_select: Box::new(|| {
                    println!("bar");
                }),
                sub_menu: None,
                is_exit: false
            }
            ));

                let clim = Clim::new(menu_option).init();

        println!("{:?}", clim.unwrap());
    }
}
