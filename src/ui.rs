use std::cmp;

use bevy::{
    color::palettes::{
        css::{BLACK, GREEN, WHITE},
        tailwind::GRAY_500,
    },
    ecs::query::QueryData,
    prelude::*,
};

#[derive(Event, Default, Debug)]
pub struct UiInputsEvent {
    pub energy_level: String,
}

#[derive(Resource)]
pub struct UiInputEntities {
    pub energy_level: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct EnergyLevel(pub u32);

#[derive(Component, Default, QueryData)]
pub struct EnergyLabelMarker;
#[derive(Component, Default)]
pub struct EnergyLevelPlusMarker;
#[derive(Component, Default)]
pub struct EnergyLevelMinusMarker;

/// adds right column with ui elements to scene
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(130.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });

    let root_id = root.id();

    add_header(&mut commands, root_id, &font, "Potential model:");
    add_button(
        &mut commands,
        root_id,
        &font,
        "Infinite well",
        InfiniteWellModelMarker,
    );
    add_button(
        &mut commands,
        root_id,
        &font,
        "Harmonic oscillator",
        HarmonicOscillatorModelMarker,
    );

    add_spacer(&mut commands, root_id);

    add_header(&mut commands, root_id, &font, "Energy level:");

    let init_energy_level = EnergyLevel(1);
    let energy_value_label =
        add_energy_level_value_row(&mut commands, &font, root_id, init_energy_level);
    commands.spawn(init_energy_level);

    commands.insert_resource(UiInputEntities {
        energy_level: energy_value_label,
    });

    commands.insert_resource(PotentialModelInput::InfiniteWell);

    add_legend_box(&mut commands, &font);

    add_control_info_labels(commands, &font);
}

/// adds component to set energy level
/// returns the label (entity) with the numeric value
pub fn add_energy_level_value_row(
    commands: &mut Commands,
    font: &Handle<Font>,
    root_id: Entity,
    init_energy_level: EnergyLevel,
) -> Entity {
    let row = NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Row,
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            ..default()
        },
        ..default()
    };

    let row_id = commands.spawn(row).id();
    commands.entity(root_id).push_children(&[row_id]);

    let energy_level_value_entity = add_button_label_with_marker(
        commands,
        row_id,
        font,
        &init_energy_level.0.to_string(),
        EnergyLabelMarker,
    );

    add_square_button(commands, row_id, font, "-", EnergyLevelMinusMarker);
    add_square_button(commands, row_id, font, "+", EnergyLevelPlusMarker);

    energy_level_value_entity
}

/// adds a generic vertical spacer element with fixed height
fn add_spacer(commands: &mut Commands, root_id: Entity) {
    let spacer_id = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .id();
    commands.entity(root_id).push_children(&[spacer_id]);
}

/// generates a column header styled text
pub fn generate_header(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
            margin: UiRect {
                bottom: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}

/// adds a label with a given marker
/// used for when we want to change the label dynamically
// is this specific to buttons? needs more generic name I think
pub fn add_button_label_with_marker<T>(
    commands: &mut Commands,
    row_id: Entity,
    font: &Handle<Font>,
    label: &str,
    marker: T,
) -> Entity
where
    T: Component,
{
    let label = generate_button_label(font, label);
    let spawned_label = commands.spawn((marker, label)).id();
    commands.entity(row_id).push_children(&[spawned_label]);
    spawned_label
}

/// generates a text label
/// meant to be added to a button (button-related dimensions)
// this obviously needs improvement, we should have a button component etc..
// but bevy's ui is very wip currently so keeping implementation low effort
pub fn generate_button_label(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(10.0),
            width: Val::Px(30.0),
            height: Val::Auto,
            align_self: AlignSelf::Center,
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}

/// generates a text label displayed on the bottom left corner of the window
pub fn generate_legend(font: &Handle<Font>, label: &str, color: impl Into<Color>) -> impl Bundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            left: Val::Px(0.0),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: color.into(),
            },
        ),
        ..default()
    }
}

/// adds container element and legends to bottom left corner of window
pub fn add_legend_box(commands: &mut Commands, font: &Handle<Font>) -> Entity {
    let row = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            left: Val::Px(10.0),
            bottom: Val::Px(0.0),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        ..default()
    };

    let row_id = commands.spawn(row).id();

    add_legend(commands, row_id, &font, "Ψ(x)", WHITE);
    add_legend(commands, row_id, &font, "|Ψ(x)|^2", GRAY_500);

    row_id
}

/// adds legend to container
pub fn add_legend(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
    color: impl Into<Color>,
) -> Entity {
    let bundle = generate_legend(font, label, color);
    let entity = commands.spawn(bundle).id();
    commands.entity(container_id).push_children(&[entity]);
    entity
}

