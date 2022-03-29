mod notes;

use rand::Rng;

pub fn random_exercise() -> (String, String) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..3);
    match random {
        0 => guess_note_by_neck(),
        1 => guess_neck_notes(),
        2 => guess_similar_notes(),
        _ => panic!(),
    }
}

pub fn guess_note_by_neck() -> (String, String) {
    let notes = notes::init_notes();
    let notes_count = notes.len();
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..notes_count);
    let entry = &notes[random];
    let (note, neck_notes) = entry;
    let neck_notes_count = neck_notes.len();
    let random2 = rng.gen_range(0..neck_notes_count);
    let neck_note = &neck_notes[random2];
    let question = format!("Какая это нота: {}?", neck_note.display_full_name());
    let answer = format!("{}, {}", note.name(), note.placement());
    (question, answer)
}

pub fn guess_neck_notes() -> (String, String) {
    let notes = notes::init_notes();
    let notes_count = notes.len();
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..notes_count);
    let entry = &notes[random];
    let (note, neck_notes) = entry;
    let neck_notes_count = neck_notes.len();
    let random2 = rng.gen_range(0..neck_notes_count);
    let neck_note = &neck_notes[random2];
    let question = format!(
        "На каком ладу находится нота {} на {} струне?",
        note.placement(),
        neck_note.string()
    );
    let answer = format!("Нота {}, {}", note.name(), neck_note.display_fret());
    (question, answer)
}

pub fn guess_similar_notes() -> (String, String) {
    let notes = notes::init_notes();
    let notes_count = notes.len();
    let mut rng = rand::thread_rng();
    let entry = loop {
        let random = rng.gen_range(0..notes_count);
        let entry = &notes[random];
        if entry.1.len() > 1 {
            break entry;
        }
    };

    let (note, neck_notes) = entry;
    let neck_notes_count = neck_notes.len();
    let random2 = rng.gen_range(0..neck_notes_count);
    let neck_note = &neck_notes[random2];

    let neck_note2 = loop {
        let random = rng.gen_range(0..neck_notes_count);
        let neck_note2 = &neck_notes[random];
        if !neck_note2.eq(neck_note) {
            break neck_note2;
        }
    };

    let question = format!(
        "{}, где находится на {} струне?",
        neck_note.display_full_name(),
        neck_note2.string()
    );
    let answer = format!(
        "{}, нота {}, {}",
        neck_note2.display_fret(),
        note.name(),
        note.placement()
    );
    (question, answer)
}
