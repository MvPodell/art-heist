mod ascii_art;

struct Challenge {
    name: String, // E.g. "Antechamber"
    desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    paths: Vec<Path>,
    password: Option<String>
}
struct Path {
    target: ChallengeID, // More about this in a minute
    triggers:Vec<String>, // e.g. "go north", "north"
    pre_message: Option<String>, // the name of the path the is given to the user when they are asked what they want to do
    post_message: Option<String>, // What message, if any, to print when the doorway is traversed
    required_resource: Option<Resource>,
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

fn lock_picking_challenge() -> bool{
    use std::io;
    let secret_combination = "1402";

    println!("You need to guess the 4-digit combination.");

    let mut attempts = 3;

    while attempts > 0 {
        println!("Enter a 4-digit combination: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        let input_set: std::collections::HashSet<char> = guess.chars().collect();
        let actual_set: std::collections::HashSet<char> = secret_combination.chars().collect();

        if guess == secret_combination {
            println!("Congratulations! You successfully unlocked the lock.");
            return true;
        } else if input_set ==actual_set{
            println!("You have all the right numbers but in the wrong order. You have {} attempts remaining.", attempts - 1);
        } else {
            println!("Incorrect combination. You have {} attempts remaining.", attempts - 1);
        }

        attempts -= 1;

        if attempts == 0 {
            println!("Out of attempts. The lock remains locked.");
        }
    }
    return false;
}


fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;

        // resources that the user can choose in the beginning
    let available_resources = [
        Resource { // 0
            name: "Flashlight".to_string(),
            description: "A small flashlight with fresh batteries.".to_string(),
            picked: false,
        },
        Resource { //1
            name: "Night Vision Goggles".to_string(),
            description: "Special goggles that allow you to see in the dark.".to_string(),
            picked: false,
        },
        Resource { //2
            name: "Lock Pick Set".to_string(),
            description: "A set of lockpicking tools.".to_string(),
            picked: false,
        },
        Resource { //3
            name: "Knife".to_string(),
            description: "A sharp knife that could be useful in a pinch.".to_string(),
            picked: false,
        },
        Resource {// 4
            name: "Makeup Palette".to_string(),
            description: "A makeup palette that could be used for disguise.".to_string(),
            picked: false,
        },
        Resource { //5
            name: "Marbles".to_string(),
            description: "A bag of marbles that can be used as distractions.".to_string(),
            picked: false,
        },
        Resource { // 6
            name: "Banana".to_string(),
            description: "A ripe banana. It might come in handy.".to_string(),
            picked: false,
        },
        Resource { //7
            name: "Pocket Sand".to_string(),
            description: "A small bag of sand that could be used to blind pursuers.".to_string(),
            picked: false,
        },
    ];
        
    let challenges = [
        // ChallengeID corresponds to the index inside list challenges that you want
        Challenge { // 0
            name: "Rules".into(), // Turn a &'static string (string constant) into a String
            desc: "\n \nThis is a choose your own adventure. Use the number keys to select your choice and press enter to move forward in the game. Pick 1, 2, or 3.".into(),
            paths: vec![Path{target:ChallengeID(1), triggers:vec!["1".into(), "2".into(), "3".into()], pre_message: None, post_message:Some("Nice job!".to_string()), required_resource: None}],
            password: None,
        },
        Challenge { //1
            name: "Entrance".into(),
            desc: "\n \nHow do you plan to enter the museum? Through the door, vent, or window?".into(),
            paths: vec![
                Path{target:ChallengeID(2), triggers:vec!["1".into()], pre_message: Some("door".into()), post_message:Some("You slip through the door and find yourself in a dark basement.".into()), required_resource: None}, // No required resource for this path
                Path{target:ChallengeID(2), triggers:vec!["2".into()], pre_message: Some("vent".into()), post_message:Some("You shimmy through an exterior vent and crawl through the ducts.".into()), required_resource: None},
                Path{target:ChallengeID(2), triggers:vec!["3".into()], pre_message: Some("window".into()), post_message:Some("You climb through the window and find yourself in a dark storage room.".into()), required_resource: None},
            ],
            password: None,
            
        },
        Challenge { // 2
            name: "You're stuck!".into(),
            desc: "\n \nOh no, you are locked in...do you (1) take a moment to think and eat a snack, (2) go straight to the door and pick the lock, (3) shine your flashlight to see if you can find a way out?".into(),
            // resource checking
            paths:vec![
                Path{target:ChallengeID(3), triggers:vec!["1".into()], pre_message: Some("eat a snack".into()), post_message:Some("The pungent smell of banana fills the room, so much so that you can almost see it drifting out under the door.".into()), required_resource: Some(available_resources[6].clone())},
                Path{target:ChallengeID(3), triggers:vec!["2".into()], pre_message: Some("pick the lock".into()), post_message:Some("You pick the lock on the door and it swings open too fast for you to catch it before it slams with a CRASH into the wall. ".into()), required_resource: Some(available_resources[2].clone())},
                Path{target:ChallengeID(3), triggers:vec!["3".into()], pre_message: Some("use your flashlight".into()), post_message:Some("You turn on the flashlight and discover a secret door".into()), required_resource: Some(available_resources[0].clone())},
            ],
            password: None,
        },
        Challenge { // 3
            name: "Footsteps".into(),
            desc: "\n \nYou exit into a hallway and hear footsteps coming your way. Do you (1) run blindly away, (2) hide behind a nearby statue, or (3) run towards the sound?".into(),
            paths:vec![
                Path{target:ChallengeID(10), triggers:vec!["1".into()], pre_message: Some("run away".into()), post_message:Some("The guard's partner has caught you. End Game".into()), required_resource: None},
                Path{target:ChallengeID(11), triggers:vec!["2".into()], pre_message: Some("hide".into()), post_message:Some("The security guard stops next to you...you need a distraction. TBD".into()), required_resource: None},
                Path{target:ChallengeID(5), triggers:vec!["3".into()], pre_message: Some("run towards the sound".into()), post_message:Some("You approach the security guard...what now? TBD".into()), required_resource: None},
            ],
            password: None,
        },
        Challenge { // 4
            name: "Map".into(),
            desc: "\n \nYou've successfully evaded the guard! Where do you go next, (1) the tourist shop, (2) security office, or (3) cafe?".into(),
            paths:vec![
                Path{target:ChallengeID(6), triggers:vec!["1".into()], pre_message: Some("tourist shop".into()), post_message:Some("You enter the gift shop...".into()), required_resource: None},
                Path{target:ChallengeID(7), triggers:vec!["2".into()], pre_message: Some("security office".into()), post_message:Some("You enter the security office...".into()), required_resource: None},
                Path{target:ChallengeID(10), triggers:vec!["3".into()], pre_message: Some("cafe".into()), post_message:Some("You enter the cafe and find a group of guards sitting at one of the tables eating doughnuts. You're toast.".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 5
            name: "Talk to Guard".into(),
            desc: "\n \nYou run smack into the guard. Do something quickly! Do you (1) pretend to be a ghost, (2) pretend to be lost, or (3) try to befriend them".into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["1".into()], pre_message: Some("pretend to be a ghost".into()), post_message:Some("OOoooOooooOOO".into()), required_resource: Some(available_resources[4].clone())},
                Path{target:ChallengeID(10), triggers:vec!["2".into()], pre_message: Some("pretend to be lost".into()), post_message:Some("The guard didn't believe your act and called the police".into()), required_resource: None},
                Path{target:ChallengeID(8), triggers:vec!["3".into()], pre_message: Some("befriend the guard".into()), post_message:Some("Success".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 6
            name: "Tourist Shop".into(),
            desc: "\n \nYou are at the tourist shop: (1) try on merch, (2) look at the tourist maps, (3) raid the cash register".into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["1".into()], pre_message: Some("try on merch".into()), post_message:Some("Success".into()), required_resource: None},
                Path{target:ChallengeID(8), triggers:vec!["2".into()], pre_message: Some("look at the tourist maps".into()), post_message:Some("You look at the maps and find directions to the painting".into()), required_resource: None},
                Path{target:ChallengeID(10), triggers:vec!["3".into()], pre_message: Some("raid the cash register".into()), post_message:Some("You found money but not the painting, you did not complete the mission".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 7
            name: "Security office".into(),
            desc: "\n \nYou sneak into the security office. There are lots of things lying around and cameras on the wall. Do you investigate (1) the cameras, (2) the fridge, (3) the posters on the wall".into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["3".into()], pre_message: Some("cameras".into()), post_message:Some("The system is password protect but theres a hint! I have cities, but no houses. I have mountains, but no trees. I have water, but no fish. What am I? (Format like 'A ___'".into()), required_resource: None},
                Path{target:ChallengeID(10), triggers:vec!["2".into()], pre_message: Some("fridge".into()), post_message:Some("Food? At a time like this?".into()), required_resource: None},
                Path{target:ChallengeID(8), triggers:vec!["1".into()], pre_message: Some("posters".into()), post_message:Some("Congrats you found blueprints that show you where the painting is".into()), required_resource: None},
            ],
            password: Some("A map".into()), 
        },
        Challenge { // 8
            name: "Painting".into(),
            desc: "\n \nYou finally arrive at the painting, how do you get it off the wall: (1) cut it out of the frame, (2) pick the security lock, (3) use acetone to dissolve the clue".into(),
            paths:vec![
                Path{target:ChallengeID(9), triggers:vec!["1".into()], pre_message: Some("cut it out".into()), post_message:None, required_resource: Some(available_resources[3].clone())},
                Path{target:ChallengeID(13), triggers:vec!["2".into()], pre_message: Some("pick the lock".into()), post_message:None, required_resource: Some(available_resources[2].clone())},
                Path{target:ChallengeID(9), triggers:vec!["3".into()], pre_message: Some("dissolve the glue".into()), post_message:None, required_resource: Some(available_resources[4].clone())},
            ],
            password: None, 
        },
        Challenge { // 9
            name: "Good End".into(),
            desc: "\n \nYou win".into(),
            paths:vec![],
            password: None, 
        },
        Challenge { // 10
            name: "Bad End".into(),
            desc: "\n \nYou Lose".into(),
            paths:vec![],
            password: None, 
        },
        Challenge { // 11
            name: "Distraction!".into(),
            desc: "\n \nThe guard's almost reached you! What are you going to do? 1) Throw a marble in another direction to get them off your tail 2) strategically place your banana peel so that they slip 3) Attack them with your knife?".into(),
            paths:vec![
                Path{target:ChallengeID(4), triggers:vec!["3".into()], pre_message: Some("throw the marble".into()), post_message:Some("The marble bounces off into another room and the guard runs after the noise.".into()), required_resource: Some(available_resources[5].clone())},
                Path{target:ChallengeID(4), triggers:vec!["2".into()], pre_message: Some("use the banana peel".into()), post_message:Some("The guard slips on the banana peel and you manage to run off while they're down.".into()), required_resource: Some(available_resources[6].clone())},
                Path{target:ChallengeID(10), triggers:vec!["1".into()], pre_message: Some("attack them with your knife".into()), post_message:Some("The guard knows martial arts. They pin you to the ground easily and call the police.".into()), required_resource: Some(available_resources[3].clone())},
            ],
            password: None, 
        },
        Challenge { // 12
            name: "It's dark".into(),
            desc: "\n \nYou run towards the guard, taking advantage of the darkness to catch them off guard. What's your course of action? \n 1) Attack them 2) Blind them with your sand. 3) Cautiously talk to them.".into(),
            paths:vec![
                Path{target:ChallengeID(10), triggers:vec!["3".into()], pre_message: Some("attack them your knife".into()), post_message:Some("Really? You're a CS major, stop kidding yourself.".into()), required_resource: Some(available_resources[3].clone())},
                Path{target:ChallengeID(4), triggers:vec!["2".into()], pre_message: Some("blind them with your sand".into()), post_message:Some("The guard screams and claws at their eyes and you manage to escape during their suffering.".into()), required_resource: Some(available_resources[7].clone())},
                Path{target:ChallengeID(5), triggers:vec!["1".into()], pre_message: Some("talk to them".into()), post_message:Some("".into()), required_resource: Some(available_resources[0].clone())},
            ],
            password: None, 
        },
        Challenge { // 13
            name: "Lock picking".into(),
            desc: "Press 1 to End Game".into(),
            paths:vec![Path{target:ChallengeID(9), triggers:vec!["1".into()], pre_message: None, post_message: None, required_resource: None},],
            password: None, 
        },

    ];

    // let end_challenges = [ChallengeID(10), ChallengeID(9)];
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
        println!("{}\n", here.desc);

        println!("Available options:");

        for (index, path) in current_challenge.paths.iter().enumerate() {
            // Check if the path has a required resource
            if let Some(required_resource) = &path.required_resource {
                // Check if the player has the required resource
                if selected_resources.contains(required_resource) {
                    println!("{}. {}", index + 1, path.pre_message.as_ref().unwrap_or(&"".to_string()));
                }
            } else {
                println!("{}. {}", index + 1, path.pre_message.as_ref().unwrap_or(&"".to_string()));
            }
        }

        println!("Enter the number of your choice:");

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

                    // Check if the path has a required resource
                    if let Some(required_resource) = &selected_path.required_resource {
                        // Check if the player has the required resource
                        if !selected_resources.contains(required_resource) {
                            println!("You need the {} to access this path.", required_resource.name);
                            continue; // Don't proceed with this path
                        }
                    }

                    // allow user to quit game
                    if input.eq("exit") || input.eq("quit") {
                        println!("Exiting the game. Goodbye!");
                        return;
                    }

                    if let Some(post_message) = &selected_path.post_message {
                        println!("{}", post_message);
                    }

                    // Check if this path leads to ending the game
                    if selected_path.target.0 == 10 || selected_path.target.0 == 9{
                        println!("Game Over!");
                        return; // eng the game here
                    }

                    // lock picking
                    if selected_path.target.0 == 13 {
                        // check if the  current is the lock-picking challenge - 13
                        let lock_picked = lock_picking_challenge();
                        if lock_picked {
                            println!("You've successfully picked the lock and stolen the painting. You win!");
                            // Move to the next challenge 
                            current_challenge_id = ChallengeID(9);
                        } else {
                            println!("You can't get the painting off by picking the lock. You lose :(");
                            //  return to original challenge
                            current_challenge_id = ChallengeID(8);
                        }
                    }

                    // password checking
                    if let Some(password) = current_challenge.password.clone() {
                        let mut password_input = String::new();
                        io::stdin().read_line(&mut password_input).unwrap();
                        let password_input = password_input.trim();

                        if password_input == password {
                            println!("Password correct! Access approved.");
                        } else {
                            println!("Incorrect password. Access denied.");
                            continue; //go back to the security office options
                        }
                    }

                    // Move to the next challenge based on the selected path's target
                    current_challenge_id = selected_path.target;
                } else {
                    println!("Invalid. Please select a valid option.");
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