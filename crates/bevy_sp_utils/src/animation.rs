use bevy::prelude::*;

/// Adds a system to animate sprites based on their `AnimationSequence` and `AnimationTimer`.
pub fn plugin(app: &mut App) {
    app.add_systems(Update, animate_sprite);
}

#[derive(Bundle, Clone, Default)]
pub struct AnimatedSpriteBundle {
    sequence: AnimationSequence,
    timer: AnimationTimer,
    atlas: TextureAtlas,
}

#[derive(Component, Clone)]
pub enum AnimationSequence {
    Loop {
        first: usize,
        last: usize,
    },
    Mirror {
        first: usize,
        last: usize,
        forward: bool,
    },
    Sequence {
        index: usize,
        sequence: Vec<usize>,
    },
    SequenceMirror {
        index: usize,
        sequence: Vec<usize>,
        forward: bool,
    },
}

impl Default for AnimationSequence {
    fn default() -> Self {
        AnimationSequence::Loop { first: 0, last: 0 }
    }
}

impl AnimationSequence {
    pub fn loop_forwards(first: usize, last: usize) -> Self {
        AnimationSequence::Loop { first, last }
    }

    pub fn mirror(first: usize, last: usize) -> Self {
        AnimationSequence::Mirror {
            first,
            last,
            forward: true,
        }
    }

    pub fn sequence(sequence: Vec<usize>) -> Self {
        AnimationSequence::Sequence { index: 0, sequence }
    }

    pub fn sequence_mirror(sequence: Vec<usize>) -> Self {
        AnimationSequence::SequenceMirror {
            index: 0,
            sequence,
            forward: true,
        }
    }

    pub fn start_index(&self) -> usize {
        match self {
            AnimationSequence::Loop { first, .. } => *first,
            AnimationSequence::Mirror { first, .. } => *first,
            AnimationSequence::Sequence { sequence, .. } => {
                sequence.get(0).map(|x| *x).unwrap_or_default()
            }
            AnimationSequence::SequenceMirror { sequence, .. } => {
                sequence.get(0).map(|x| *x).unwrap_or_default()
            }
        }
    }

    pub fn next_index(&mut self, atlas_index: usize) -> usize {
        match self {
            AnimationSequence::Loop { first, last } => {
                if atlas_index == *last {
                    *first
                } else {
                    atlas_index + 1
                }
            }
            AnimationSequence::Mirror {
                first,
                last,
                forward,
            } => {
                if *forward && atlas_index == *last {
                    *forward = false;
                } else if !*forward && atlas_index == *first {
                    *forward = true;
                }

                if *forward {
                    atlas_index + 1
                } else {
                    atlas_index - 1
                }
            }
            AnimationSequence::Sequence { index, sequence } => {
                if *index == sequence.len() - 1 {
                    *index = 0;
                } else {
                    *index += 1;
                }
                sequence[*index]
            }
            AnimationSequence::SequenceMirror {
                index,
                sequence,
                forward,
            } => {
                if *forward && *index == sequence.len() - 1 {
                    *forward = false;
                } else if !*forward && *index == 0 {
                    *forward = true;
                }

                if *forward {
                    *index += 1;
                } else {
                    *index -= 1;
                }
                sequence.get(*index).map(|x| *x).unwrap_or_default()
            }
        }
    }
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationSequence,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (mut seq, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = seq.next_index(atlas.index);
        }
    }
}
