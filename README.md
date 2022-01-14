# simple_tui
A library that handles the displaying of widgets and input events using crossterm as backend,

This project is still in a very very early stage of developement, but when more Widgets are added I
am quite confident that it might be quite usefull.


## currently implemented Widgets
* Button with click event
* Container, that can contain any number of any widget including another container
* input
* text

## examples
for a simple example with an exit button and three input fields, that can be accessed by pressing tab or clicking at them with the mouse
check the `examples/text_editor` folder.

### accessing a Button wiget inside a Container
```rs
let container = Widgets::Container(Container::new(
    ContainerKind::Column,
    vec![
        // first widget inside container with index 0
        Widgets::Button(Button::new(
            /* fields of button */
        ))
        // second widget indside container with index 1
        // ...
    ],
    StyleSheet::default()
));
// change text of button
container.modify_element(
    0, // modify first element with index 0
    |mut widget| {
        // check if widget is button, when not just return original widget without changin it
        // when it is the button we searched for, change its text and return the `Button` in the form of `Widgets`
        match widget {
            Widgets::Text(mut btn) => {
                btn.text = String::from("clicked");
                Widgets::Text(t)
            },
            _ => widget
            // the value that gets returned in this block 
            // will be the widget in the first place of the contents field of the container
        }
    }
)

// change entire Widget
container.modify_element(
    0, // modify first element with index 0
    |_| {
        // returns text widget
        Widgets::Text(Text::new(
            /* fields */
        ))
    }
)
```
