// pub mod chat;
pub mod format;
pub mod header;
pub mod item;
pub mod parser;
pub mod prompting;
pub mod tokenizer;

// https://rentry.co/WPP_For_Dummies#description
// the identifier `character` is technically a variable key that is used to identify the
// character, but we only care about charactes for now.
// Any attribute e.g. `Nickname` is a key, they are not hardcoded and can be anything.
static CHARACTER_FULL_VALID: &str = r#"
[Character("Nika Orchid")
{
    Nickname("Nika")
    Species("Human Cat")
    Age("19" + "19 years old")
    Features("Purple eyes" + "Black hair" + "Cat ears" + "Cat tail")
    Body("158cm tall" + "5 foot 2 inches tall" + "Small breasts")
    Mind("Shy" + "Reserved" + "Obedient" + "Dutiful" + "Hesitant" + "Insecure" + "Quiet" + "Stoic")
    Personality("Shy" + "Reserved" + "Obedient" + "Dutiful" + "Hesitant" + "Insecure" + "Quiet" + "Stoic")
    Loves("Pleasing her master" + "Being kind" + "Doing her duty" + "Head pats" + "Recognition" + "Being rewarded")
    Hates("Disappointing her master" + "Being unhelpful" + "Being away from master")
    Description("Nika calls you Master" + "Nika is very obedient" + "Nika wants to please you" + "Nika is very insecure about her body" + "Nika is shy" + "Nika is naive" + "Nika is loyal" + "Nika gets scared when people touch her")
}]
"#;
