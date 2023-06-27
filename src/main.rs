mod machine_learning;
mod music_theory;

use machine_learning::*;
use music_theory::{Accidental::*, NoteName::*, *};

fn print_figured_bass(realisation: Realisation) {
    println!();
    println!();
    println!();

    for chord in realisation.chords.iter() {
        print!("{}   ", chord.s);
        if chord.s.to_string().len() == 2 {
            print!(" ");
        }
    }
    println!();
    for chord in realisation.chords.iter() {
        print!("{}   ", chord.a);
        if chord.a.to_string().len() == 2 {
            print!(" ");
        }
    }
    println!();
    for chord in realisation.chords.iter() {
        print!("{}   ", chord.t);
        if chord.t.to_string().len() == 2 {
            print!(" ");
        }
    }
    println!();
    for chord in realisation.chords.iter() {
        print!("{}   ", chord.b);
        if chord.b.to_string().len() == 2 {
            print!(" ");
        }
    }
    println!();
    println!();
    println!();
}

fn main() {
    // let key_sig_1 = KeySignature::from_note(E, Some(Flat), Tonality::Major).unwrap();
    // let figured_bass_1 = vec![
    //     Figure::new(E, Some(Flat), 3, vec![(5, None), (3, None)]),
    //     Figure::new(F, None, 3, vec![(6, None), (3, None)]),
    //     Figure::new(G, None, 3, vec![(6, None), (3, None)]),
    //     Figure::new(A, Some(Flat), 3, vec![(5, None), (3, None)]),
    //     Figure::new(B, Some(Flat), 3, vec![(6, None), (4, None)]),
    //     Figure::new(B, Some(Flat), 2, vec![(5, None), (3, None)]),
    //     Figure::new(C, None, 3, vec![(5, None), (3, None)]),
    //     Figure::new(G, None, 2, vec![(6, None), (4, None)]),
    //     Figure::new(A, Some(Flat), 2, vec![(6, None), (3, None)]),
    //     Figure::new(E, Some(Flat), 3, vec![(5, None), (3, None)]),
    //     Figure::new(D, None, 3, vec![(6, None), (3, None)]),
    //     Figure::new(C, None, 3, vec![(5, None), (3, None)]),
    //     Figure::new(A, Some(Flat), 2, vec![(6, None), (3, None)]),
    //     Figure::new(B, Some(Flat), 2, vec![(5, None), (3, None)]),
    //     Figure::new(E, Some(Flat), 3, vec![(5, None), (3, None)]),
    // ];

    let key_sig_2 = KeySignature::from_note(G, None, Tonality::Major).unwrap();
    let figured_bass_2 = vec![
        Figure::new(E, None, 3, vec![(5, None), (3, None)]),
        Figure::new(F, Some(Sharp), 3, vec![(6, Some(Sharp)), (3, None)]),
        Figure::new(G, None, 3, vec![(6, None), (3, None)]),
        Figure::new(D, Some(Sharp), 3, vec![(6, None), (3, None)]),
        Figure::new(E, None, 3, vec![(5, None), (3, None)]),
        Figure::new(B, None, 2, vec![(5, None), (3, Some(Sharp))]),
        Figure::new(C, None, 3, vec![(5, None), (3, None)]),
        Figure::new(C, None, 3, vec![(6, None), (3, None)]),
        Figure::new(B, None, 2, vec![(5, None), (3, Some(Sharp))]),
        Figure::new(A, None, 2, vec![(6, None), (3, None)]),
        Figure::new(G, None, 2, vec![(6, None), (3, None)]),
        Figure::new(A, None, 2, vec![(5, None), (3, None)]),
        Figure::new(B, None, 2, vec![(6, None), (4, None)]),
        Figure::new(B, None, 2, vec![(5, None), (3, Some(Sharp))]),
        Figure::new(E, None, 3, vec![(5, None), (3, None)]),
    ];

    let mut chords_input = Vec::with_capacity(figured_bass_2.len());
    for figure in figured_bass_2.iter() {
        let mut notes = Vec::with_capacity(figure.figures.len());
        for note in figure.to_notes(key_sig_2) {
            notes.push(note);
        }
        chords_input.push((figure.bass, notes));
    }

    let mut ml = MachineLearning::new(100, 2000, 0.1, 0.2, 0.1);
    let realisation = ml.start(&chords_input);

    print_figured_bass(realisation);
}
