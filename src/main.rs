use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, collide_aabb::{Collision, collide}}};

const PARTICLE_COLOR:Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR:Color = Color::rgb(0.1, 0.1, 0.1);


// const PHYSICS_STEP:f32 = 1.0/60.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(collision_detection)
        .add_system(update_position)
        .run();
}

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 1..4 {
        // let pos = Vec3
        commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(PARTICLE_COLOR)),
                transform: Transform::from_translation(Vec3::new(5.0 * (i as f32) , 100.0 * (i as f32), 0.)),
                ..default()
            },
            Particle,
            Collider,
            Velocity(Vec2::new(10.0, 0.0)),
        ));
    }
}


fn update_position(mut query: Query<(&mut Velocity, &mut Transform)>){
    for (mut velocity, mut transform) in &mut query{
        transform.translation.x += velocity.x; //* PHYSICS_STEP;
        transform.translation.y += velocity.y; //* PHYSICS_STEP;

        // pull towards center test
        velocity.x -= transform.translation.x / 1000.0;
        velocity.y -= transform.translation.y / 1000.0;
    }
}

fn collision_detection(mut query: Query<(&mut Velocity, &Transform, &Collider)>){
    
    let mut combinations = query.iter_combinations_mut();
    println!("\nNEW COMBO");
    while let Some([mut a1, mut a2]) = combinations.fetch_next() {
        // println!("{}",a1.1.translation);
        // println!("{}",a2.1.translation);

        // DO COLLISION DETECTION NERD

    }
    
    // fn intersect(p1:Particle, p2:Particle) -> bool{
    //     let a = p1.radius + p2.radius;
    //     let dx = p1.pos_x - p2.pos_x;
    //     let dy = p1.pos_y - p2.pos_y;
    //     return a * a > (dx*dx + dy*dy);
    // }
}

