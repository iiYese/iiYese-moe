mod design;
mod philosophy;
mod programming;

use dioxus::prelude::*;
use crate::book::*;
use programming::*;

pub fn bike_shed<T>(cx: Scope<'_, T>) -> Book<'_> {
    Book {
        title: "Bike Shed",
        description: 
            "This is a loosely organized collection of thoughts that revolve around
            bikeshedding various different topics from programming, to philosophy and whatever
            activates as many neurons in my brain as possible. They are only that.
            My thoughts. They may not even by my opinions. I might have changed my mind
            or had new thoughts. I have no commitment to what I have written here. 
            These were just things I was thinking about and thought were 
            worth writing down at that time. Unless stated otherwise things here 
            are backed up by no research or study other than that of my own.",
        chapters: vec![
            Chapter {
                title: "Programming",
                description: "This is where you can find most of the paint.",
                pages: vec![
                    readability(cx)
                ]
            }
        ]
    }
}
