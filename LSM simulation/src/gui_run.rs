use std::{f32::consts::PI, cmp::Ordering};

use csv::Writer;
use rand::thread_rng;
use three_d::*;


use crate::{vec_tree::*, instance_data::*};

#[cfg(not(target_arch = "wasm32"))]
use tinyfiledialogs::save_file_dialog;

#[derive(PartialEq)]
enum PlotType { Auxin, PIN }


pub fn distance_direction(pos: Vector3<f32>, direction: Vector3<f32>,other: Vector3<f32>) -> (f32,f32) {
    let v = pos - other;
    let magnitude = v.magnitude();
    let other_dir = v.normalize();
    let dist = direction.cross(other_dir).magnitude()*magnitude;
    (dist,magnitude)
}
pub fn generate_trees_showcase(distance:f32)-> Vec<Tree>{
    let settings = Settings::global_copy();
    
    let mut trees = vec![];

    let d = distance;
    
    let mut tree = rnai60(&settings);
    tree.transformation = Mat4::from_translation(vec3(d,0.,d));

    // let branching_angle = PI*0.1;
    // let divergence_angle = PI*(137./180.);
    // let segment_length = 2.;

    // tree.update_transformations(divergence_angle,branching_angle,segment_length);

    trees.push(tree);

    let mut tree = wild_type_week_11(&settings);
    tree.transformation = Mat4::from_translation(vec3(-d,0.,-d));

    // let branching_angle = PI*0.4;
    // let divergence_angle = PI*(137./180.);
    // let segment_length = 2.;

    // tree.update_transformations(divergence_angle,branching_angle,segment_length);
    trees.push(tree);

    //poles 
    // let mut tree = pole(40,&settings);
    // tree.transformation = Mat4::from_translation(vec3(-d,0.,d));

    // let branching_angle = PI*0.4;
    // let divergence_angle = PI*(137./180.);
    // let segment_length = 2.;

    // tree.update_transformations(divergence_angle,branching_angle,segment_length);
    // trees.push(tree);

    // let mut tree = pole_internode_size_active(40,5,&settings);
    // tree.transformation = Mat4::from_translation(vec3(d,0.,-d));

    // let branching_angle = PI*0.4;
    // let divergence_angle = PI*(137./180.);
    // let segment_length = 2.;

    // tree.update_transformations(divergence_angle,branching_angle,segment_length);

    //trees = trees.iter().map(|t|{ Tree::calculate_static_distribution(t,0.001)}).collect();
    trees
}

