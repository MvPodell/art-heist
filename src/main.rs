mod ascii_art;

struct Challenge {
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
    ascii_art: Option<String>,
}

// Identification unit for a challenge
#[derive(PartialEq, Eq, Clone, Copy)]
struct ChallengeID(usize);

fn enter_to_continue() {
    use std::io;
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn lock_picking_challenge() -> bool{
    use std::io;
    let secret_combination = "1887";

    println!("You need to guess the 4-digit combination. The year Pomona was founded...");

    let mut attempts = 3;

    while attempts > 0 {
        println!("Enter a 4-digit combination: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        let input_set: std::collections::HashSet<char> = guess.chars().collect();
        let actual_set: std::collections::HashSet<char> = secret_combination.chars().collect();

        if guess == secret_combination {
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
            ascii_art: Some("false".into())
        },
        Resource { //1
            name: "Night Vision Goggles".to_string(),
            description: "Special goggles that allow you to see in the dark.".to_string(),
            picked: false,
            ascii_art: Some("false".into()),
        },
        Resource { //2
            name: "Lock Pick Set".to_string(),
            description: "A set of lockpicking tools.".to_string(),
            picked: false,
            ascii_art: Some("false".into()),
        },
        Resource { //3
            name: "Knife".to_string(),
            description: "A sharp knife that could be useful in a pinch.".to_string(),
            picked: false,
            ascii_art: Some("false".into()),
        },
        Resource {// 4
            name: "Makeup Palette".to_string(),
            description: "A makeup palette that could be used for disguise.".to_string(),
            picked: false,
            ascii_art: Some("false".into()),
        },
        Resource { //5
            name: "Marbles".to_string(),
            description: "A bag of marbles that can be used as distractions.".to_string(),
            picked: false,
            ascii_art: Some("false".into()),
        },
        Resource { // 6
            name: "Banana".to_string(),
            description: "A ripe banana. It might come in handy.".to_string(),
            picked: false,
            ascii_art: None,
        },
        Resource { //7
            name: "Pocket Sand".to_string(),
            description: "A small bag of sand that could be used to blind pursuers.".to_string(),
            picked: false,
            ascii_art: Some("false".into()),
        },
    ];
        
    let challenges = [
        // ChallengeID corresponds to the index inside list challenges that you want
        Challenge { //0
            desc: "How do you plan to enter the museum? Through the door, vent, or window?".into(),
            paths: vec![
                Path{target:ChallengeID(1), triggers:vec!["1".into()], pre_message: Some("door".into()), post_message:Some("You slip through the door and find yourself in a dark basement.".into()), required_resource: None}, // No required resource for this path
                Path{target:ChallengeID(1), triggers:vec!["2".into()], pre_message: Some("vent".into()), post_message:Some("You shimmy through an exterior vent and crawl through the ducts.".into()), required_resource: None},
                Path{target:ChallengeID(1), triggers:vec!["3".into()], pre_message: Some("window".into()), post_message:Some("You climb through the window and find yourself in a dark storage room.".into()), required_resource: None},
            ],
            password: None,
            
        },
        Challenge { // 1
            desc: "Oh no, you are locked in...do you (1) take a moment to think and eat a snack, (2) go straight to the door and pick the lock, (3) shine your flashlight to see if you can find a way out?".into(),
            // resource checking
            paths:vec![
                Path{target:ChallengeID(2), triggers:vec!["1".into()], pre_message: Some("eat a snack".into()), post_message:Some(format!("{} \nThe pungent smell of banana fills the room, so much so that you can almost see it drifting out under the door.", ascii_art::BANANA).into()), required_resource: Some(available_resources[6].clone())},
                Path{target:ChallengeID(2), triggers:vec!["2".into()], pre_message: Some("pick the lock".into()), post_message:Some("You pick the lock on the door and it swings open too fast for you to catch it before it slams with a CRASH into the wall. ".into()), required_resource: Some(available_resources[2].clone())},
                Path{target:ChallengeID(2), triggers:vec!["3".into()], pre_message: Some("use your flashlight".into()), post_message:Some("You turn on the flashlight and discover a secret door".into()), required_resource: Some(available_resources[0].clone())},
            ],
            password: None,
        },
        Challenge { // 2 Footsteps
            desc: "You exit into a hallway and hear footsteps coming your way. Do you (1) run blindly away, (2) hide behind a nearby priceless statue, or (3) run towards the sound?".into(),
            paths:vec![
                Path{target:ChallengeID(9), triggers:vec!["1".into()], pre_message: Some("run away".into()), post_message:Some("The guard's partner has caught you.".into()), required_resource: None},
                Path{target:ChallengeID(10), triggers:vec!["2".into()], pre_message: Some("hide".into()), post_message:Some("The security guard stops next to you...you need a distraction.".into()), required_resource: None},
                Path{target:ChallengeID(4), triggers:vec!["3".into()], pre_message: Some("run towards the sound".into()), post_message:Some("You approach the security guard and...".into()), required_resource: None},
            ],
            password: None,
        },
        Challenge { // 3 Evaded guard
            desc: "You've successfully evaded the guard! Where do you go next, (1) the tourist shop, (2) security office, or (3) cafe?".into(),
            paths:vec![
                Path{target:ChallengeID(5), triggers:vec!["1".into()], pre_message: Some("tourist shop".into()), post_message:Some("You enter the gift shop...".into()), required_resource: None},
                Path{target:ChallengeID(6), triggers:vec!["2".into()], pre_message: Some("security office".into()), post_message:Some("You enter the security office...".into()), required_resource: None},
                Path{target:ChallengeID(9), triggers:vec!["3".into()], pre_message: Some("cafe".into()), post_message:Some("You enter the cafe and find a group of guards sitting at one of the tables eating doughnuts. You're toast.".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 4 talk to guard
            desc: "You run smack into the guard. Do something quickly! Do you (1) pretend to be a ghost, (2) pretend to be lost, or (3) try to befriend them".into(),
            paths:vec![
                Path{target:ChallengeID(7), triggers:vec!["1".into()], pre_message: Some("pretend to be a ghost".into()), post_message:Some(format!("{} \nYou quickly slap on some pale foundation and white powder. You're the splitting image of Casper. OOoooOooooOOO. \nThe guard screams with terror, runs away, and leaves you with an open path to the painting.", ascii_art::CASPER).into()), required_resource: Some(available_resources[4].clone())},
                Path{target:ChallengeID(9), triggers:vec!["2".into()], pre_message: Some("pretend to be lost".into()), post_message:Some("The guard didn't believe your act and called the police. You should enroll in an acting class.".into()), required_resource: None},
                Path{target:ChallengeID(7), triggers:vec!["3".into()], pre_message: Some("befriend the guard".into()), post_message:Some("Your charming wit convinced the guard your mission was worth it.".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 5 tourist shop
            desc: "You are at the tourist shop: (1) try on merch, (2) look at the tourist maps, (3) raid the cash register".into(),
            paths:vec![
                Path{target:ChallengeID(7), triggers:vec!["1".into()], pre_message: Some("try on merch".into()), post_message:Some("The merch looks amazing on you, but the guard catches you while you are stashing your bag. You didn't have room to carry the painting anyways :(".into()), required_resource: None},
                Path{target:ChallengeID(7), triggers:vec!["2".into()], pre_message: Some("look at the tourist maps".into()), post_message:Some("You look at the maps and find directions to the painting".into()), required_resource: None},
                Path{target:ChallengeID(9), triggers:vec!["3".into()], pre_message: Some("raid the cash register".into()), post_message:Some("You found money but not the painting, you did not complete the mission".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 6 security office
            desc: "You sneak into the security office. There are lots of things lying around and cameras on the wall. Do you investigate (1) the cameras, (2) the posters on the wall (3) the fridge".into(),
            paths:vec![
                Path{target:ChallengeID(13), triggers:vec!["1".into()], pre_message: Some("cameras".into()), post_message:None, required_resource: None},
                Path{target:ChallengeID(7), triggers:vec!["2".into()], pre_message: Some("posters".into()), post_message:Some("Congrats you found blueprints that show you where the painting is. You sneakily walk through the hallways avoiding any remaining guards until you arrive at the world famous piece of art.".into()), required_resource: None},
                Path{target:ChallengeID(9), triggers:vec!["3".into()], pre_message: Some("fridge".into()), post_message:Some("Food? At a time like this?".into()), required_resource: None},
            ],
            password: None, 
        },
        Challenge { // 7 painting
            desc: format!("{}\n You finally arrive at the painting, how do you get it off the wall: (1) cut it out of the frame, (2) pick the security lock, (3) use acetone to dissolve the clue", ascii_art::CECIL).into(),
            paths:vec![
                Path{target:ChallengeID(8), triggers:vec!["1".into()], pre_message: Some("cut it out".into()), post_message:Some("While doing irreparable damage to the painting, you manage to get it out of the frame in the fastest time.".into()), required_resource: Some(available_resources[3].clone())},
                Path{target:ChallengeID(12), triggers:vec!["2".into()], pre_message: Some("pick the lock".into()), post_message:None, required_resource: Some(available_resources[2].clone())},
                Path{target:ChallengeID(8), triggers:vec!["3".into()], pre_message: Some("dissolve the glue".into()), post_message:Some("You paint the glue on the back of the frame. The painting remains unharmed and you can sell it for maximum profit!".into()), required_resource: Some(available_resources[4].clone())},
            ],
            password: None, 
        },
        Challenge { // 8 Good end
            desc: "You win".into(),
            paths:vec![],
            password: None, 
        },
        Challenge { // 9 Bad end
            desc: "You Lose".into(),
            paths:vec![],
            password: None, 
        },
        Challenge { // 10 distraction
            desc: "The guard's almost reached you! What are you going to do? 1) Throw a marble in another direction to get them off your tail 2) strategically place your banana peel so that they slip 3) Attack them with your knife?".into(),
            paths:vec![
                Path{target:ChallengeID(3), triggers:vec!["3".into()], pre_message: Some("throw the marble".into()), post_message:Some("The marble bounces off into another room and the guard runs after the noise.".into()), required_resource: Some(available_resources[5].clone())},
                Path{target:ChallengeID(3), triggers:vec!["2".into()], pre_message: Some("use the banana peel".into()), post_message:Some("The guard slips on the banana peel and you manage to run off while they're down.".into()), required_resource: Some(available_resources[6].clone())},
                Path{target:ChallengeID(9), triggers:vec!["1".into()], pre_message: Some("attack them with your knife".into()), post_message:Some("The guard knows martial arts. They pin you to the ground easily and call the police.".into()), required_resource: Some(available_resources[3].clone())},
            ],
            password: None, 
        },
        Challenge { // 11 Run towards guard
            desc: "You run towards the guard, taking advantage of the darkness to catch them off guard. What's your course of action? \n 1) Attack them 2) Blind them with your sand. 3) Cautiously talk to them.".into(),
            paths:vec![
                Path{target:ChallengeID(9), triggers:vec!["3".into()], pre_message: Some("attack them your knife".into()), post_message:Some("Really? You're a CS major, stop kidding yourself.".into()), required_resource: Some(available_resources[3].clone())},
                Path{target:ChallengeID(3), triggers:vec!["2".into()], pre_message: Some("blind them with your sand".into()), post_message:Some("The guard screams and claws at their eyes and you manage to escape during their suffering.".into()), required_resource: Some(available_resources[7].clone())},
                Path{target:ChallengeID(4), triggers:vec!["1".into()], pre_message: Some("talk to them".into()), post_message:Some("".into()), required_resource: Some(available_resources[0].clone())},
            ],
            password: None, 
        },
        Challenge { // 12 lock picking
            desc: "You are picking the lock.".into(),
            paths:vec![Path{target:ChallengeID(8), triggers:vec!["1".into()], pre_message: None, post_message: None, required_resource: None},],
            password: None, 
        },
        Challenge { //13 password
            desc: "The system is password protect but theres a hint! I have cities, but no houses. I have mountains, but no trees. I have water, but no fish. What am I? (Format like 'A ___'".into(),
            paths:vec![],
            password: Some("A map".into()), 
        },
    ];

    // let end_challenges = [ChallengeID(10), ChallengeID(9)];
    let mut input = String::new();
    let mut current_challenge_id = ChallengeID(0);

    
    println!();
    println!();
    println!("{}", ascii_art::TITLE);
    println!("-----------------------------------------------------------------------------------------");
    enter_to_continue();
    println!();
    println!("{}", ascii_art::MUSEUM);
    enter_to_continue();
    println!("In the heart of the bustling city of Claremont, nestled within the renowned Benton Art Museum, lies a masterpiece that has captivated collectors and historians alike. The painting, a stunning rendition of Cecil, the beloved mascot of Pomona College, is not just a symbol of the institution's rich heritage but a work of unparalleled artistry. Crafted by a reclusive genius, it is rumored to conceal a hidden treasure map to an ancient artifact of immeasurable worth. As an audacious art thief, you embark on a perilous heist into the museum's fortified walls, driven by the allure of untold riches and the challenge of outwitting state-of-the-art security systems to claim the elusive masterpiece and decipher its enigmatic secret.");
    enter_to_continue();
    println!("This is a choose your own adventure. Be sure to make smart and careful decisions to win the game.");
    // pick resources
    let mut selected_resources = Vec::new();
    println!("What tools will you carry into the museum with you? Some tools may seem useless but are in actuality very useful. Pick three.");
    enter_to_continue();
    println!("Available Resources (Pick 3):");
    for (index, resource) in available_resources.iter().enumerate() {
        println!("{}. {} - {}", index + 1, resource.name, resource.description);
    }

    // user will pick 3 resources to carry
    while selected_resources.len() < 3 {
        print!("Enter the number of the resource to pick (1-{}): ", available_resources.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.eq("exit") || input.eq("quit") {
            println!("Exiting the game. Goodbye!");
            return;
        }
        
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

        // check if in a terminating challenge
        if current_challenge_id == ChallengeID(8) {
            println!("{} You've successfully stolen the painting and won the game! You have established a reputation as a renowned international thief.", ascii_art::MONEY);
            break;
        }
        if current_challenge_id == ChallengeID(9) {
            println!("{} \nGame Over! You try to make a getaway but you trip on a step. You are truly a terrible thief and the police arrest you.", ascii_art::JAIL);
            break;
        }

        // if let Some(ascii_art) = &here.ascii_art {
        //     println!("{}", ascii_art);
        // }
        println!("-------------------------------------------------------------------------------------------");
        println!("{}", here.desc);
        println!("-------------------------------------------------------------------------------------------");

        // password checking
        if current_challenge_id == ChallengeID(13){
            let password = current_challenge.password.clone();
            if let Some(password) = password {
                let mut password_input = String::new();
                io::stdin().read_line(&mut password_input).unwrap();
                let password_input = password_input.trim();

                if password_input == password {
                    println!("Password correct! Access approved. Remarkable skills :) You can now see where the painting is...run quick!");
                    current_challenge_id = ChallengeID(8); // Forward to painting
                    continue;
                } else {
                    println!("Incorrect password. Access denied. You are not a computer whiz...");
                    current_challenge_id = ChallengeID(7); // Back to security office
                    continue;
                }
            }
        }

        let mut options_printed = false;

        for (index, path) in current_challenge.paths.iter().enumerate() {
            // Check if the path has a required resource
            if let Some(required_resource) = &path.required_resource {
                // Check if the player has the required resource
                if selected_resources.contains(required_resource) {
                    if !options_printed {
                        println!("Available options:");
                        options_printed = true;
                    }
                    println!("{}. {}", index + 1, path.pre_message.as_ref().unwrap_or(&"".to_string()));
                }
            } else {
                if !options_printed {
                    println!("Available options:");
                    options_printed = true;
                }
                println!("{}. {}", index + 1, path.pre_message.as_ref().unwrap_or(&"".to_string()));
            }
        }
        if !options_printed{
            println!("You had insufficient resources to continue moving forward in the heist. Please come better prepared next time.");
            println!("You lose.");
            break;
        }


        println!("Enter the number of your choice:");

        // get player input
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // allow user to quit game
        if input.eq("exit") || input.eq("quit") {
            println!("Exiting the game. Goodbye!");
            return;
        }

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

                    if let Some(post_message) = &selected_path.post_message {
                        println!("-----------------------------------------------------------------------------------------");
                        println!("{}\n", post_message);
                    }

                    // Move to the next challenge based on the selected path's target
                    current_challenge_id = selected_path.target;

                    // lock picking
                    if selected_path.target.0 == 12 {
                        // check if the  current is the lock-picking challenge - 13
                        let lock_picked = lock_picking_challenge();
                        if lock_picked {
                            println!("The lock pops open with a satisying click");
                            // Move to the next challenge 
                            current_challenge_id = ChallengeID(8);
                        } else {
                            //  return to original challenge
                            current_challenge_id = ChallengeID(7);
                        }
                    }

                } else {
                    println!("Invalid. Please select a valid option.");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number.");
            }
        }
    }
}