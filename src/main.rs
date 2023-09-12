mod ascii_art;
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

#[derive(PartialEq, Eq, Clone, Debug)]
struct Resource {
    name: String, // E.g. "Flashlight"
    description: String, // "E.g. A small portable source of light"
    picked: bool, // True if user selected resource
}

// Identification unit for a challenge
#[derive(PartialEq, Eq, Clone, Copy)]
struct ChallengeID(usize);

fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;

        // resources that the user can choose in the beginning
    let available_resources = [
        Resource {
            name: "Flashlight".to_string(),
            description: "A small flashlight with fresh batteries.".to_string(),
            picked: false,
        },
        Resource {
            name: "Night Vision Goggles".to_string(),
            description: "Special goggles that allow you to see in the dark.".to_string(),
            picked: false,
        },
        Resource {
            name: "Lock Pick Set".to_string(),
            description: "A set of lockpicking tools.".to_string(),
            picked: false,
        },
        Resource {
            name: "Knife".to_string(),
            description: "A sharp knife that could be useful in a pinch.".to_string(),
            picked: false,
        },
        Resource {
            name: "Makeup Palette".to_string(),
            description: "A makeup palette that could be used for disguise.".to_string(),
            picked: false,
        },
        Resource {
            name: "Marbles".to_string(),
            description: "A bag of marbles that can be used as distractions.".to_string(),
            picked: false,
        },
        Resource {
            name: "Banana".to_string(),
            description: "A ripe banana. It might come in handy.".to_string(),
            picked: false,
        },
        Resource {
            name: "Pocket Sand".to_string(),
            description: "A small bag of sand that could be used to blind pursuers.".to_string(),
            picked: false,
        },
    ];
        
    let challenges = [
        // ChallengeID corresponds to the index inside list challenges that you want
        Challenge { // 0
            name: "Rules".into(), // Turn a &'static string (string constant) into a String
            desc: "This is a choose your own adventure. Use the number keys to select your choice and press enter to move forward in the game. Pick 1, 2, or 3.".into(),
            paths: vec![Path{target:ChallengeID(1), triggers:vec!["1".into(), "2".into(), "3".into()], message:Some("Nice job!".to_string())}]
        },
        Challenge { //1
            name: "Entrance".into(),
            desc: "How do you plan to enter the museum? Through the (1) door, (2) vent, or (3) window?".into(),
            paths: vec![
                Path{target:ChallengeID(2), triggers:vec!["1".into()], message:Some("You slip through the door and find yourself in a dark basement.".into())},
                Path{target:ChallengeID(2), triggers:vec!["2".into()], message:Some("You shimmy through an exterior vent and crawl through the ducts.".into())},
                Path{target:ChallengeID(2), triggers:vec!["3".into()], message:Some("You climb through the window and find yourself in a dark storage room.".into())}
            ]
        },
        Challenge { // 2
            name: "You're stuck!".into(),
            desc: "Oh no, you are locked in...do you (1) take a moment to think and eat a snack, (2) go straight to the door and pick the lock, (3) shine your flashlight to see if you can find a way out?".into(),
            // resource checking
            paths:vec![
                Path{target:ChallengeID(3), triggers:vec!["1".into()], message:Some("The pungent smell of banana fills the room, so much so that you can almost see it drifting out under the door.".into())},
                Path{target:ChallengeID(3), triggers:vec!["2".into()], message:Some("You pick the lock on the door and it swings open too fast for you to catch it before it slams with a CRASH into the wall. ".into())},
                Path{target:ChallengeID(3), triggers:vec!["3".into()], message:Some("You turn on the flashlight and discover a secret door".into())},
            ]
        },
        Challenge { // 3
            name: "Footsteps".into(),
            desc: "You exit into a hallway and hear footsteps coming your way. Do you (1) run blindly away, (2) hide behind a nearby statue, or (3) run towards the sound?".into(),
            paths:vec![
                Path{target:ChallengeID(10), triggers:vec!["1".into()], message:Some("The guard's partner has caught you. End Game".into())},
                Path{target:ChallengeID(4), triggers:vec!["2".into()], message:Some("The security guard stops next to you...you need a distraction. TBD".into())},
                Path{target:ChallengeID(5), triggers:vec!["3".into()], message:Some("You approach the security guard...what now? TBD".into())},
            ]
        },
        Challenge { // 4
            name: "Map".into(),
            desc: "You've successfully evaded the guard! Where do you go next, (1) the tourist shop, (2) security office, or (3) cafe?".into(),
            paths:vec![
                Path{target:ChallengeID(6), triggers:vec!["1".into()], message:Some("You enter the gift shop...".into())},
                Path{target:ChallengeID(7), triggers:vec!["2".into()], message:Some("You enter the security office...".into())},
                Path{target:ChallengeID(10), triggers:vec!["3".into()], message:Some("That's lame. End Game".into())},
            ]
        },
        Challenge { // 5
            name: "Talk to Guard".into(),
            desc: "You run smack into the guard. Quick do you (1) pretend to be a ghost, (2) pretend to be lost, or (3) be friendly".into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["1".into()], message:Some("Success".into())},
                Path{target:ChallengeID(8), triggers:vec!["2".into()], message:Some("Bad End".into())},
                Path{target:ChallengeID(8), triggers:vec!["3".into()], message:Some("Success".into())},
            ]
        },
        Challenge { // 6
            name: "Tourist Shop".into(),
            desc: "You are at the tourist shop: (1) try on merch, (2) look at the tourist maps, (3) raid the cash register".into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["1".into()], message:Some("Success".into())},
                Path{target:ChallengeID(10), triggers:vec!["2".into()], message:Some("Bad End".into())},
                Path{target:ChallengeID(8), triggers:vec!["3".into()], message:Some("Success".into())},
            ]
        },
        Challenge { // 7
            name: "Security office".into(),
            desc: "You sneak into the security office. There are lots of things lying around and cameras on the wall. Do you investigate (1) the cameras, (2) the fridge, (3) the posters on the wall".into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["3".into()], message:Some("You see the painting on the cameras, now you know where to go".into())},
                Path{target:ChallengeID(10), triggers:vec!["2".into()], message:Some("Bad End".into())},
                Path{target:ChallengeID(8), triggers:vec!["1".into()], message:Some("Congrats you found blueprints that show you where the painting is".into())},
            ]
        },
        Challenge { // 8
            name: "Painting".into(),
            desc: "You finally arrive at the painting, how do you get it off the wall: (1) cut it out of the frame, (2) pick the security lock, (3) use acetone to dissolve the clue".into(),
            paths:vec![
                Path{target:ChallengeID(9), triggers:vec!["1".into()], message:None},
                Path{target:ChallengeID(9), triggers:vec!["2".into()], message:None},
                Path{target:ChallengeID(9), triggers:vec!["3".into()], message:None},
            ]
        },
        Challenge { // 9
            name: "Good End".into(),
            desc: "You win".into(),
            paths:vec![]
        },
        Challenge { // 10
            name: "Bad End".into(),
            desc: "You Lose".into(),
            paths:vec![]
        },
    ];

    let end_challenges = [ChallengeID(10), ChallengeID(9)];
    let mut input = String::new();
    let mut current_challenge_id = ChallengeID(0);

    
    println!();
    println!();
    println!("{}", ascii_art::TITLE);
    println!("============================");
    println!();
    println!("{}", ascii_art::MUSEUM);
    println!("You are embarking on an art heist at ___.");
    
    // pick resources
    let mut selected_resources = Vec::new();
    println!("Available Resources:");
    for (index, resource) in available_resources.iter().enumerate() {
        println!("{}. {} - {}", index + 1, resource.name, resource.description);
    }
    
    // user will pick 3 resources to carry
    while selected_resources.len() < 3 {
        print!("Enter the number of the resource to pick (1-{}): ", available_resources.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice >= 1 && choice <= available_resources.len() {
                let selected_index = choice - 1;
                if !selected_resources.contains(&available_resources[selected_index]) {
                    selected_resources.push(available_resources[selected_index].clone());
                    println!("You picked up the {}.", available_resources[selected_index].name);
                } else {
                    println!("You already have one of those! Pick something else.");
                }
            } else {
                println!("Invalid input. Enter a number between 1 and {}.", available_resources.len());
            }
        } else {
            println!("Invalid input. Enter a valid number.");
        }
    }

    fn find_challenge_by_id(challenges: &[Challenge], id: ChallengeID) -> Option<&Challenge> {
        challenges.get(id.0)
    }
    
    loop {
        let current_challenge = match find_challenge_by_id(&challenges, current_challenge_id) {
            Some(challenge) => challenge,
            None => {
                println!("Game over!");
                break;
            }
        };
        // We don't want to move out of rooms, so we take a reference
        let here = &challenges[current_challenge_id.0];
        println!("{}\n{}", here.name, here.desc);

        // get player input
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Check if the player's choice is a valid option
        let selected_path_index = input.parse::<usize>();

        match selected_path_index {
            Ok(index) => {
                if index >= 1 && index <= current_challenge.paths.len() {
                    let selected_path = &current_challenge.paths[index - 1];

                    if let Some(message) = &selected_path.message {
                        println!("{}", message);
                    }

                    // Check if this path leads to ending the game
                    if selected_path.target.0 == 10 || selected_path.target.0 == 9{
                        println!("Game Over!");
                        return; // End the game here
                    }

                    // Move to the next challenge based on the selected path's target
                    current_challenge_id = selected_path.target;
                } else {
                    println!("Invalid choice. Please select a valid option.");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number.");
            }
        }


        // if end_challenges.contains(&current_challenge_id) {
        //     println!{"hello"}
        //     break;
        // }
        // loop {
        //     // print!("What will you do?\n> ");
        //     io::stdout().flush().unwrap();
        //     input.clear();
        //     io::stdin().read_line(&mut input).unwrap();
        //     let input = input.trim();

        //     // give option to leave game
        //     if input.eq("exit") || input.eq("quit") {
        //         println!("Exiting the game. Goodbye!");
        //         return;
        //     }

        //     if let Some(path) = here.paths.iter().find(|d| d.triggers.iter().any(|t| *t == input)) {
        //         if let Some(msg) = &path.message {
        //             println!("{}", msg);
        //         }
        //         current_challenge_id = path.target;
        //         break;
        //     } else {
        //         println!("You can't do that!");
        //     }
        // }
    }
}