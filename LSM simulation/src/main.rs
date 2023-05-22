// Entry point for non-wasm
//extern crate rand;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod vec_tree;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod instance_data;
#[cfg(not(target_arch = "wasm32"))]
mod model_functions;
#[cfg(not(target_arch = "wasm32"))]
mod gui_run;


use serde_pickle::SerOptions;
#[cfg(not(target_arch = "wasm32"))]
use vec_tree::*;
#[cfg(not(target_arch = "wasm32"))]
use instance_data::*;
#[cfg(not(target_arch = "wasm32"))]
use gui_run::*;

#[cfg(target_arch = "wasm32")]
use super::vec_tree::*;
#[cfg(target_arch = "wasm32")]
use super::instance_data::*;
#[cfg(target_arch = "wasm32")]
use super::gui_run::*;

use rayon::prelude::*;
//use std::cmp::Ordering;
use std::fmt::write;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::time::Instant;
use std::{f32::consts::PI, borrow::BorrowMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use linya::*;
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    //run_gui_showcase().await;
    run_gui_showcase().await;
    //run().await;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum CameraType {
    Primary,
    Secondary,
}

use three_d::*;
//use three_d::*;
use rand::prelude::*;

pub fn iter_range_steps(start:f32,end:f32,steps: i32)->Vec<f32>{
    let result :Vec<f32> =(0..steps).map(|i:i32| -> f32 {
        (i as f32*(end-start))/(steps as f32-1.)+start
    }).collect();
    result
}
pub fn iter_range_step_by(start:f32,end:f32,step: f32)->Vec<f32>{
    let steps = ((end-start)/step).ceil() as i32;
    let result :Vec<f32> =(0..steps+1).map(|i:i32| -> f32 {
        (i as f32* step)+start
    }).collect();
    result
}

pub fn iter_range_step_around(middle:f32,steps_side:i32,step_size: f32)->Vec<f32>{
    let start = middle - (steps_side as f32) * step_size;
    let result :Vec<f32> =(0..steps_side*2+1).map(|i:i32| -> f32 {
        (i as f32* step_size)+start
    }).collect();
    result
}

pub fn save_tree(trees:&Vec<Tree>,name:&str){
    let active_gain     = SETTINGS.lock().unwrap().active_gain;
    let dormant_gain     = SETTINGS.lock().unwrap().dormant_gain;
    let decay     = SETTINGS.lock().unwrap().decay;
    let pin_decay = SETTINGS.lock().unwrap().pin_decay;
    let (pin_production_1,pin_production_2) = SETTINGS.lock().unwrap().pin_production;
    let path = format!("./results/v2/{name}/active_gain {active_gain:.3} dormant_gain {dormant_gain:.3} decay {decay:.3} pin_decay {pin_decay:.3} pin_production_1 {pin_production_1:.3} pin_production_2 {pin_production_2:.2}");
    let mut file = File::create(path).unwrap();
    let pickle = serde_pickle::to_vec(&trees,SerOptions::new().proto_v2()).unwrap();
    file.write_all(&pickle);
}

pub fn save_tree_main_stem(trees:&Vec<Tree>,name:&str,settings:&Settings){
    let active_gain     = settings.active_gain;
    let dormant_gain     = settings.dormant_gain;
    let decay     = settings.decay;
    let pin_decay = settings.pin_decay;
    let (pin_production_1,pin_production_2) = settings.pin_production;
    let path = format!("./results_rnai60/{name}/active_gain {active_gain:.3} dormant_gain {dormant_gain:.3} decay {decay:.3} pin_decay {pin_decay:.3} pin_production_1 {pin_production_1:.3} pin_production_2 {pin_production_2:.2}");
    let mut file = File::create(path).unwrap();

    let data : Vec<Vec<(f32,f32)>> = trees.iter().map(|tree|{
        tree.main_stem_values()
    }).collect();

    let pickle = serde_pickle::to_vec(&data,SerOptions::new().proto_v2()).unwrap();
    file.write_all(&pickle);
}

