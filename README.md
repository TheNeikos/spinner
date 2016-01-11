_Spinner_, a simple library to create interactive terminal applications
=======================================================================

To use Spinner simply add it to your `Cargo.toml`

```toml
[dependencies.spinner]
version = "0.2"
```

Usage
-----

There are two parts to Spinner, one part is the spinner itself, the other a
simple interface to build menus.


### Spinners

To use a Spinner simply go and create one using the SpinnerBuilder:
```rust
use spinner::SpinnerBuilder;
let sp = SpinnerBuilder::new("Long Running operation, please wait...".into()).start();
```

Will inform the user that your app currently is doing background processing.
`sp` is a `SpinnerHandle`, through which you can tell the user for example how
far along the process you are, or perhaps a message in between.

```
use spinner::SpinnerBuilder;
let sp = SpinnerBuilder::new("Long Running operation, please wait...".into()).start();
sp.message("Updating...".into());
sp.update(format!("Finished {} out of {}.", i, max));
```

#### Customizing

A spinner can be customized in three ways:

- The `step` duration, which is the 'refresh' period of the message.
- The `format`, how a given string is printed, due to limitations this is
    done through a closure, but it also allows more special formatting than
    just a format string.
- The `spinner` itself, which is the list of characters that change every
    step.


### Menus

Menus are simple, type checked ways to ask the user for information.

A simple menu might look like this:

```rust
use spinner::menu::*;
let m = Menu::new(vec![
    MenuOption("First Name".into(), MenuType::Text(false), None),
    MenuOption("Last Name".into(), MenuType::Text(true), None),
    MenuOption("Age".into(), MenuType::Integer(true), Some(MenuValue::Integer(1))),
    MenuOption("How much Ketchup?".into(), MenuType::Float(true), None),
]);

let results = m.display();
```

In results will then be an array of `MenuValue`, which can then be retrieved by
either `get_{string,int,float}`, calling one of these on the wrong type will
**panic!**. So be careful to take out the correct value out of the correct menu
questions.

#### MenuOption

A MenuOption is a NewType. It consists of a string which will constitute the
question being presented to the user. Then a MenuType, telling the checker what
input is expected. If you need something else use `MenuType::Text` and work with
that. You also have to tell whether that input is optional or not.
(true=optional, false=not optional). At last, an `Option<MenuValue>` which allows
you to put in either `None`, for no default value or `Some<MenuValue>` which
will be used if the user inputs nothing.
