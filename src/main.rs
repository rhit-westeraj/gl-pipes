use std::time::SystemTime;

// use cli::CliArgs;
use draw::{make_ball_joint, make_pipe_section, RenderObject};
use draw::{make_instanced_ball_joint, make_instanced_pipe_section};
use rand::Rng;
use three_d::{
    degrees, vec3, Camera, ClearState, CpuMaterial, CpuMesh, DirectionalLight, FrameOutput, Gm,
    InstancedMesh, Instances, OrbitControl, PhysicalMaterial, Srgba, Window, WindowSettings,
};
use world::World;

use crate::cli::make_cli_parser;

mod draw;
mod pipe;
mod util;
mod world;
mod cli;

const MAX_PIPES: u32 = 10;


fn main() {
    //===============================================
    // WORLD INITIALIZATION
    //===============================================
    let mut world = World::new();
    let mut rng = rand::thread_rng();
    let init_data = world.new_pipe(&mut rng);

    let cli_args = make_cli_parser().get_matches();
    dbg!(cli_args);

    //===============================================
    // ENGINE INIT
    //===============================================
    let window = Window::new(WindowSettings {
        title: "Rust Pipes".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-20.0, 10.0, -20.0),
        vec3(0.0, 10.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // let axes = Axes::new(&context, 0.1, 2.0);
    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    //FIXME Needs three-d fix
    //Instanced Rendering Data
    let mut pipe_instances = Instances {
        transformations: Vec::new(),
        colors: Some(Vec::new()),
        ..Default::default()
    };
    let mut ball_instances = Instances {
        transformations: Vec::new(),
        colors: Some(Vec::new()),
        ..Default::default()
    };

    let base_instance_material = CpuMaterial {
        albedo: Srgba {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        },
        ..Default::default()
    };

    let mut pipe_instance_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cylinder(16)),
        PhysicalMaterial::new(&context, &base_instance_material),
    );
    let mut ball_instance_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::sphere(16)),
        PhysicalMaterial::new(&context, &base_instance_material),
    );

    let start_time = SystemTime::now();

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        //World Update Step
        if !world.is_gen_complete() {
            for i in 0..world.active_pipes_count() {
                if !world.is_pipe_alive(i) {
                    continue;
                }
                let delta_state = world.pipe_update(i, &mut rng);

                if delta_state.last_node != delta_state.current_node {
                    if delta_state.last_dir != delta_state.current_dir {
                        make_instanced_ball_joint(
                            &mut ball_instances,
                            delta_state.last_node,
                            delta_state.pipe_color,
                        );
                    }
                    make_instanced_pipe_section(
                        &mut pipe_instances,
                        delta_state.last_node,
                        delta_state.current_node,
                        delta_state.pipe_color,
                    );
                }
            }
        }
        if rng.gen_bool(world.new_pipe_chance()) && world.max_active_count_reached(MAX_PIPES) {
            let data = world.new_pipe(&mut rng);
            make_instanced_ball_joint(&mut ball_instances, data.start_node, data.color);
        }

        match start_time.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs_f64() >= 15.0 {
                    world.set_gen_complete();
                    for i in 0..world.active_pipes_count() {
                        world.kill_pipe(i);
                    }
                }
            }
            Err(_) => panic!("Timer Did an oopsie, Panicking!!!!"),
        }

        // let pipe_count = pipe_instance_mesh.instance_count();
        // let ball_count = ball_instance_mesh.instance_count();
        // println!("Pipe instances: {pipe_count}, Ball_Instances: {ball_count}");

        // pipe_instance_mesh.set_instances(&pipe_instances);
        // ball_instance_mesh.set_instances(&ball_instances);
        // match pipe_instances.validate() {
        //     Ok(_) => (),
        //     Err(err) => panic!("{err}"),
        // }
        // match ball_instances.validate() {
        //     Ok(_) => (),
        //     Err(err) => panic!("{err}"),
        // }

        pipe_instance_mesh.set_instances(&pipe_instances);
        ball_instance_mesh.set_instances(&ball_instances);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                [&pipe_instance_mesh, &ball_instance_mesh].into_iter(),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}

#[cfg(test)]
mod tests {}