pub async fn run_gui_showcase() {
    //SETTINGS.lock().unwrap().segments_amount=40;
    //SETTINGS.lock().unwrap().decay=0.;
    //SETTINGS.lock().unwrap().init_pin=50.;

    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&[
            "./assets/bud.glb",
            "./assets/segment.glb",
            "./assets/segment_branching.glb",
            "https://fingal.github.io/assets/segment_branching1.glb",
            "./assets/segment_branching2.glb",
        ]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://fingal.github.io/assets/bud.glb",
            "https://fingal.github.io/assets/segment.glb",
            "https://fingal.github.io/assets/segment_branching.glb",
            "https://fingal.github.io/assets/segment_branching1.glb",
            "https://fingal.github.io/assets/segment_branching2.glb",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };
    let mut rng = thread_rng();
    let nodes =0;
    let mut initial_size = 20;
    let prob = 0.75;
    //let mut tree = pole(40);
    //let mut tree = wild_type_week_11();
    let mut trees = generate_trees_showcase(80.);
    
    // for i in 1..tree.get_size(){
    //     println!("{} {}",tree.nodes[i].main_child,tree.nodes[i].secondary_child);
    //     println!("{:?}",tree.nodes[i].transformation);
    // }

    let branching_angle = PI*0.2;
    let divergence_angle = PI*(137./180.);
    let segment_length = 2.;
    


    let window = three_d::Window::new(three_d::WindowSettings {
        title: "Trees".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut primary_camera = Camera::new_perspective(
        window.viewport(),
        vec3(-300.0, 250.0, 200.0),
        vec3(0.0, 160.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut control = OrbitControl::new(
        *primary_camera.target(),
        0.5 * primary_camera.target().distance(*primary_camera.position()),
        5.0 * primary_camera.target().distance(*primary_camera.position()),
    );


    let segment_mesh: CpuMesh= loaded.deserialize("segment").unwrap();
    let bud_mesh: CpuMesh = loaded.deserialize("bud").unwrap();

    let branching_mesh: CpuMesh = match loaded.deserialize("segment_branching"){
        Ok(x) => {x},
        Err(_) => {
            println!("first import failed");
            match loaded.deserialize("segment_branching1"){
                Ok(x) => {x},
                Err(_) =>{
                    println!("second import failed");
                    loaded.deserialize("segment_branching2").unwrap()
                }
            }
        },
    };

    let mut render_params: RenderParams=Default::default();
    let colors = [Color::new(217, 95, 2,55),Color::new(27, 120, 55,255)];
    let mut instances_datas:Vec<IntancesData> = (&mut trees).iter_mut().enumerate().map(|(_i,t)|{
        render_params.max_color=colors[_i];

        IntancesData::from_tree(&context, t, &segment_mesh, &branching_mesh, &bud_mesh, &render_params)
    }).collect();
    let mut raycast_data = IntancesData::instances_datas_to_positions(&instances_datas);
    

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let mut directional = DirectionalLight::new(
        &context,
        10.0,
        Color::new_opaque(255, 255, 255),
        &vec3(0.0, -1.0, -1.0),
    );

    let mut gui = three_d::GUI::new(&context);
    // }

    let mut i=0;
    let mut growTree = false;
    let mut paused = false;
    let mut clicks_positions = vec![];



    //mesh for raycasting testing
    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    instanced_mesh.set_animation(|time| Mat4::from_angle_x(Rad(time)));

    let mut closest_point = (Vector3::<f32>{x:0.,y:0.,z:0.},0 as usize,-1,-1);


    // text_fields_for settings
    
    let mut active_gain :String = SETTINGS.lock().unwrap().active_gain.to_string();
    let mut dormant_gain:String = SETTINGS.lock().unwrap().dormant_gain.to_string();
    let mut decay:String = SETTINGS.lock().unwrap().decay.to_string();
    let mut pin_decay:String = SETTINGS.lock().unwrap().pin_decay.to_string();
    let mut pin_production_1:String = SETTINGS.lock().unwrap().pin_production.0.to_string();
    let mut pin_production_2:String = SETTINGS.lock().unwrap().pin_production.1.to_string();
    let mut render_buds=false;


    let mut show_plot=false;

    let mut plot_type = PlotType::Auxin;

    let mut simulation_step=0;
    

    window.render_loop(move |mut frame_input| {
        let mut new_trees = vec![];
        for tree in &mut trees{
            i+=1;
            let mut new_tree=tree.clone();
            if !paused{
                for _ in 0..30{
                    simulation_step+=1;
                    new_tree=Tree::update_tree_copy(&new_tree);
                }
            }
            if growTree{random_growth(&mut new_tree, prob);}
            //new_tree.update_transformations(divergence_angle, branching_angle, segment_length);
            new_trees.push(new_tree);
        }
        trees = new_trees;
        //print!("{:?}",tree.nodes[0]);

        let mut instances_datas:Vec<IntancesData> = trees.iter_mut().enumerate().map(|(_i,t)|{
            render_params.max_color=colors[_i];
            IntancesData::from_tree(&context, t, &segment_mesh, &branching_mesh, &bud_mesh,&render_params)
        }).collect();
        raycast_data = IntancesData::instances_datas_to_positions(&instances_datas);

        //println!("frames {}, instances {}",1000./frame_input.elapsed_time,tree_size);

        
    //     let colors = segment_instances.colors.as_mut();
    // // The division was valid
    //     let size = segment_instances.count();
    //     {
    //         match segment_instances.colors.as_mut(){
    //             Some(mut colors) => colors[rng.gen_range(0..size) as usize]=Color::BLACK,
    //             None => {},
    //         }
    //     }
    //     segment_meshes.set_instances(&segment_instances);
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                let sp = SidePanel::left("side_panel");
                sp.show(gui_context, |ui| {
                    // ui.heading("Debug Panel");
                    // ui.radio_value(&mut camera_type, CameraType::Primary, "Primary camera");
                    // ui.radio_value(&mut camera_type, CameraType::Secondary, "Secondary camera");
                    let mut text:String = Default::default();
                    let mut _x=0;
                    if closest_point.2!=-1{
                        let tree=&trees[closest_point.1];

                        if closest_point.3<0{
                            text.push_str(&format!("auxin {:.3}\n",tree.nodes[closest_point.2 as usize].data.auxin));
                            text.push_str(&format!("auxin flow {:.3}\n",tree.nodes[closest_point.2 as usize].data.auxin_flow));
                            text.push_str(&format!("pin {:.3}\n\n",tree.nodes[closest_point.2 as usize].data.pin));
                            for seg in tree.nodes[closest_point.2 as usize].segments.iter().rev() {
                                _x+=1;
                                if _x>8{
                                    break;
                                }
                                text.push_str(&format!("auxin {:.3}\n",seg.data.auxin));
                                text.push_str(&format!("auxin flow {:.3}\n",seg.data.auxin_flow));
                                text.push_str(&format!("pin {:.3}\n\n",seg.data.pin));
                            }
                        }
                        else{
                            text.push_str(&format!("{:?}\n",tree.nodes[closest_point.2 as usize].segments[closest_point.3 as usize].data));
                        };
                        
                    }
                    ui.text_edit_multiline(&mut text);

                    // let mut active_gain :String = SETTINGS.lock().unwrap().active_gain.to_string();
                    // let mut dormant_gain :String = SETTINGS.lock().unwrap().dormant_gain.to_string();
                    // let mut decay :String = SETTINGS.lock().unwrap().decay.to_string();
                    // let mut pin_decay :String = SETTINGS.lock().unwrap().pin_decay.to_string();
                    // let mut pin_production_1 :String = SETTINGS.lock().unwrap().pin_production.0.to_string();
                    // let mut pin_production_2 :String = SETTINGS.lock().unwrap().pin_production.1.to_string();
                    ui.label(format!("simulation step {simulation_step}"));
                    ui.label("active gain");
                    ui.text_edit_singleline(&mut active_gain);
                    ui.label("dormant gain");
                    ui.text_edit_singleline(&mut dormant_gain);
                    ui.label("decay");
                    ui.text_edit_singleline(&mut decay);
                    ui.label("pin decay");
                    ui.text_edit_singleline(&mut pin_decay);
                    ui.label("pin production flow");
                    ui.text_edit_singleline(&mut pin_production_1);
                    ui.label("pin production");
                    ui.text_edit_singleline(&mut pin_production_2);
                    if ui.button("refresh settings").clicked(){
                        let settings = Settings::global_copy();
                        SETTINGS.lock().unwrap().active_gain=active_gain.parse::<f32>().unwrap_or(settings.active_gain);
                        SETTINGS.lock().unwrap().dormant_gain=dormant_gain.parse::<f32>().unwrap_or(settings.dormant_gain);
                        SETTINGS.lock().unwrap().decay=decay.parse::<f32>().unwrap_or(settings.decay);
                        SETTINGS.lock().unwrap().pin_decay=pin_decay.parse::<f32>().unwrap_or(settings.pin_decay);
                        SETTINGS.lock().unwrap().pin_production=(pin_production_1.parse::<f32>().unwrap_or(settings.pin_production.0),pin_production_2.parse::<f32>().unwrap_or(settings.pin_production.1));


                        for tree in &mut trees{
                            tree.new_settings(Settings::global_copy());
                        }
                    };


                    ui.checkbox(&mut render_params.render_buds, "Show buds");
                    let response = ui.add(egui::Slider::new(&mut render_params.auxin_max, 0. ..=2.));
                    let response = ui.add(egui::Slider::new(&mut render_params.color_exp, 0. ..=1.));
                    let response = ui.add(egui::Slider::new(&mut render_params.initial_width, 0. ..=3.));
                    let response = ui.add(egui::Slider::new(&mut render_params.order_width_influence, 0. ..=3.));


                    //ui.checkbox(&mut growTree, "Grow tree");
                    if ui.button("pause/resume").clicked(){
                        paused = !paused;
                    };
                    if ui.button("redraw").clicked(){
                        simulation_step=0;
                        let settings = Settings::global_copy();
                        SETTINGS.lock().unwrap().active_gain=active_gain.parse::<f32>().unwrap_or(settings.active_gain);
                        SETTINGS.lock().unwrap().dormant_gain=dormant_gain.parse::<f32>().unwrap_or(settings.dormant_gain);
                        SETTINGS.lock().unwrap().decay=decay.parse::<f32>().unwrap_or(settings.decay);
                        SETTINGS.lock().unwrap().pin_decay=pin_decay.parse::<f32>().unwrap_or(settings.pin_decay);
                        SETTINGS.lock().unwrap().pin_production=(pin_production_1.parse::<f32>().unwrap_or(settings.pin_production.0),pin_production_2.parse::<f32>().unwrap_or(settings.pin_production.1));

                        trees = generate_trees_showcase(80.);
                    }
                    ui.checkbox(&mut show_plot, "Show plots");
                    if show_plot{
                        
                        ui.radio_value(&mut plot_type, PlotType::Auxin, "Auxin");
                        ui.radio_value(&mut plot_type, PlotType::PIN, "Pin");
                        for (tree,name) in trees.iter().zip(["Kantarelli","Wild Type","Pole"]){
                            let values = tree.main_stem_values();
                            let mut first_peak = 0.;
                            for i in 0..tree.settings.segments_amount{
                                if (first_peak<values[i as usize].0){
                                    first_peak=values[i as usize].0;
                                }
                            }
                            let mut last_peak=0.;
                            for _i in 0..tree.settings.segments_amount{
                                let i = values.len() as i32-1-_i;
                                if (last_peak<values[i as usize].0){
                                    last_peak=values[i as usize].0;
                                }
                            }
                            let ratio = (last_peak-first_peak)/(first_peak+0.001);
                            Window::new(name).default_size(vec2(200.,400.)).show( gui_context, |ui| {
                                ui.label(format!("{ratio:.2}"));
                                let values = tree.main_stem_values();
                                let auxin: plot::PlotPoints = (0..values.len()).map(|i| {
                                    match plot_type{
                                        PlotType::Auxin => [i as f64, values[i].0 as f64],
                                        PlotType::PIN => [i as f64, values[i].1 as f64]
                                    }
                                }).collect();
                                let starting_value = values[0].0;
                                let line = plot::Line::new(auxin);
                                let line = match plot_type{
                                    PlotType::Auxin =>line.color(Color32::DARK_GREEN),
                                    PlotType::PIN => line.color(Color32::LIGHT_BLUE)
                                };
                                plot::Plot::new("my_plot").view_aspect(2.0).allow_scroll(false).include_y(0.0).include_y(starting_value*2.).show(ui, |plot_ui| {
                                    plot_ui.line(line);
                                    //plot_ui.line(pin_line);
                                });
                            });
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]{
                    if ui.button("save result").clicked(){
                        //let result=save_file_dialog("file location", "./");
                        if let Some(path) =save_file_dialog("file location", "./"){
                            println!("{path}");
                            for (tree,name) in trees.iter().zip(["Kantarelli","Wild Type","Pole"]){
                                let values = tree.main_stem_values();
                                let mut wtr = Writer::from_path(format!("{path} {name}.csv")).unwrap();
                                wtr.write_record(&["internode","Auxin","PIN"]).unwrap();
                                for (i,(a,b)) in values.iter().enumerate(){
                                    wtr.write_record(&[format!("{}",(i as i32 + tree.settings.segments_amount-tree.nodes[tree.tip_index as usize].segments.len() as i32-1)/tree.settings.segments_amount+1),format!("{a:.5}"),format!("{b:.5}")]).unwrap();
                                }
                                wtr.flush().unwrap();
                            }
                        }
                    }
                }
                });
                panel_width = gui_context.used_rect().width() as f64;
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        primary_camera.set_viewport(viewport);
        control.handle_events(&mut primary_camera, &mut frame_input.events);
        for event in &frame_input.events{
            if let Event::MousePress { button, position, modifiers, handled } = event {
                if let MouseButton::Right=button{
                    //przepisac na obsluge wielu drzew?

                    let pos = ((position.0 as f32)* frame_input.device_pixel_ratio as f32,viewport.height as f32 - (position.1 as f32)* frame_input.device_pixel_ratio as f32);
                    let p = primary_camera.position_at_pixel(pos);
                    println!("{:?}",p);
                    let v = primary_camera.view_direction_at_pixel(pos);
                    println!("{:?}",v);
                    closest_point = *raycast_data.iter().min_by(|(a,_,_,_),(b,_,_,_)|{
                        let (a1,a2) = distance_direction(p, v, *a);
                        let (b1,b2) = distance_direction(p, v, *b);

                        if a1 < 1.5 && b1<1.5 && (a2-b2).abs()<2.{
                                if b2<a2{
                                    return Ordering::Greater;
                                }
                                else{
                                    return Ordering::Less;
                                }
                            }
                        if a1<b1{
                            return Ordering::Less;
                        }
                        else{
                            return Ordering::Greater;
                        }
                    }).unwrap();
                    println!("{:?}",closest_point);
                    clicks_positions= vec!{(closest_point.0)};
                }
            }

        }

        // draw
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0,1.0, 1.0, 1.0))
            .write(|| {
                let camera = &primary_camera;
                instanced_mesh.set_instances(&Instances {
                    transformations: (0..clicks_positions.len())
                        .map(|i| {
                            Mat4::from_translation(
                                clicks_positions[i],
                            )
                        })
                        .collect(),
                    ..Default::default()
                });
                instanced_mesh.render(camera, &[&ambient, &directional]);
                // for object in models
                //     .iter()
                //     .flatten()
                // {
                //     object.render(camera, &[&ambient, &directional]);
                // }
                //let mut instances_data = IntancesData::from_tree(&context, &tree, &segment_mesh, &branching_mesh, &bud_mesh);
                for instance_data in instances_datas{
                    instance_data.render(camera, &[&ambient, &directional]);
                }
                gui.render();
            });

        FrameOutput::default()
    });
}



// pub async fn benchmark() {
//     let mut precision = 0.01;
//     for exp in 0..30{
//         precision *=0.5;
//         let mut average_i=0.;
//         let steps = 5;
//         let sec = timeit_loops!(steps,{
//             let mut tree=kanttarelli_week_11();
//             let mut difference =100.;
//             let mut i = 0;
//             while difference>precision{
//                 difference=0.;
//                 for _ in 0..2{
//                     let new_tree=Tree::update_tree(&tree);
//                     difference+=tree.auxin_difference(&new_tree);
//                     tree=new_tree;
//                     i+=1;
//                 }
//                 //println!("step {i} difference {difference:.5}");
//             }
//             average_i+=i as f32;
//         });
//         average_i/=steps as f32;
//         println!("time taken {sec:.4} seconds and {average_i:.2} steps on average for precision {precision}");
//     }

// }
