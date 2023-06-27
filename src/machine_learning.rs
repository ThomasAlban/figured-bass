use crate::music_theory::*;
use rand::Rng;

#[derive(Clone)]
pub struct Realisation {
    pub chords: Vec<Chord>,
    pub score: i32,
}
impl Realisation {
    // generates random voicings from the collections of notes and the bassline
    fn rand_note(lb: &NoteWithOctave, possibles: &Vec<Note>) -> NoteWithOctave {
        let note_index = rand::thread_rng().gen_range(0..possibles.len());
        let note = possibles[note_index];
        let mut octave = lb.octave;
        if ((NoteWithOctave::from_note(note, octave).to_semitones() as i32)
            - lb.to_semitones() as i32)
            < 0
        {
            octave += 1;
        }
        NoteWithOctave::from_note(note, octave)
    }

    fn generate_chord(chord_input: &(NoteWithOctave, Vec<Note>)) -> Chord {
        'finding_chord: loop {
            let b = chord_input.0;
            let t = Self::rand_note(&b, &chord_input.1);
            let a = Self::rand_note(&t, &chord_input.1);
            let s = Self::rand_note(&a, &chord_input.1);

            // all the notes in the figures in the parts
            for possibility in chord_input.1.iter() {
                if !(s.note == *possibility
                    || a.note == *possibility
                    || t.note == *possibility
                    || b.note == *possibility)
                {
                    // dbg!("all the notes");
                    continue 'finding_chord;
                }
            }

            // all parts in range
            let sr = s.in_range_inclusive(
                NoteWithOctave::new(NoteName::C, None, 4),
                NoteWithOctave::new(NoteName::G, None, 5),
            );
            let ar = a.in_range_inclusive(
                NoteWithOctave::new(NoteName::G, None, 3),
                NoteWithOctave::new(NoteName::C, None, 5),
            );
            let tr = t.in_range_inclusive(
                NoteWithOctave::new(NoteName::C, None, 3),
                NoteWithOctave::new(NoteName::G, None, 4),
            );
            if !sr || !ar || !tr {
                // dbg!("all parts in range");
                continue 'finding_chord;
            }
            // we have now found a valid chord, so break
            break Chord::new(s, a, t, b);
        }
    }

    pub fn score(chords: &[Chord]) -> i32 {
        let mut score = 0;
        let mut prev_chord: Option<Chord> = None;
        for chord in chords.iter() {
            // penalise doubling notes in parts
            if chord.b == chord.t || chord.t == chord.a || chord.a == chord.s {
                score += -25;
            }

            if let Some(prev_chord) = prev_chord {
                let s_interval = prev_chord.s.semitones_between(chord.s) as i32;
                let a_interval = prev_chord.a.semitones_between(chord.a) as i32;
                let t_interval = prev_chord.t.semitones_between(chord.t) as i32;

                // give points for small intervals, deduct points for larger ones
                score += (s_interval * -10) + 40;
                score += (a_interval * -5) + 20;
                score += (t_interval * -5) + 20;

                // penalise parallel octaves
                let b_a =
                    (prev_chord.b.note == prev_chord.a.note) && (chord.b.note == chord.a.note);
                let b_s =
                    (prev_chord.b.note == prev_chord.s.note) && (chord.b.note == chord.s.note);
                let b_t =
                    (prev_chord.b.note == prev_chord.t.note) && (chord.b.note == chord.t.note);
                let t_a =
                    (prev_chord.t.note == prev_chord.a.note) && (chord.t.note == chord.a.note);
                let t_s =
                    (prev_chord.t.note == prev_chord.s.note) && (chord.t.note == chord.s.note);
                let a_s =
                    (prev_chord.a.note == prev_chord.s.note) && (chord.a.note == chord.s.note);
                if b_t || b_a || b_s || t_a || t_s || a_s {
                    score += -100;
                }

                // penalise parallel 5ths
                let b_t = (prev_chord.b.semitones_up_to(prev_chord.t) == 7)
                    && chord.b.semitones_up_to(chord.t) == 7;
                let b_a = (prev_chord.b.semitones_up_to(prev_chord.a) == 7)
                    && chord.b.semitones_up_to(chord.a) == 7;
                let b_s = (prev_chord.b.semitones_up_to(prev_chord.s) == 7)
                    && chord.b.semitones_up_to(chord.s) == 7;
                let t_a = (prev_chord.t.semitones_up_to(prev_chord.a) == 7)
                    && chord.t.semitones_up_to(chord.a) == 7;
                let t_s = (prev_chord.t.semitones_up_to(prev_chord.s) == 7)
                    && chord.t.semitones_up_to(chord.s) == 7;
                let a_s = (prev_chord.a.semitones_up_to(prev_chord.s) == 7)
                    && chord.a.semitones_up_to(chord.s) == 7;
                if b_t || b_a || b_s || t_a || t_s || a_s {
                    score += -100;
                }
            }

            if !chord.s.in_range_inclusive(
                NoteWithOctave::new(NoteName::E, None, 4),
                NoteWithOctave::new(NoteName::E, None, 5),
            ) {
                score += -5;
            }
            if !chord.a.in_range_inclusive(
                NoteWithOctave::new(NoteName::B, None, 3),
                NoteWithOctave::new(NoteName::B, None, 4),
            ) {
                score += -5;
            }
            if !chord.t.in_range_inclusive(
                NoteWithOctave::new(NoteName::G, None, 3),
                NoteWithOctave::new(NoteName::G, None, 4),
            ) {
                score += -5;
            }

            prev_chord = Some(*chord);
        }
        score
    }

    pub fn new(chords_input: &[(NoteWithOctave, Vec<Note>)]) -> Self {
        let mut chords: Vec<Chord> = Vec::with_capacity(chords_input.len());

        for chord_input in chords_input.iter() {
            let chord = Self::generate_chord(chord_input);
            chords.push(chord);
        }
        let score = Self::score(&chords);

        Self { chords, score }
    }
    // randomly changes the realisation
    pub fn mutate(&self, chords_input: &[(NoteWithOctave, Vec<Note>)]) -> Self {
        let mut realisation = self.clone();
        // generate a new chord 3 times
        let mut rand_indexes = Vec::with_capacity(3);
        for _ in 0..3 {
            loop {
                let rand_index = rand::thread_rng().gen_range(0..realisation.chords.len());
                if !rand_indexes.contains(&rand_index) {
                    rand_indexes.push(rand_index);
                    break;
                }
            }
        }
        for i in rand_indexes {
            realisation.chords[i] = Self::generate_chord(&chords_input[i])
        }
        // re-score now that we've changed stuff
        realisation.score = Self::score(&realisation.chords);

        realisation
    }
}

