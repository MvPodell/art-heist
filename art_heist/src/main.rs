struct Challenge {
    name: String, // E.g. "Antechamber"
    desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    paths: Vec<Path>
}
struct Path {
    target: ChallengeID, // More about this in a minute
    triggers:Vec<String>, // e.g. "go north", "north"
    message: Option<String> // What message, if any, to print when the doorway is traversed
    // Any other info about the door would go here
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct RoomID(usize);

fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
        
    let challenges = [
        // ChallengeID corresponds to the index inside list challenges that you want
        Challenge { // 0
            name: "pickTools".into(), // Turn a &'static string (string constant) into a String
            desc: "It's time to pick your tools".into(),
            paths: vec![Choice{target:ChallengeID(1), triggers:vec!["1".into(), "2".into(), "3".into()], message:Some("Interesting choice...")}]
        },
        Challenge { //1
            name: "entrance".into(),
            desc: "How do you plan to enter the museum? Through the door, vent, or window?".into(),
            paths: vec![
                Path{target:ChallengeID(2), triggers:vec!["door".into()], message:Some("You slip through the door and find yourself in a dark basement.".into())},
                Path{target:ChallengeID(2), triggers:vec!["vent".into()], message:Some("You shimmy through an exterior vent and crawl through the ducts.".into())},
                Path{target:ChallengeID(2), triggers:vec!["window".into()], message:Some("You climb through the window and find yourself in a dark storage room.".into())}
            ]
        },
        Challenge { // 2
            name: "You're stuck!".into(),
            desc: "Oh no, you are locked in...do you eat a snack, pick the lock, use the flashlight?".into(),
            // resource checking
            paths:vec![
                Path{target:ChallengeID(3), triggers:vec!["Banana".into()], message:Some("You eat a banana, but now you smell like banana".into())},
                Path{target:ChallengeID(3), triggers:vec!["Lock".into()], message:Some("Pick the lock".into())},
                Path{target:ChallengeID(3), triggers:vec!["Light".into()], message:Some("You turn on the flashlight and discover a secret door".into())},
            ]
        },
        Challenge { // 3
            name: "Footsteps".into(),
            desc: "You exit into a hallway but, you hear footsteps coming down. Do you run blindly away, hide behind a statue, or run towards the sound?".into(),
            paths:vec![
                Path{target:ChallengeID(4), triggers:vec!["Run blindly away".into()], message:Some("That's lame. End Game".into())},
                Path{target:ChallengeID(4), triggers:vec!["Hide".into()], message:Some("The security guard stops next to you...you need a distraction. TBD".into())},
                Path{target:ChallengeID(4), triggers:vec!["Run towards".into()], message:Some("You approach the security guard...what now? TBD".into())},
            ]
        },
        Challenge { // 4
            name: "Map".into(),
            desc: "You successfully evaded the guard. Where do you go next, the tourist shop, security office, or cafe?".into(),
            paths:vec![
                Path{target:ChallengeID(5), triggers:vec!["Tourist shop".into()], message:Some("You enter the gift shop...".into())},
                Path{target:ChallengeID(5), triggers:vec!["Security office".into()], message:Some("You enter the security office...".into())},
                Path{target:ChallengeID(5), triggers:vec!["Cafe".into()], message:Some("That's lame. End Game".into())},
            ]
        },
        Challenge { // 5
            name: "Painting".into(),
            desc: "You finally arrive at the painting, how do you get it off the wall:  cut it out of the frame, pick the security lock, use acetone to dissolve the clue".into(),
            paths:vec![
                Path{target:ChallengeID(6), triggers:vec!["Cut it".into()], message:Some("Success".into())},
                Path{target:ChallengeID(6), triggers:vec!["Pick lock".into()], message:Some("Success".into())},
                Path{target:ChallengeID(6), triggers:vec!["Acetone".into()], message:Some("Success".into())},
            ]
        },
        Challenge { // 6
            name: "End".into(),
            desc: "You win".into(),
            paths:vec![]
        },
    ];

    let end_rooms = [RoomID(6), RoomID(6), RoomID(6)];
    let mut input = String::new();

    let mut at = RoomID(0);
    println!("The Spooky Mansion Adventure");
    println!("============================");
    println!();
    println!("You've been walking for hours in the countryside, and have finally stumbled on the spooky mansion you read about in the tour guide.");
    loop {
        // We don't want to move out of rooms, so we take a reference
        let here = &rooms[at.0];
        println!("{}\n{}", here.name, here.desc);
        if end_rooms.contains(&at) {
            break;
        }
        loop {
            print!("What will you do?\n> ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if let Some(door) = here.doors.iter().find(|d| d.triggers.iter().any(|t| *t == input)) {
                if let Some(msg) = &door.message {
                    println!("{}", msg);
                }
                at = door.target;
                break;
            } else {
                println!("You can't do that!");
            }
        }
    }
}