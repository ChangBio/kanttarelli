use std::{f32::consts::PI, iter::Enumerate};

use three_d::*;
use crate::vec_tree::*;

//pub struct 

fn segment_color(max_color:Color,auxin_level: f32, render_params:&RenderParams) -> Color{
    let white = vec3(1.,1.,1.);
    let blue = vec3(0.0,0.,1.);
    //let blue: Vec3 = max_color.to_vec3();
    //println!("{blue:?}");
    let maximum=render_params.auxin_max;
    let coeff = (auxin_level.clamp(0., maximum)/maximum).powf(render_params.color_exp);
    let c = white*(1.-coeff)+blue*coeff;
    Color::from_rgb_slice(&[c.x,c.y,c.z])
}
fn branching_color(max_color:Color,auxin_level: f32, render_params:&RenderParams) -> Color{
    segment_color(max_color,auxin_level, render_params)
}
#[derive(Clone,Copy)]
pub struct RenderParams{
    pub render_buds: bool,
    pub auxin_max: f32,
    pub color_exp: f32,
    pub initial_width: f32,
    pub order_width_influence: f32,
    pub branching_angle: f32,
    pub divergence_angle: f32,
    pub segment_length:f32,
    pub max_color: Color
}
impl Default for RenderParams{
    fn default() -> Self {
        RenderParams { render_buds: false, 
            auxin_max: 0.7, 
            color_exp: 0.3, 
            initial_width: 2.5, 
            order_width_influence: 2.,
            branching_angle: PI*0.3,
            divergence_angle: PI*(137.5/180.),
            segment_length: 2.,
            max_color: Color::from_rgb_slice(&[1.,0.,0.]),
        }
    }
}

pub struct IntancesData{
    segment_instances: Instances,
    branching_instances: Instances,
    bud_instances: Instances,
    segment_meshes:Gm<InstancedMesh,PhysicalMaterial>,
    branching_meshes:Gm<InstancedMesh,PhysicalMaterial>,
    bud_meshes:Gm<InstancedMesh,PhysicalMaterial>,
    pub objects_position_index: Vec<(Vec3,i32,i32)>,
    pub render_buds: bool,
    pub render_params: RenderParams
}

impl IntancesData{
    pub fn from_tree(context: &Context, tree: &mut Tree,segment_mesh: &CpuMesh,branching_mesh: &CpuMesh,bud_mesh: &CpuMesh,render_params: &RenderParams) -> IntancesData {
        let color = Color::new(120, 120, 120, 255);
        let a = vec3(1.,1.,1.);
        Color::from_rgb_slice(a.as_ref());
        let mut segments_transformations = vec![];
        let mut bud_transformations = vec![];
        let mut branching_transformations = vec![];
        let mut segments_colors = vec![];
        let mut branching_colors = vec![];
        let mut bud_colors = vec![];
        let mut objects_position_index = vec![];

        tree.update_transformations(render_params.divergence_angle, render_params.branching_angle, 
            render_params.segment_length, render_params.initial_width, render_params.order_width_influence);
        for i in 0..tree.get_size(){
            let mut transformation = tree.transformation*tree.nodes[i].transformation;    

            let s = render_params.initial_width/((tree.nodes[i].initial_order as f32)*render_params.order_width_influence+1.);
            //let width_transformation = Mat4::from_nonuniform_scale(s,1.,s);

       
            for (j,segment) in tree.nodes[i].segments.iter().enumerate(){
                transformation = transformation*Mat4::from_translation(vec3(0.,2.,0.));
                segments_transformations.push(transformation);
                segments_colors.push(segment_color(render_params.max_color,segment.data.auxin,render_params));
                objects_position_index.push(((transformation*vec4(0.,0.,0.,1.)).truncate(),i as i32,j as i32));
            }
            transformation = transformation*Mat4::from_translation(vec3(0.,2.,0.));
            objects_position_index.push(((transformation*vec4(0.,0.,0.,1.)).truncate(),i as i32,-1));
            match tree.nodes[i].bud_state{
                BudState::BranchingSegment => {
                    branching_transformations.push(transformation);
                    branching_colors.push(segment_color(render_params.max_color,tree.nodes[i].data.auxin,render_params));

                },
                BudState::ActiveBud => {
                    bud_transformations.push(transformation*Mat4::from_angle_x(radians(PI/2.)));
                    bud_colors.push(Color::from_rgb_slice(&[0.,1.,0.]));
                },
                BudState::DecapitatedSegment => {
                    bud_transformations.push(transformation*Mat4::from_angle_x(radians(PI/2.)));
                    bud_colors.push(Color::from_rgb_slice(&[0.,0.,0.]));
                },
                BudState::DormantBud => {
                    bud_transformations.push(transformation*Mat4::from_angle_x(radians(PI/2.)));
                    bud_colors.push(Color::from_rgb_slice(&[1.,1.,0.]));
                },
            }
        }
        //segments
        let segment_instances = Instances {
            transformations:segments_transformations,
            colors:Some(segments_colors),
            ..Default::default()
        };
        let segment_meshes:Gm<InstancedMesh,PhysicalMaterial> = Gm::new(
            InstancedMesh::new(&context,&segment_instances,segment_mesh),
            PhysicalMaterial::default()
        );
        //buds
        let bud_instances = Instances {
            transformations:bud_transformations,
            colors:Some(bud_colors),
            ..Default::default()
        };
        let bud_meshes:Gm<InstancedMesh,PhysicalMaterial> = Gm::new(
            InstancedMesh::new(&context,&bud_instances,bud_mesh),
            PhysicalMaterial::default()
        );
        //branching=
        let branching_instances = Instances {
            transformations:branching_transformations,
            colors:Some(branching_colors),
            ..Default::default()
        };
        let branching_meshes:Gm<InstancedMesh,PhysicalMaterial> = Gm::new(
            InstancedMesh::new(&context,&branching_instances,branching_mesh),
            PhysicalMaterial::default()
        );
        
        IntancesData{
            segment_instances,
            branching_instances,
            bud_instances,
            segment_meshes,
            branching_meshes,
            bud_meshes,
            objects_position_index,
            render_buds:render_params.render_buds,
            render_params:*render_params
        }
    }
    pub fn render(&self, camera: &Camera, lights: &[&dyn Light]){
        self.segment_meshes.render(camera, lights);
        self.branching_meshes.render(camera, lights);
        if self.render_buds{
            self.bud_meshes.render(camera, lights);
        }
    }
    pub fn instances_datas_to_positions(instance_datas:&Vec<IntancesData>)->Vec<(Vec3,usize,i32,i32)>{
        let mut result = vec![];
        for (tree_id,instance_data) in instance_datas.iter().enumerate(){
            for (pos,node,segment) in &instance_data.objects_position_index{
                result.push((*pos,tree_id,*node,*segment));
            }
        }
        result
    }
}

