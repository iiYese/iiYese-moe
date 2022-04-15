use dioxus::prelude::*;
use crate::book::*;

pub fn readability<T>(cx: Scope<'_, T>) -> PageProp<'_> {
    PageProp {
        title: "Readability",
        children: cx.render(rsx!{
            p {
                "My main issue with how we think about code readability is that
                we tout practices blindly without ever thinking about what it is 
                we're actually trying to achieve."
            }
            p {
                r#""Readable" is far too vague to ever provide us with a clear aim."#
            }
            p {
                "So here is what I've thought about."
            }
            br{}
            br{}
            br{}

            Paragraph {
                title: "My rules",
                p {
                    "Good naming should be a given and is also another topic on its own.
                    I am going to assume its presence as it's not the fosus of this post."
                }
                blockquote {
                    "When reading through the code path of any unit of code 
                    cognitive load over time should be as flat as possible." 
                }
                figure {
                    class: "smol",
                    img { src: "/static/good.png" }
                    img { src: "/static/bad.png" }
                    ul {
                        li {  
                            r#"With "unit" being a collection of code that we can 
                            considered at once. For example a function or module."#
                        }
                        li {  
                            r#"Units can be and are made up of other units."#
                        }
                    }
                }
                blockquote {
                    "Cognitive load for the code path of any unit of code 
                    should be as low as possible for its intended audience."
                }
                figure {
                    class: "smol",
                    img { src: "/static/okaybetter.png" }
                }
            }
            Paragraph {
                title: "A problem",
                p {
                    r#"How I've defined a "unit" so far has a problem. Mainly that 
                    this definition alone is still a little ambiguous and unclear.
                    What does one think about when trying to avoid 
                    bad code partitioning decisions?"#
                }
                p{
                    "If you are an experienced programmer you most likely 
                    have viscerally experienced good unit partitioning in a codebase 
                    before. You do know what it looks like and can even produce it 
                    yourself. So how can we difine a rule or something to think about 
                    that can effectively and somewhat consistently 
                    produce good partitioning?"
                }
                p {
                    "Here are my further thoughts."
                }
            }
            Paragraph {
                title: "A solution",
                blockquote {
                    "Code should be partitioned with the intent to create assumptions
                    that you can comfortably make. These assumptions should help you 
                    think about the thing that you actually want to think about."
                }

                p {
                    "Now if you partition too much you are juggling assumptions.
                    When trying to think about the thing you're trying to think about
                    you are not." b { " You are thinking about your assmptions." }
                }
                p {
                    "When you partition too little you are not thinking about the thing
                    you want to think about. It's too granular. You are instead 
                    thinking about" b { " how the machine " } "or" 
                    b { " your tools " } "think about it."
                }
                br {}
                br {}

                p {
                    "If you are as pedantic as me you will have noticed that this is still
                    context dependent. After a lot of deliberation I've come to the conclusion 
                    that this should deliberately be by design."
                }
                p {
                    "It is an impossible task for me or anyone else trying to produce
                    ways of thinking about these topics to be exhaustive. Furthermore
                    is that something that should even be done? I clearly can't do it 
                    because I don't have all the variables one might encounter. 
                    Even trying to would incorporate some of them would introduce 
                    variables into this process that not everyone cares about."
                }
                p {
                    "Individual people on a case by case basis trying to employ this 
                    way of thinking do have these variables and they only the variables 
                    that" b { " they care about. " } "So I think continuing this tangent 
                    is a task better left to the reader."
                }
            }
            Paragraph {
                title: "Anothe problem",
                p {
                    r#"How does one define or select the appropriate "intended audience"? 
                    In contrary to the last point this is something that I think can be worked on.
                    I have not been able to come to something I am happy with so far."#
                }
            }
        })
    }
}
