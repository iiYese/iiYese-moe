use dioxus::prelude::*;
use crate::book::*;

pub fn bike_shed<'a>() -> BookProp<'a> {
    BookProp {
        title: "Bike Shed",
        description: 
            "This is a loosely organized collection of thoughts that revolve around \
            bikeshedding various different topics from programming, to philosophy and whatever \
            activates as many neurons in my brain as possible. They are only that. \
            My thoughts. And unless stated otherwise they are backed up by no research or study \
            other than that of my own.",
        chapters: vec![

        ]
    }
}