/// adds button to container
pub fn add_button<T>(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
    marker: T,
) where
    T: Component,
{
    let button = commands
        .spawn((
            marker,
            ButtonBundle {
                style: Style {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    // justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    label.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.0,
                        color: WHITE.into(),
                    }
                    .clone(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.entity(container_id).push_children(&[button]);
}

/// adds header to container
pub fn add_header(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
) -> Entity {
    let label = generate_header(font, label);
    let spawned_label = commands.spawn(label).id();
    commands
        .entity(container_id)
        .push_children(&[spawned_label]);
    spawned_label
}

/// adds a square button to container
pub fn add_square_button<T>(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
    marker: T,
) where
    T: Component,
{
    let button = commands
        .spawn((
            marker,
            ButtonBundle {
                style: Style {
                    top: Val::Px(0.0),
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    label.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.0,
                        color: WHITE.into(),
                    }
                    .clone(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.entity(container_id).push_children(&[button]);
}

/// processes the ui events
/// basically, maps events to state
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_ui_inputs(
    mut events: EventReader<UiInputsEvent>,
    mut commands: Commands,
    energy_level_query: Query<Entity, With<EnergyLevel>>,
) {
    for input in events.read() {
        match parse_i32(&input.energy_level) {
            Ok(i) => {
                // ensure only 1 energy level active at a time
                despawn_all_entities(&mut commands, &energy_level_query);
                // spawn new level
                commands.spawn(EnergyLevel(i));
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

pub fn parse_i32(str: &str) -> Result<u32, String> {
    let f = str.parse::<u32>();
    match f {
        Ok(i) => Ok(i),
        Err(e) => Err(format!("Failed to parse u32: {}", e)),
    }
}

/// removes all entities matching a query (1 filter)
pub fn despawn_all_entities<T>(commands: &mut Commands, query: &Query<Entity, With<T>>)
where
    T: Component,
{
    for e in query.iter() {
        let entity = commands.entity(e);
        entity.despawn_recursive();
    }
}

/// removes all entities matching a query (2 filters)
/// TODO refactor with despawn_all_entities? shouldn't have to add more functions for each type parameter..
pub fn despawn_all_entities_tu<T, U>(
    commands: &mut Commands,
    query: &Query<Entity, (With<T>, With<U>)>,
) where
    T: Component,
    U: Component,
{
    for e in query.iter() {
        let entity = commands.entity(e);
        entity.despawn_recursive();
    }
}

/// handles interactions with plus button
/// it updates the button's appearance and sends an event
#[allow(clippy::type_complexity)]
pub fn plus_button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EnergyLevelPlusMarker>),
    >,
    mut my_events: EventWriter<PlusMinusInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        plus_minus_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut my_events,
            PlusMinusInput::Plus,
        );
    }
}

/// handles interactions with minus button
/// it updates the button's appearance and sends an event
#[allow(clippy::type_complexity)]
pub fn minus_button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EnergyLevelMinusMarker>),
    >,
    mut my_events: EventWriter<PlusMinusInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        plus_minus_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut my_events,
            PlusMinusInput::Minus,
        );
    }
}

/// handles interactions with plus or minus button
/// it updates the button's appearance and sends an event
fn plus_minus_button_handler(
    interaction: (&Interaction, &mut BackgroundColor, &mut BorderColor),
    my_events: &mut EventWriter<PlusMinusInputEvent>,
    plus_minus: PlusMinusInput,
) {
    let (interaction, color, border_color) = interaction;
    match *interaction {
        Interaction::Pressed => {
            *color = GREEN.into();
            border_color.0 = GREEN.into();
            println!("sending plus minus event: {:?}", plus_minus);
            my_events.send(PlusMinusInputEvent { plus_minus });
        }
        Interaction::Hovered => {}
        Interaction::None => {
            *color = BLACK.into();
            border_color.0 = BLACK.into();
        }
    }
}

/// handles energy level inputs
/// basically, we listen to clicks on the +/- buttons
/// then query the current energy level, update it, and spawn the new value.
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_energy_level_ui_inputs(
    mut events: EventReader<PlusMinusInputEvent>,
    mut commands: Commands,
    mut energy_level_query: Query<&EnergyLevel>,
    energy_level_entity_query: Query<Entity, With<EnergyLevel>>,
) {
    for input in events.read() {
        for e in energy_level_query.iter_mut() {
            // println!("got energy level: {:?}", e);
            // update
            let current = e.0;
            let increment: i32 = match input.plus_minus {
                PlusMinusInput::Plus => 1,
                PlusMinusInput::Minus => -1,
            };
            let new_i = current as i32 + increment;
            // pressing "-" at 0 stays at 0
            let mut new = cmp::max(0, new_i) as u32;
            // currently no hermitian polynomials for n > 10, and this seems not needed for now anyway
            new = cmp::min(10, new);

            // ensure only one energy level at a time
            despawn_all_entities(&mut commands, &energy_level_entity_query);
            // spawn updated energy level
            let energy_level = EnergyLevel(new);
            commands.spawn(energy_level);
        }
    }
}

/// updates the UI energy level to reflect the current system entity
pub fn update_energy_level_label(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    input_entities: Res<UiInputEntities>,
    mut label_query: Query<(Entity, &mut Text), With<EnergyLabelMarker>>,
) {
    // current energy level
    for energy_level in energy_level_query.iter() {
        // find the UI label
        let entity_id = commands.entity(input_entities.energy_level).id();
        for (entity, mut text) in label_query.iter_mut() {
            if entity == entity_id {
                // update value
                text.sections[0].value = energy_level.0.to_string();
            }
        }
    }
}

/// carried in the "clicked + or -" event
// TODO this probably doesn't need to be a resource
#[derive(Debug, Default, Clone, Copy, Resource)]
pub enum PlusMinusInput {
    #[default]
    Plus,
    Minus,
}

/// event for when user clicked + or - on UI
#[derive(Event, Default, Debug)]
pub struct PlusMinusInputEvent {
    pub plus_minus: PlusMinusInput,
}

/// state for selected model
#[derive(Debug, Default, Clone, Copy, Resource, PartialEq)]
pub enum PotentialModelInput {
    #[default]
    InfiniteWell,
    HarmonicOscillator,
}

/// event triggered when selecting a model on UI
#[derive(Event, Default, Debug)]
pub struct PotentialModelInputEvent {
    pub model: PotentialModelInput,
}

/// bevy marker for infinite well model button
#[derive(Component, Default)]
pub struct InfiniteWellModelMarker;

/// bevy marker for harmonic oscillator model button
#[derive(Component, Default)]
pub struct HarmonicOscillatorModelMarker;

/// handles interactions with model button
/// styles button accordingly and when clicked, triggers an event with the selected input
#[allow(clippy::type_complexity)]
pub fn infinite_well_model_button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<InfiniteWellModelMarker>),
    >,
    mut my_events: EventWriter<PotentialModelInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        potential_model_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut my_events,
            PotentialModelInput::InfiniteWell,
        );
    }
}

