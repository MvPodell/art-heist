
struct Challenge {
    name: String, // E.g. "Antechamber"
    desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    choices: Vec<Choice>
}

struct Choice {
    target: ChallengeID, // More about this in a minute
    triggers:Vec<String>, // e.g. "go north", "north"
    message: Option<String> // What message, if any, to print when the doorway is traversed
    // Any other info about the door would go here
}
// To add ASCII keys, add it to the desc prop at the beginning of the string followed by two newline characters (\n\n)
let challenges = [
    Challenge {
        name: "pickTools".into(), // Turn a &'static string (string constant) into a String
        desc: "It's time to pick your tools".into(),
        choices: vec![Choice{target:ChallengeID(1), triggers:vec!["1".into(), "2".into(), "3".into()], message:Some("Interesting choice...")}]
    },
    Challenge {
        name: "entrance".into(),
        desc: "How do you plan to enter the museum?".into(),
        choices: vec![
            Choice{target:ChallengeID(2), triggers:vec!["door".into()], message:Some("You slip through the door and find yourself in a dark basement.".into())},
            Choice{target:ChallengeID(3), triggers:vec!["vent".into()], message:Some("You shimmy through an exterior vent and crawl through the ducts.".into())},
            Choice{target:ChallengeID(4), triggers:vec!["window".into()], message:Some("You climb through the window and find yourself in a dark storage room.".into())}
        ]
    },
    Challenge {
        name: "You're stuck!".into(),
        desc: "\n\nBAD END".into(),
        choices:vec![]
    },
    Challenge {
        name: "The Vault".into(),
        desc: "When you regain consciousness, you feel a stabbing sensation in your lower back.  Reaching beneath you, you discover a massive diamond!  This room is full of gold and jewels, and a convenient ladder leading back outdoors!\n\nYou win!".into(),
        choices:vec![]
    }
];