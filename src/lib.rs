use bevy::prelude::*;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    #[cfg(feature = "android")]
    {
        println!("android 执行");
        use bevy::window::WindowPlugin;
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "DQWM".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                // 关键：确保没有 winit
                .build(),
        );
        // android-game-activity 会自动注册 Activity 生命周期处理
    }
    #[cfg(feature = "desktop")]
    {
        println!("android 执行");
        app.add_plugins(DefaultPlugins);
    }

    app.insert_resource(Counter(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, counter_text_system))
        .run();
}

#[derive(Resource)]
struct Counter(u32);

#[derive(Component)]
struct CountText;

#[derive(Component)]
struct CountButton;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    CountButton,
                    Node {
                        width: Val::Px(240.),
                        height: Val::Px(100.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderRadius::all(Val::Px(5.)),
                ))
                .with_child((Text::new("Click me"), TextColor(Color::WHITE)));
            parent.spawn((
                Text::new("100"),
                CountText,
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(Val::Px(20.)),
                    ..default()
                },
            ));
        });
}

fn button_system(
    mut interaction: Query<&Interaction, (Changed<Interaction>, With<CountButton>)>,
    mut counter: ResMut<Counter>,
) {
    for i in &mut interaction {
        if *i == Interaction::Pressed {
            counter.0 += 10;
        }
    }
}

fn counter_text_system(counter: Res<Counter>, mut text: Query<&mut Text, With<CountText>>) {
    for mut t in &mut text {
        *t = Text::new(counter.0.to_string());
    }
}