pub fn generate_trees(settings: &Settings){
    //może przepisać to na macro
    let precision = 0.001;
    {
        let mut trees = vec![];

        for _ in 0..20{
            let now = Instant::now();
            let mut tree = rnai60(&settings);
            tree = Tree::calculate_static_distribution(&tree,precision);
            trees.push(tree);
            let elapsed_time = now.elapsed();
            let seconds = elapsed_time.as_secs_f32();
            //println!("time {seconds}")
        }
        //println!("kanttarelli saving");
        save_tree_main_stem(&trees,"rnai60",&settings);
    }

    {
        let mut trees = vec![];
    for _ in 0..20{
        let mut tree = wild_type_week_11(settings);
        tree = Tree::calculate_static_distribution(&tree,precision);
        trees.push(tree);
    }
    save_tree_main_stem(&trees,"WT",&settings);
    }

    // {
    //     let mut trees = vec![];
    // for _ in 0..1{
    //     let mut tree = pole(40,settings);
    //     tree = Tree::calculate_static_distribution(&tree,precision);
    //     trees.push(tree);
    // }
    // save_tree_main_stem(&trees,"pole",&settings);}

    // {
    //     let mut trees = vec![];
    // for i in 3..8{
    //     let mut tree = pole_internode_size(40,i,settings);
    //     tree = Tree::calculate_static_distribution(&tree,precision);
    //     trees.push(tree);
    // }
    // save_tree_main_stem(&trees,"pole_segments",&settings);}

    // {
    //     let mut trees = vec![];
    // for i in 3..8{
    // let mut tree = pole_internode_size_active(40,i,settings);
    //     tree = Tree::calculate_static_distribution(&tree,precision);
    //     trees.push(tree);
    // }
    // save_tree_main_stem(&trees,"pole_segments_activated",&settings);}
}

pub fn reset_settings(){
    SETTINGS.lock().unwrap().init_auxin= 0.;
    SETTINGS.lock().unwrap().init_strigolactin= 0.;
    SETTINGS.lock().unwrap().init_pin= 1.;
    SETTINGS.lock().unwrap().dormant_gain= 0.25;
    SETTINGS.lock().unwrap().active_gain= 0.5;
    SETTINGS.lock().unwrap().decay= 0.15;
    SETTINGS.lock().unwrap().pin_decay= 0.05;
    SETTINGS.lock().unwrap().pin_production= (1.0,0.02);
}