#[derive(Clone)]
pub struct Generation {
    pub realisations: Vec<Realisation>,
}
impl Generation {
    fn new(
        population_size: u32,
        non_mutated_percentage: f32,
        mutate_thrice_percentage: f32,
        mutate_twice_percentage: f32,
        chords_input: &[(NoteWithOctave, Vec<Note>)],
        prev_generation: &Option<Generation>,
    ) -> Self {
        let mut realisations = Vec::with_capacity(population_size as usize);

        if let Some(prev_generation) = prev_generation {
            // sort the previous generation by their score
            let mut prev_generation = prev_generation.clone();
            prev_generation
                .realisations
                .sort_by(|a, b| a.score.cmp(&b.score));

            let mutate_thrice_number = (population_size as f32 * mutate_thrice_percentage) as u32;
            let mutate_twice_number = (population_size as f32 * mutate_twice_percentage) as u32;
            let non_mutated_number = (population_size as f32 * non_mutated_percentage) as u32;

            assert!(
                (mutate_thrice_number * 3 + mutate_twice_number * 2 + non_mutated_number)
                    < population_size
            );

            for i in (prev_generation.realisations.len() - non_mutated_number as usize)
                ..(prev_generation.realisations.len())
            {
                realisations.push(prev_generation.realisations[i].clone());
            }

            for _ in 0..mutate_thrice_number {
                let realisation = prev_generation.realisations.pop().unwrap();
                for _ in 0..3 {
                    let mutated = realisation.mutate(chords_input);
                    realisations.push(mutated);
                }
            }

            for _ in 0..mutate_twice_number {
                let realisation = prev_generation.realisations.pop().unwrap();
                for _ in 0..2 {
                    let mutated = realisation.mutate(chords_input);
                    realisations.push(mutated);
                }
            }

            let remaining_size =
                population_size - (mutate_thrice_number * 3 + mutate_twice_number * 2);

            for _ in 0..remaining_size {
                let realisation = Realisation::new(chords_input);
                realisations.push(realisation);
            }
        } else {
            // if there is no previous generation, then just create new realisations
            for _ in 0..population_size {
                let realisation = Realisation::new(chords_input);
                realisations.push(realisation);
            }
        }

        Self { realisations }
    }
}

pub struct MachineLearning {
    total_generations: u32,
    population_size: u32,

    non_mutated_percentage: f32,
    mutate_thrice_percentage: f32,
    mutate_twice_percentage: f32,

    current_generation: Option<Generation>,
}
impl MachineLearning {
    pub fn new(
        total_generations: u32,
        population_size: u32,
        non_mutated_percentage: f32,
        mutate_thrice_percentage: f32,
        mutate_twice_percentage: f32,
    ) -> Self {
        Self {
            total_generations,
            population_size,

            non_mutated_percentage,
            mutate_thrice_percentage,
            mutate_twice_percentage,

            current_generation: None,
        }
    }
    pub fn start(&mut self, chords_input: &[(NoteWithOctave, Vec<Note>)]) -> Realisation {
        for i in 0..self.total_generations {
            let current_generation = Generation::new(
                self.population_size,
                self.non_mutated_percentage,
                self.mutate_thrice_percentage,
                self.mutate_twice_percentage,
                chords_input,
                &self.current_generation,
            );
            self.current_generation = Some(current_generation);

            let generation = self.current_generation.clone().unwrap();
            let mut best_index = 0;
            for i in 0..(self.population_size as usize) {
                if generation.realisations[i].score > generation.realisations[best_index].score {
                    best_index = i;
                }
            }
            println!(
                "current generation: {i}, best score: {}",
                generation.realisations[best_index].score
            );
        }
        let generation = self.current_generation.clone().unwrap();
        let mut best_index = 0;
        for i in 0..(self.population_size as usize) {
            if generation.realisations[i].score > generation.realisations[best_index].score {
                best_index = i;
            }
        }
        generation.realisations[best_index].clone()
    }
}