/// handles interactions with model button
/// styles button accordingly and when clicked, triggers an event with the selected input
#[allow(clippy::type_complexity)]
pub fn harmonic_oscillator_button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<HarmonicOscillatorModelMarker>),
    >,
    mut my_events: EventWriter<PotentialModelInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        potential_model_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut my_events,
            PotentialModelInput::HarmonicOscillator,
        );
    }
}

/// handles interactions with model button
/// styles button accordingly and when clicked, triggers an event with the selected input
fn potential_model_button_handler(
    interaction: (&Interaction, &mut BackgroundColor, &mut BorderColor),
    my_events: &mut EventWriter<PotentialModelInputEvent>,
    polarity: PotentialModelInput,
) {
    let (interaction, color, border_color) = interaction;
    match *interaction {
        Interaction::Pressed => {
            *color = GREEN.into();
            border_color.0 = GREEN.into();
            my_events.send(PotentialModelInputEvent { model: polarity });
        }
        Interaction::Hovered => {}
        Interaction::None => {
            *color = BLACK.into();
            border_color.0 = BLACK.into();
        }
    }
}

/// basically maps the model selection event to state
#[allow(clippy::too_many_arguments)]
pub fn listen_potential_model_ui_inputs(
    mut events: EventReader<PotentialModelInputEvent>,
    mut model: ResMut<PotentialModelInput>,
) {
    for input in events.read() {
        *model = input.model;
    }
}

/// labels showing panning and zooming keys
fn add_control_info_labels(mut commands: Commands, font: &Handle<Font>) {
    // TODO wrapper component and relative position
    commands.spawn(generate_control_info_label(font, "move right: a", 0.0));
    commands.spawn(generate_control_info_label(font, "move left: d", 20.0));
    commands.spawn(generate_control_info_label(font, "move up: q", 40.0));
    commands.spawn(generate_control_info_label(font, "move down: e", 60.0));
    commands.spawn(generate_control_info_label(font, "zoom in: w", 80.0));
    commands.spawn(generate_control_info_label(font, "zoom out: s", 100.0));
}

fn generate_control_info_label(font: &Handle<Font>, label: &str, top: f32) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(top),
            left: Val::Px(10.0),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}
