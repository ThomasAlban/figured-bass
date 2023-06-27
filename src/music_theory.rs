use std::fmt::Display;

// const MAJOR_SCALE: [u32; 6] = [2, 2, 1, 2, 2, 2];
const MAJOR_SCALE: [u32; 7] = [0, 2, 4, 5, 7, 9, 11];
const MINOR_SCALE: [u32; 7] = [0, 2, 3, 5, 7, 8, 10];

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}
impl NoteName {
    fn get_semitone(self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }
    fn from_semitones(semitones: u32) -> Option<Self> {
        let semitones = semitones % 12;
        match semitones {
            0 => Some(Self::C),
            2 => Some(Self::D),
            4 => Some(Self::E),
            5 => Some(Self::F),
            7 => Some(Self::G),
            9 => Some(Self::A),
            11 => Some(Self::B),
            _ => None,
        }
    }
}
impl From<NoteName> for &str {
    fn from(value: NoteName) -> Self {
        match value {
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
            NoteName::A => "A",
            NoteName::B => "B",
        }
    }
}
impl TryFrom<&str> for NoteName {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok(NoteName::C),
            "D" => Ok(NoteName::D),
            "E" => Ok(NoteName::E),
            "F" => Ok(NoteName::F),
            "G" => Ok(NoteName::G),
            "A" => Ok(NoteName::A),
            "B" => Ok(NoteName::B),
            _ => Err(()),
        }
    }
}
impl From<NoteName> for i32 {
    fn from(value: NoteName) -> Self {
        match value {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }
}
impl TryFrom<u32> for NoteName {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NoteName::C),
            1 => Ok(NoteName::D),
            2 => Ok(NoteName::E),
            3 => Ok(NoteName::F),
            4 => Ok(NoteName::G),
            5 => Ok(NoteName::A),
            6 => Ok(NoteName::B),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Accidental {
    Flat = -1,
    Sharp = 1,
}
impl Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Accidental::Flat => "b",
            Accidental::Sharp => "#",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub name: NoteName,
    pub accidental: Option<Accidental>,
}
impl Note {
    pub fn new(name: NoteName, accidental: Option<Accidental>) -> Self {
        let note = Self { name, accidental };
        // if the enharmonic equivalent is better, use that instead
        let enharmonic_equivalent = note.enharmonic_equivalent();
        if enharmonic_equivalent.accidental.is_none() {
            enharmonic_equivalent
        } else {
            note
        }
    }
    pub fn to_semitones(self) -> u32 {
        let mut semitones: i32 = 0;
        semitones += self.name.get_semitone();
        if let Some(accidental) = self.accidental {
            semitones += accidental as i32;
        }
        semitones as u32
    }
    pub fn from_semitones(semitones: u32, accidental: Accidental) -> Self {
        let semitones = semitones % 12;
        let (name, accidental) = match NoteName::from_semitones(semitones) {
            Some(name) => (name, None),
            None => match accidental {
                Accidental::Sharp => (
                    NoteName::from_semitones(semitones - 1).unwrap(),
                    Some(Accidental::Sharp),
                ),
                Accidental::Flat => (
                    NoteName::from_semitones(semitones + 1).unwrap(),
                    Some(Accidental::Flat),
                ),
            },
        };
        Self { name, accidental }
    }
    pub fn semitones_between(&self, other: Note) -> u32 {
        ((self.to_semitones() as i32) - (other.to_semitones() as i32)).unsigned_abs()
    }
    pub fn enharmonic_equivalent(self) -> Self {
        match self.accidental {
            Some(accidental) => match accidental {
                Accidental::Sharp => Note::from_semitones(self.to_semitones(), Accidental::Flat),
                Accidental::Flat => Note::from_semitones(self.to_semitones(), Accidental::Sharp),
            },
            None => self,
        }
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note_name: &str = self.name.into();
        let result = match self.accidental {
            Some(accidental) => note_name.to_owned() + accidental.to_string().as_str(),
            None => note_name.to_owned(),
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoteWithOctave {
    pub note: Note,
    pub octave: u32,
}
impl NoteWithOctave {
    pub fn new(name: NoteName, accidental: Option<Accidental>, octave: u32) -> Self {
        let note = Note::new(name, accidental);
        Self { note, octave }
    }
    pub fn to_semitones(self) -> u32 {
        self.note.to_semitones() + (self.octave * 12)
    }
    pub fn from_semitones(semitones: u32, accidental: Accidental) -> Self {
        let note = Note::from_semitones(semitones, accidental);
        let octave = semitones / 12;
        Self { note, octave }
    }
    pub fn from_note(note: Note, octave: u32) -> Self {
        Self { note, octave }
    }
    pub fn semitones_between(&self, other: NoteWithOctave) -> u32 {
        ((self.to_semitones() as i32) - (other.to_semitones() as i32)).unsigned_abs()
    }
    pub fn semitones_up_to(&self, other: NoteWithOctave) -> i32 {
        (other.to_semitones() as i32) - (self.to_semitones() as i32)
    }
    pub fn in_range_inclusive(&self, lb: NoteWithOctave, ub: NoteWithOctave) -> bool {
        lb.semitones_up_to(*self) >= 0 && self.semitones_up_to(ub) >= 0
    }
}
impl Display for NoteWithOctave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tonality {
    Major,
    Minor,
}

#[derive(Debug, Clone, Copy)]
pub struct KeySignature {
    pub accidental: Accidental,
    pub number: u32,
    pub tonality: Tonality,
}
impl KeySignature {
    #[allow(clippy::manual_map)]
    pub fn from_note(
        note_name: NoteName,
        accidental: Option<Accidental>,
        tonality: Tonality,
    ) -> Option<Self> {
        let note = Note::new(note_name, accidental);
        let major_sharp_keys = vec![
            Note::new(NoteName::C, None),
            Note::new(NoteName::G, None),
            Note::new(NoteName::D, None),
            Note::new(NoteName::A, None),
            Note::new(NoteName::E, None),
            Note::new(NoteName::B, None),
            Note::new(NoteName::F, Some(Accidental::Sharp)),
        ];
        let minor_sharp_keys = vec![
            Note::new(NoteName::A, None),
            Note::new(NoteName::E, None),
            Note::new(NoteName::B, None),
            Note::new(NoteName::F, Some(Accidental::Sharp)),
            Note::new(NoteName::C, Some(Accidental::Sharp)),
            Note::new(NoteName::G, Some(Accidental::Sharp)),
            Note::new(NoteName::D, Some(Accidental::Sharp)),
        ];
        let major_flat_keys = vec![
            Note::new(NoteName::C, None),
            Note::new(NoteName::F, None),
            Note::new(NoteName::B, Some(Accidental::Flat)),
            Note::new(NoteName::E, Some(Accidental::Flat)),
            Note::new(NoteName::A, Some(Accidental::Flat)),
            Note::new(NoteName::D, Some(Accidental::Flat)),
            Note::new(NoteName::G, Some(Accidental::Flat)),
        ];
        let minor_flat_keys = vec![
            Note::new(NoteName::A, None),
            Note::new(NoteName::D, None),
            Note::new(NoteName::G, None),
            Note::new(NoteName::C, None),
            Note::new(NoteName::F, None),
            Note::new(NoteName::B, Some(Accidental::Flat)),
            Note::new(NoteName::E, Some(Accidental::Flat)),
        ];
        match tonality {
            Tonality::Major => {
                if let Some(pos) = major_sharp_keys.iter().position(|&x| x == note) {
                    Some(Self {
                        accidental: Accidental::Sharp,
                        number: pos as u32,
                        tonality: Tonality::Major,
                    })
                } else if let Some(pos) = major_flat_keys.iter().position(|&x| x == note) {
                    Some(Self {
                        accidental: Accidental::Flat,
                        number: pos as u32,
                        tonality: Tonality::Major,
                    })
                } else {
                    None
                }
            }
            Tonality::Minor => {
                if let Some(pos) = minor_sharp_keys.iter().position(|&x| x == note) {
                    Some(Self {
                        accidental: Accidental::Sharp,
                        number: pos as u32,
                        tonality: Tonality::Minor,
                    })
                } else if let Some(pos) = minor_flat_keys.iter().position(|&x| x == note) {
                    Some(Self {
                        accidental: Accidental::Flat,
                        number: pos as u32,
                        tonality: Tonality::Minor,
                    })
                } else {
                    return None;
                }
            }
        }
    }
    pub fn get_starting_note(&self) -> Note {
        let cycle_direction = if self.accidental == Accidental::Sharp {
            7
        } else {
            5
        };
        match self.tonality {
            Tonality::Major => Note::from_semitones(self.number * cycle_direction, self.accidental),
            Tonality::Minor => {
                Note::from_semitones(self.number * cycle_direction - 3, self.accidental)
            }
        }
    }
    pub fn get_scale(&self) -> Vec<Note> {
        let mut semitones = Vec::with_capacity(7);
        for i in 0..7 {
            if self.tonality == Tonality::Major {
                semitones.push(MAJOR_SCALE[i] + self.get_starting_note().to_semitones());
            } else {
                semitones.push(MINOR_SCALE[i] + self.get_starting_note().to_semitones());
            }
        }
        semitones
            .iter()
            .map(|x| Note::from_semitones(*x, self.accidental))
            .collect()
    }
}

pub struct Figure {
    pub bass: NoteWithOctave,
    pub figures: Vec<(u32, Option<Accidental>)>,
}
impl Figure {
    pub fn new(
        name: NoteName,
        accidental: Option<Accidental>,
        octave: u32,
        figures: Vec<(u32, Option<Accidental>)>,
    ) -> Self {
        // order the figures from low to high
        let mut figures = figures;
        figures.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Self {
            bass: NoteWithOctave::new(name, accidental, octave),
            figures,
        }
    }
    pub fn to_notes(&self, key_sig: KeySignature) -> Vec<Note> {
        let mut chord = vec![self.bass.note];
        let mut scale = key_sig.get_scale();
        // put the bass note at the front of the vector
        while scale[0].name != self.bass.note.name {
            scale.rotate_left(1);
        }
        for figure in self.figures.iter() {
            // go however many notes up the scale to get to the correct note
            let mut note = scale[((figure.0 - 1) % 8) as usize];
            // if the figure has an accidental, shift it up or down by 1
            if let Some(accidental) = figure.1 {
                note = match accidental {
                    Accidental::Sharp => {
                        Note::from_semitones(note.to_semitones() + 1, Accidental::Sharp)
                    }
                    Accidental::Flat => {
                        Note::from_semitones(note.to_semitones() + 11, Accidental::Flat)
                    }
                };
            }
            chord.push(note);
        }
        chord
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Chord {
    pub s: NoteWithOctave,
    pub a: NoteWithOctave,
    pub t: NoteWithOctave,
    pub b: NoteWithOctave,
}
impl Chord {
    pub fn new(s: NoteWithOctave, a: NoteWithOctave, t: NoteWithOctave, b: NoteWithOctave) -> Self {
        Self { s, a, t, b }
    }
}