pub async fn run(){
    //let tree = kanttarelli_week_11(Settings.global_copy());
    
//             let path = format!("./pickle/diffusion/populations with diffusion strength {} steps {}_2",diffusion_strength,diffusion_steps);
//             let mut file = File::create(path).unwrap();
//             let pickle = serde_pickle::to_vec(&population_amounts, true).unwrap();
//             file.write_all(&pickle);
    let precision = 0.0001;
    let mut i=0;

    // for active_gain in iter_range_steps(0.5,2.0,4){
    //     for dormant_ratio in iter_range_steps(0.1,1.0,4){
    //         let dormant_gain = dormant_ratio*active_gain;
    //         for decay in iter_range_steps(0.05,0.4,8){
    //             for pin_decay in iter_range_steps(0.,0.3,7){
    //                 for pin_production_1 in iter_range_steps(0.2,2.,10){
    //                     for pin_production_2 in iter_range_steps(0.,0.1,6){
    //                         i+=1;
    //                         {
    //                             SETTINGS.lock().unwrap().active_gain=active_gain;
    //                             SETTINGS.lock().unwrap().dormant_gain=dormant_gain;
    //                             SETTINGS.lock().unwrap().decay=decay;
    //                             SETTINGS.lock().unwrap().pin_decay=pin_decay;
    //                             SETTINGS.lock().unwrap().pin_production=(pin_production_1,pin_production_2);
    //                         }
    //                         // let mut tree = kanttarelli_week_11();
    //                         // calculate_static_distribution(&tree, precision)

    //                     }
            
    //                 }
    //             }
    //         }
    //     }
    // }
    // let mut pin_decay_vec = iter_range_step_by(0.01,0.05,0.01);
    // pin_decay_vec.append(&mut iter_range_step_by(0.1,0.25,0.05));
    // println!("{pin_decay_vec:?}");
    // println!("decay {:?}",iter_range_step_by(0.05,0.30,0.025));
    // println!("pin_production {:?}",iter_range_step_by(0.5,1.50,0.1));
    // for decay in iter_range_step_by(0.025,0.30,0.025){
    //     for pin_decay in &pin_decay_vec{
    //         let now = Instant::now();
    //         {
    //             SETTINGS.lock().unwrap().pin_decay= *pin_decay;
    //             SETTINGS.lock().unwrap().decay=decay;

    //         }
    //         println!("{}, {}",pin_decay,decay);
    //         generate_trees();
    //         let elapsed_time = now.elapsed();
    //         let seconds = elapsed_time.as_secs_f32();
    //         println!("calculations took {seconds:.3} seconds\n");
    //     }
    // }
    // {
    //     SETTINGS.lock().unwrap().pin_decay=0.05;
    //     SETTINGS.lock().unwrap().decay=0.15;
    //}

    // for decay in iter_range_step_by(0.05,0.30,0.025){
    //     for pin_production in iter_range_step_by(0.5,1.50,0.1){
    //         let now = Instant::now();
    //         {
    //             SETTINGS.lock().unwrap().pin_production=(pin_production,0.02);
    //             SETTINGS.lock().unwrap().decay=decay;

    //         }
    //         println!("{}, {}",pin_production,decay);
    //         generate_trees();
    //         let elapsed_time = now.elapsed();
    //         let seconds = elapsed_time.as_secs_f32();
    //         println!("calculations took {seconds:.3} seconds\n");
    //     }
    // }    
    // println!("{:?}",iter_range_step_by(0.1,0.4,0.1));
    // for decay in iter_range_step_by(0.3,0.50,0.05){
    //     for active_gain in iter_range_step_by(0.1,0.4,0.1){
    //         let now = Instant::now();
    //         {
    //             SETTINGS.lock().unwrap().active_gain=active_gain;
    //             SETTINGS.lock().unwrap().dormant_gain=0.5*active_gain;
    //             SETTINGS.lock().unwrap().decay=decay;

    //         }
    //         println!("{}, {}",active_gain,decay);
    //         generate_trees();
    //         let elapsed_time = now.elapsed();
    //         let seconds = elapsed_time.as_secs_f32();
    //         println!("calculations took {seconds:.3} seconds\n");
    //     }
    // }


    // for pin_production_2 in iter_range_step_by(0.01,0.05,0.01){
    //     for pin_production in iter_range_step_by(0.5,1.70,0.1){
    //         let now = Instant::now();
    //         {
    //             SETTINGS.lock().unwrap().pin_production=(pin_production,pin_production_2);
    //         }
    //         println!("{}, {}",pin_production,pin_production_2);
    //         generate_trees();
    //         let elapsed_time = now.elapsed();
    //         let seconds = elapsed_time.as_secs_f32();
    //         println!("calculations took {seconds:.3} seconds\n");
    //     }
    // }    
    let decays=iter_range_step_around(0.155,6,0.02);
    let gains=iter_range_step_around(0.7,6,0.1);
    let mut settings_vec = vec![];
    // for pin_decay in iter_range_step_by(0.1,0.15,0.01){
    //     for decay in iter_range_step_by(0.1,0.15,0.01){
    //         for active_gain in iter_range_step_by(0.4,0.5,0.01){
    // for decay in iter_range_step_by(0.125,0.175,0.005){
    //     for active_gain in iter_range_step_by(0.3,0.6,0.02){
    println!("{:?}",decays);
    println!("{:?}",gains);
    
    for decay in decays{
        for active_gain in &gains{
            for gain_ratio in [0.17]{
                let mut settings : Settings = Default::default();
                settings.dormant_gain=active_gain*gain_ratio;
                settings.active_gain=*active_gain;
                settings.pin_production=(1.,0.06);
                settings.decay=decay;
                settings.pin_decay=0.05;
                settings_vec.push(settings);
            }
        }
    }
    
    let progress = Mutex::new(Progress::new());
    let mut bar = Mutex::new(progress.lock().unwrap().bar(settings_vec.len(), "in progress"));
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    settings_vec.par_iter().for_each(|setting| {
        let setting_str=format!("active_gain: {}, decay: {}, pin_decay: {}",setting.active_gain,setting.decay,setting.pin_decay);
        println!("started for settings: {setting_str}");
        let now = Instant::now();
        generate_trees(&setting);
        let elapsed_time = now.elapsed();
        let seconds = elapsed_time.as_secs_f32();
        println!("finished {setting_str}\ncalculations took {seconds:.3} seconds\n");

        let val = COUNTER.fetch_add(1, Ordering::SeqCst);
        
        progress.lock().unwrap().set_and_draw(&bar.lock().unwrap(), val);

     });
    // for pin_decay in [0.07]{
    //     for decay in [0.15,0.175,0.2]{
    //         for active_gain in iter_range_step_by(0.25,0.35,0.01){
    //             for gain_ratio in iter_range_step_by(0.5,0.6,0.01){
    //                 let now = Instant::now();
    //                 let mut settings : Settings = Default::default();
    //                 settings.active_gain=active_gain;
    //                 settings.dormant_gain=active_gain*gain_ratio;
    //                 settings.pin_decay=pin_decay;
    //                 settings.decay=decay;
    //                 println!("started {active_gain}, {gain_ratio}");
    //                 generate_trees(settings);
    //                 let elapsed_time = now.elapsed();
    //                 let seconds = elapsed_time.as_secs_f32();
    //                 println!("finished {active_gain}, {gain_ratio}");
    //                 println!("calculations took {seconds:.3} seconds\n");
    //             }
    //         }  
    //     }
    // }
    // println!("active_gain {:?}",iter_range_step_by(0.45,0.55,0.01));
    // println!("gain_ratio {:?}",iter_range_step_by(0.5,0.6,0.01));
    // println!("pin_decay {:?}",[0.04,0.5]);
    // println!("decay {:?}",[0.15,0.175,0.2]);
    //     for pin_production_1 in [0.04,0.5]{
    // }

    // println!("pin_decay= {:?}", iter_range_step_by(0.1,0.15,0.01));
    // println!("decay= {:?}", iter_range_step_by(0.1,0.15,0.01));
    // println!("active_gain= {:?}", iter_range_step_by(0.4,0.5,0.01));

    //for 
}