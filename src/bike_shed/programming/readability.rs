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
                    context dependent. This is by design."
                }
                p {
                    "After a lot of deliberation I've come to the conclusion that
                    good definitions for topics like this should be well defined enough
                    as to allow people to communicate more effectively but should still
                    selectively allow for a necessary drgree of interpretation."
                }
                p {
                    "It is an impossible task for me or anyone else trying to produce
                    ways of thinking about these topics to be exhaustive. Furthermore
                    is that something that should even be done? I clearly can't do it 
                    because I don't have all the variables and even trying to would 
                    introduce variables that not everyone cares about."
                }
                p {
                    "People do have these variabels and they are the readers. 
                    The people who might try employ this way of thinking. Each reader
                    also has only the variables they care about."
                }
            }
            Paragraph {
                title: "Anothe problem",
                p {
                    r#"How does one define or select an "intended audience"?
                    This is something I have not been able to answer and am still
                    unhappy about."#
                }
            }
        })
    }
}
