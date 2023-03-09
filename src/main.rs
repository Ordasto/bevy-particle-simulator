use bevy::{prelude::*, sprite::MaterialMesh2dBundle};


const PARTICLE_COLOR:Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR:Color = Color::rgb(0.1, 0.1, 0.1);


// const PHYSICS_STEP:f32 = 1.0/60.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
//        .add_system(collision_detection)
        .add_system(gravity_down)
        .add_system(constraint)
        .add_system(apply_physics)
        .run();
}



#[derive(Component)]
struct Particle {
    prev_pos: Vec3,
    acceleration: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    
    let pos = Vec3::new(2.0, 0.0, 0.0);
    for _ in 1..2 {
        commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(PARTICLE_COLOR)),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Particle {
                prev_pos: pos.clone(),
                acceleration: Vec3::new(0.0,0.0,0.0), 
            }
        ));
    }
}

fn gravity_down(mut query: Query<&mut Particle>) {
    for mut particle in &mut query {
        particle.acceleration.y += -20.0;
    }
}

fn constraint(mut query: Query<&mut Transform>) {
    let constraint_pos = Vec2::new(0.0, 0.0);
    let radius = 200.0;
    for mut transf in &mut query {
        let to_transf = Vec2::new(transf.translation.x - constraint_pos.x, transf.translation.y - constraint_pos.y);
        let dist = to_transf.length();
        // let dist = (to_transf.x * to_transf.x) + (to_transf.y * to_transf.y);
        if dist > radius - 10.0 {
            let n = to_transf / dist;
            let new_pos = constraint_pos + n * (dist-10.0);
            transf.translation.x = new_pos.x;
            transf.translation.y = new_pos.y;
        }
    }
    
}

fn apply_physics(mut query: Query<(&mut Particle, &mut Transform)>, time: Res<Time>) { 
    let dt = time.delta().as_secs_f32();
    for (mut particle, mut transform) in &mut query {
       
        let vel = transform.translation - particle.prev_pos; 

        particle.prev_pos = transform.translation;

        transform.translation = transform.translation + vel + particle.acceleration * dt * dt;

        particle.acceleration.x = 0.0;
        particle.acceleration.y = 0.0;
    }

}




// fn update_position(mut query: Query<(&mut Velocity, &mut Transform)>){
// 
//     for (mut velocity, mut transform) in &mut query{
//         
//         // How the hell does actual physics work lmao
// 
//         transform.translation.x += velocity.x; //* PHYSICS_STEP;
//         transform.translation.y += velocity.y; //* PHYSICS_STEP;
// 
//         // pull towards center test
//         
//         velocity.x -= transform.translation.x / 1000.0;
//         velocity.y -= transform.translation.y / 1000.0;
//         
//     }
// }
// 
// fn collision_detection(mut query: Query<(&mut Velocity, &Transform)>){
//     
//     let mut combinations = query.iter_combinations_mut();
//     while let Some([mut a1, mut a2]) = combinations.fetch_next() {
//         // println!("{}",a1.1.translation);
//         // println!("{}",a2.1.translation);
// 
//         // DO COLLISION DETECTION NERD
// 
//     }
//     
//     // fn intersect(p1:Particle, p2:Particle) -> bool{
//     //     let a = p1.radius + p2.radius;
//     //     let dx = p1.pos_x - p2.pos_x;
//     //     let dy = p1.pos_y - p2.pos_y;
//     //     return a * a > (dx*dx + dy*dy);
//     // }
// }

