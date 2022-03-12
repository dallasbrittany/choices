use std::io;
use std::io::Write;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;

#[derive(Debug)]
struct GameState {
    // story_id: String, // this will be used once we choose between multiple stories by going through the story_maps directory
    link_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoryMap {
    id: String,
    name: String,
    description: String,
    init_link_id: String,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    id: String,
    description: String,
    choices: Vec<Choice>, // empty if an end state
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    description: String,
    options: Vec<LinkProbabilityPair>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinkProbabilityPair {
    link: String,
    probability: String,
}

fn get_story(data: &str) -> Result<StoryMap> {
    let story_map: StoryMap = serde_json::from_str(data)?;

    Ok(story_map)
}

fn find_link(story_links: &Vec<Link>, game_state_link_id: &String) -> Option<usize> {
    story_links.iter().position(|x| &x.id == game_state_link_id)
}

fn get_and_validate_user_input(choices: &Vec<Choice>, valid_choices: &Vec<usize>, ret: &mut String) {
    let mut no_match_yet = true;
    while no_match_yet {
        let choice = get_user_choice();
        let user_selection = valid_choices.iter()
            .position(|x| x == &choice);
        if let Some(i) = user_selection {
            println!("You chose option {}.", i+1);
            no_match_yet = false;

            let mut link_names: Vec<&String> = Vec::new();
            let mut link_probabilities: Vec<u32> = Vec::new();
            for o in &choices[i].options {
                link_names.push(&o.link);
                let val = o.probability.to_string().parse::<u32>().unwrap();
                link_probabilities.push(val);
            }

            let dist = WeightedIndex::<u32>::new(&link_probabilities).unwrap();
            let mut rng = rand::thread_rng();
            let random_choice = dist.sample(&mut rng);
            let calculated_option = &choices[i].options[random_choice];
            *ret = calculated_option.link.clone();
        }
    }
}

fn get_user_choice() -> usize {
    let mut user_input = String::new();
    print!("What do you choose? ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to process user input");
    let choice = user_input.trim().to_string().parse::<usize>();
    user_input.clear();
    let unwrapped = choice.unwrap();
    unwrapped
}

fn show_main_menu() {
    println!("Press enter to begin the story.");
}

fn get_story_file_path(story_file_name: &String) -> String {
    let story_map_dir_name = "story_maps";
    let file_path = format!("{}/{}", story_map_dir_name, story_file_name);
    file_path
}

fn get_story_from_file(data: &String) -> String {
    let story_file_name = get_story_file_path(data);
    let contents = fs::read_to_string(story_file_name)
        .expect("Could not load story");
    contents
}

fn run_story(story: &StoryMap) {
    println!("{}", story.name);
    let mut game_state = GameState {
        // story_id: story.id.to_string(),
        link_id: story.init_link_id.to_string()
    };

    loop {
        let link_id = find_link(&story.links, &game_state.link_id);
        let description = &story.links[link_id.unwrap()].description;
        println!("\n{}\n", description);

        let choices = &story.links[link_id.unwrap()].choices;
        if choices.len() == 0 {
            break;
        }

        let mut valid_choices: Vec<usize> = Vec::new();
        for (idx, choice) in choices.iter().enumerate() {
            println!("{}. {}", idx+1, choice.description);
            valid_choices.push(idx+1);
        }

        get_and_validate_user_input(&choices, &valid_choices, &mut game_state.link_id);
    }
}

fn main() {
    let story_file_name = String::from("vacation_test_story.json");
    let story_text: String = get_story_from_file(&story_file_name);
    let story = get_story(&story_text).unwrap();

    show_main_menu();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to process user input");

    run_story(&story);
}
