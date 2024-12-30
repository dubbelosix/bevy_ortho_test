use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

const MECH_PATH: &str = "med_mech_striker/scene.gltf";

#[derive(Resource)]
struct MyAnimationGraph(Handle<AnimationGraph>, AnimationNodeIndex);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs_assets: ResMut<Assets<AnimationGraph>>,
    mut images: ResMut<Assets<Image>>,
) {

    let mut animation_graph = AnimationGraph::new();
    let node_index = animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(MECH_PATH)),
        1.0,
        animation_graph.root,
    );

    // Render target things
    let render_target = create_render_target();
    let render_target_handle = images.add(render_target);

    // Camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            target: RenderTarget::Image(render_target_handle.clone()),
            order: 1,
            ..Default::default()
        },
        OrthographicProjection {
            scale: 10.0,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        },
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn(
        (
            DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y))
    );

    commands.spawn((
        Camera2d,
        Camera{order: 0,..Default::default()})
    );

    commands.spawn(
        (
        Sprite {
            image: render_target_handle,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let handle = animation_graphs_assets.add(animation_graph);

    commands.spawn(
        (
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(MECH_PATH))),
            Transform::from_scale(Vec3::splat(0.20)),
            AnimationPlayer::default(),
        ),
    );

    commands.insert_resource(MyAnimationGraph(handle,node_index));

}


fn create_render_target() -> Image {
    let size = Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };

    let mut image = Image::new_fill(size, TextureDimension::D2, &[0, 0, 0, 0], TextureFormat::Rgba8UnormSrgb,
                                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    image.sampler = ImageSampler::nearest();
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;

    image
}

fn play_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationPlayer)>,
    my_animation_graph: Res<MyAnimationGraph>,
) {
    for (entity, mut player) in &mut query {
        commands.entity(entity).insert(AnimationGraphHandle(my_animation_graph.0.clone()));
        player.play(my_animation_graph.1).repeat();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, play_animation)
        .run();
}