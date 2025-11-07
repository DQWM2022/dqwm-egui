use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "道起微末".into(),
                    name: Some("道起微末".into()),
                    position: WindowPosition::Centered(MonitorSelection::Primary), // 居中
                    resolution: (300, 533).into(),                                 // 窗口大小
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(bottom_navigation()).with_children(|parent| {
        parent.spawn((nav_button("资源"),));
        parent.spawn((nav_button("游戏"),));
        parent.spawn((nav_button("设置"),));
        parent.spawn((nav_button("关于"),));
    });

    // 放一个文本测试
    commands.spawn((
        Text::new("测试"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        Test11,
    ));
}

#[derive(Component)]
struct Test11;

// 实现底部导航

#[derive(Component)]
struct BottomNavigation;

//  底部导航按钮
#[derive(Component)]
struct NavButton;

fn bottom_navigation() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: Val::Auto,
            position_type: PositionType::Absolute, // 关键：绝对定位
            bottom: Val::Px(0.),                   // 贴底
            left: Val::Px(0.),                     // 从左开始
            flex_direction: FlexDirection::Row,    // 关键 1：横向排列
            column_gap: Val::Px(5.0),              // 列间距
            justify_content: JustifyContent::SpaceEvenly, // 关键 2：间距均分
            align_items: AlignItems::Center,       // 垂直居中

            padding: UiRect {
                // 边距
                left: Val::Px(12.0),
                right: Val::Px(12.0),
                top: Val::Px(8.0),
                bottom: Val::Px(8.0),
            },
            ..default()
        },
        BottomNavigation,
        ZIndex(99999),
    )
}

/// 单个子按钮：flex_grow = 1 保证等宽
fn nav_button(text: &str) -> impl Bundle {
    (
        Node {
            flex_grow: 1., // 关键 3：占满剩余空间
            height: percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        children![(
            Text::new(text),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            NavButton
        )],
    )
}

// fn button_system(
//     mut interaction_query: Query<(&Interaction, &mut Button, &Children), Changed<Interaction>>,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (interaction, mut button, children) in &mut interaction_query {
//         let text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 println!("Pressed {}", **text);
//                 button.set_changed();
//             }
//             Interaction::Hovered => {
//                 println!("Hovered{}", **text);
//             }
//             Interaction::None => {
//                 println!("None{}", **text);
//             }
//         }
//     }
// }

fn button_system(
    mut interaction_query: Query<(&Interaction, &mut Button, &Children), Changed<Interaction>>,
    mut label_query: Query<&mut Text, With<Test11>>, // 只改标签
    btn_text_q: Query<&Text, Without<Test11>>,       // 只读按钮文字
) {
    // 先获取标签的可变引用
    let mut label = label_query.single_mut().unwrap();

    for (interaction, mut button, children) in &mut interaction_query {
        // 获取按钮文字的只读引用
        let btn_text = &**btn_text_q.get(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                // 修改标签文字
                **label = format!("按钮说：{}", btn_text);
                button.set_changed();
            }
            Interaction::Hovered => {
                // 悬停时的逻辑
            }
            Interaction::None => {
                // 无交互时的逻辑
            }
        }
    }
}
