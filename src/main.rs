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
            name: "entrance".into(),
            desc: "How do you plan to enter the museum? Through the (1) door, (2) vent, or (3) window?".into(),
            paths: vec![
                Path{target:ChallengeID(2), triggers:vec!["1".into()], message:Some("You slip through the door and find yourself in a dark basement.".into())},
                Path{target:ChallengeID(2), triggers:vec!["2".into()], message:Some("You shimmy through an exterior vent and crawl through the ducts.".into())},
                Path{target:ChallengeID(2), triggers:vec!["3".into()], message:Some("You climb through the window and find yourself in a dark storage room.".into())}
            ]
        },
        Challenge { // 2
            name: "You're stuck!".into(),
            desc: "Oh no, you are locked in...do you (1) eat a snack, (2) pick the lock, (3) use the flashlight?".into(),
            // resource checking
            paths:vec![
                Path{target:ChallengeID(3), triggers:vec!["1".into()], message:Some("You eat a banana, but now you smell like banana".into())},
                Path{target:ChallengeID(3), triggers:vec!["2".into()], message:Some("Pick the lock".into())},
                Path{target:ChallengeID(3), triggers:vec!["3".into()], message:Some("You turn on the flashlight and discover a secret door".into())},
            ]
        },
        Challenge { // 3
            name: "Footsteps".into(),
            desc: "You exit into a hallway but, you hear footsteps coming down. Do you (1) run blindly away, (2) hide behind a statue, or (3) run towards the sound?".into(),
            paths:vec![
                Path{target:ChallengeID(4), triggers:vec!["1".into()], message:Some("The guard's partner has caught you. End Game".into())},
                Path{target:ChallengeID(4), triggers:vec!["2".into()], message:Some("The security guard stops next to you...you need a distraction. TBD".into())},
                Path{target:ChallengeID(5), triggers:vec!["3".into()], message:Some("You approach the security guard...what now? TBD".into())},
            ]
        },
        Challenge { // 4
            name: "Map".into(),
            desc: "You successfully evaded the guard. Where do you go next, (1) the tourist shop, (2) security office, or (3) cafe?".into(),
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
                Path{target:ChallengeID(8), triggers:vec!["2".into()], message:Some("Bad End".into())},
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
    let ascii_title = r#"
    _____ _______   _    _ ______ _____  _____ _______ 
    /\   |  __ \__   __| | |  | |  ____|_   _|/ ____|__   __|
   /  \  | |__) | | |    | |__| | |__    | | | (___    | |   
  / /\ \ |  _  /  | |    |  __  |  __|   | |  \___ \   | |   
 / ____ \| | \ \  | |    | |  | | |____ _| |_ ____) |  | |   
/_/    \_\_|  \_\ |_|    |_|  |_|______|_____|_____/   |_|
    "#;
    let ascii_museum = r#"
                                               /\      /\
                                               ||______||
                                               || ^  ^ ||
                                               \| |  | |/
                                                |______|
              __                                |  __  |
             /  \       ________________________|_/  \_|__
            / ^^ \     /=========================/ ^^ \===|
           /  []  \   /=========================/  []  \==|
          /________\ /=========================/________\=|
       *  | pomona |/==========================| museum |=|
      *** | ^^  ^^ |---------------------------| ^^  ^^ |--
     *****| []  [] |           _____           | []  [] | |
    *******        |          /_____\          |      * | |
   *********^^  ^^ |  ^^  ^^  |  |  |  ^^  ^^  |     ***| |
  ***********]  [] |  []  []  |  |  |  []  []  | ===***** |
 *************     |         @|__|__|@         |/ |*******|
***************   ***********--=====--**********| *********
***************___*********** |=====| **********|***********
 *************     ********* /=======\ ******** | *********
"#;

    let mut at = ChallengeID(0);
    println!();
    println!();
    println!("{}", ascii_title);
    println!("============================");
    println!();
    println!("{}", ascii_museum);
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
                    println!("You picked {}.", available_resources[selected_index].name);
                } else {
                    println!("Already have resource");
                }
            } else {
                println!("Invalid input. Enter a number between 1 and {}.", available_resources.len());
            }
        } else {
            println!("Invalid input. Enter a valid number.");
        }
    }
    
    loop {
        // We don't want to move out of rooms, so we take a reference
        let here = &challenges[at.0];
        println!("{}\n{}", here.name, here.desc);
        if end_challenges.contains(&at) {
            break;
        }
        loop {
            print!("What will you do?\n> ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            // give option to leave game
            if input.eq("exit") || input.eq("quit") {
                println!("Exiting the game. Goodbye!");
                return;
            }

            if let Some(path) = here.paths.iter().find(|d| d.triggers.iter().any(|t| *t == input)) {
                if let Some(msg) = &path.message {
                    println!("{}", msg);
                }
                at = path.target;
                break;
            } else {
                println!("You can't do that!");
            }
        }
    }
}