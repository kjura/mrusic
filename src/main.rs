use std::collections::HashMap;

fn main() {

    let transposition_factor = 5;

    // we want -5 semitones dropped from A minor -> E minor
    let mapping = HashMap::from([
        ("C", 0),
        ("C#", 1),
        ("D", 2),
        ("D#", 3),
        ("E", 4),
        ("F", 5),
        ("F#", 6),
        ("G", 7),
        ("G#", 8),
        ("A", 9),
        ("A#", 10),
        ("B", 11)
    ]);


    let lyrics = vec![
        "On a cold winter morning -> E E E D C D E",
        "in the time before the light -> A B D E D C C A A G",
        "In flames of death's eternal reign -> C D E D C C D E D C",
        "we ride towards the fight -> C D E D C C A",
        "When the darkness has fallen down -> E E E D C D C D E",
        "and the times are tough all right -> A B D E D D A A G" ,
        "The sound of evil laughter falls -> C D E D C C D E D C",
        "around the world tonight -> C D E D C C A A",
    ];

    // let mut output: Vec<String> = Vec::new();

    for verse in lyrics {

        let splitted = verse.split("-> ");
        let mut notes_vec = Vec::new();
        let mut new_notes_vec = Vec::new();

        for part in splitted.skip(1) {

            let notes = part.split(" ");

            for note in notes {
                // println!("{:?}", note);
                notes_vec.push(note); // -> vec!["E", "E", "E", "D", "C", "D", "E"]
            }

            for note in &mut notes_vec {
                

                let original_note_number = mapping.get(note).unwrap();
                let difference_number: i32 =  original_note_number - transposition_factor;
                let transposed_note_number = (difference_number).rem_euclid(12);
                let transposed_note = mapping.iter().find_map(|(key, &val)| if val == transposed_note_number { Some(*key) } else { None }).unwrap();

                new_notes_vec.push(transposed_note);
                
            }

            let finished_line: Vec<String> = new_notes_vec.iter().copied().map(|s| s.to_string()).collect();
            let rewritten_notes_line = finished_line.join(" ");

            for finisher in verse.split("-> ") {
                println!("{} {}", finisher, rewritten_notes_line);
                break;
            }

            
        }

    }

    //println!("{:?}", output);


}


