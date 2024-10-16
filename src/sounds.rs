use crate::prelude::*;

#[derive(Resource, Deref)]
pub struct FireLaserSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
pub struct InvaderKilledSound(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
pub struct InvaderNote0(pub Handle<AudioSource>);

#[derive(Resource, Deref)]
pub struct InvaderNote1(pub Handle<AudioSource>);
#[derive(Resource, Deref)]
pub struct InvaderNote2(pub Handle<AudioSource>);
#[derive(Resource, Deref)]
pub struct InvaderNote3(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct CurrentNoteIndex(u8);

pub fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fire_laser_sound_handle = asset_server.load("sounds/shoot.ogg");
    commands.insert_resource(FireLaserSound(fire_laser_sound_handle));

    let invader_killed_sound_handle = asset_server.load("sounds/invaderkilled.ogg");
    commands.insert_resource(InvaderKilledSound(invader_killed_sound_handle));

    let note_0_handle: Handle<AudioSource> = asset_server.load("sounds/fastinvader0.ogg");
    commands.insert_resource(InvaderNote0(note_0_handle));
    let note_1_handle: Handle<AudioSource> = asset_server.load("sounds/fastinvader1.ogg");
    commands.insert_resource(InvaderNote1(note_1_handle));
    let note_2_handle: Handle<AudioSource> = asset_server.load("sounds/fastinvader2.ogg");
    commands.insert_resource(InvaderNote2(note_2_handle));
    let note_3_handle: Handle<AudioSource> = asset_server.load("sounds/fastinvader3.ogg");
    commands.insert_resource(InvaderNote3(note_3_handle));

    commands.insert_resource(CurrentNoteIndex(0));
}

pub fn play_invader_sound(
    mut commands: Commands,
    note_0: Res<InvaderNote0>,
    note_1: Res<InvaderNote1>,
    note_2: Res<InvaderNote2>,
    note_3: Res<InvaderNote3>,
    mut current_note_index: ResMut<CurrentNoteIndex>,
) {
    let sound = match current_note_index.0 {
        0 => note_0.clone(),
        1 => note_1.clone(),
        2 => note_2.clone(),
        _ => note_3.clone(),
    };

    if current_note_index.0 < 3 {
        current_note_index.0 += 1;
    } else {
        current_note_index.0 = 0;
    }

    commands.spawn(AudioBundle {
        source: sound,
        settings: PlaybackSettings::DESPAWN,
    });
}
