
use std::f32::consts::PI;
use std::vec;
use std::fmt;
use three_d::*;
use rand::prelude::*;
use crate::model_functions::*;
use rand::seq::IteratorRandom;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(remote = "Vec4")]
pub struct Vector4Def {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
    /// The w component of the vector.
    pub w: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Mat4")]
pub struct Matrix4Def {
    /// The first column of the matrix.
    #[serde(with = "Vector4Def")]
    pub x: Vec4,
    /// The second column of the matrix.
    #[serde(with = "Vector4Def")]
    pub y: Vec4,
    /// The third column of the matrix.
    #[serde(with = "Vector4Def")]
    pub z: Vec4,
    /// The fourth column of the matrix.
    #[serde(with = "Vector4Def")]
    pub w: Vec4,
}


pub enum NodeExtensionResult{
    FullBefore,
    ExtensionFinished,
    Extended,
}
use std::sync::Mutex;
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Settings{
    pub segments_amount: i32,
    pub init_auxin: f32,
    pub init_strigolactin: f32,
    pub init_pin: f32,
    pub dormant_gain: f32,
    pub segment_gain: f32,
    pub active_gain: f32,
    pub decay: f32,
    pub pin_decay: f32,
    pub pin_production: (f32,f32),
    pub dt: f32
}
pub static SETTINGS: Mutex<Settings> = Mutex::new(Settings{
    segments_amount:5,
    init_auxin: 0.,
    init_strigolactin: 0.,
    init_pin: 1.,
    dormant_gain: 0.25,
    active_gain: 0.5,
    decay: 0.15,
    pin_decay: 0.05,
    segment_gain: 0.0,
    pin_production: (1.0,0.02),
    dt: 0.01,
});
impl Settings{
    pub fn global_copy()->Settings{
        let x = SETTINGS.lock().unwrap().clone();
        return x
    }
}

impl Default for Settings {
    fn default()->Settings{
        Settings{
            segments_amount:5,
            init_auxin: 0.,
            init_strigolactin: 0.,
            init_pin: 1.,
            dormant_gain: 0.25,
            active_gain: 0.5,
            decay: 0.15,
            pin_decay: 0.05,
            pin_production: (1.0,0.02),
            segment_gain: 0.0,
            dt: 0.01,
        }
    }
}
// pub static mut SETTINGS: Settings = Settings{
//     segments_amount:5,
//     init_auxin: 0.,
//     init_strigolactin: 0.,
//     init_pin: 0.,
// };


#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Data{
    pub order: i32,
    pub age: i32,
    pub auxin: f32,
    pub strigolactin: f32,
    pub pin: f32,
    pub auxin_flow:f32
}
impl Data {
    pub fn new(order: i32,init_auxin: f32, init_pin: f32) -> Data{
        let mut auxin = init_auxin;
        //let mut auxin = rand::random();
        let mut strigolactin= 0.;
        let mut pin= init_pin;
        Data { order,age:0,auxin, strigolactin, pin,auxin_flow:0. }
    }
}
impl Data{
    fn auxin_update(&mut self,dA: f32){
        self.auxin=(self.auxin+dA).max(0.);
    }
    fn pin_update(&mut self,dP: f32){
        self.pin=(self.pin+dP).max(0.);
    }
}
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Segment{
    pub data: Data,
}
impl Segment{
    pub fn new(order:i32,init_auxin: f32, init_pin: f32) -> Segment{
        Segment { data: Data::new(order,init_auxin, init_pin) }
    }
}

#[derive(Serialize, Deserialize,Clone,Debug)]
pub enum BudState{
    DormantBud,
    ActiveBud,
    BranchingSegment,
}
#[derive(Serialize, Deserialize,Clone)]
pub struct Node {
    pub bud_state: BudState,
    pub index: i32,
    pub parent: i32,
    pub order: i32,
    pub main_child: i32,
    pub secondary_child: i32,
    pub data: Data,
    #[serde(with = "Matrix4Def")]
    pub transformation: Mat4,
    pub segments: Vec<Segment>,
    pub segments_amount: i32,
    pub settings: Settings
}
impl Node{
    pub fn new(index:i32, parent: i32,order: i32,segments_amount:i32,bud_state: BudState,settings: Settings) -> Node{
        Node{
            bud_state,
            index,
            parent,
            order,
            main_child:-1,
            secondary_child:-1,
            data: Data::new(order,settings.init_auxin, settings.init_pin),
            transformation: Mat4::identity(),
            segments: vec![],
            segments_amount,
            settings
        }
    }
    fn get_children(&self) -> Vec<usize>{
        let mut result: Vec<usize> = vec![];
        if self.main_child>0{
            result.push(self.main_child as usize);
        }
        if self.secondary_child>0{
            result.push(self.secondary_child as usize);
        }
        return result;
    }
    pub fn get_out_data(&self) -> Data{
        match self.segments.len(){
            0 => {self.data.clone()}
            _ => {self.segments[0].data.clone()}
        }
    }
    pub fn activate_child(&self)-> bool{
        return false;
    }
    
    fn add_segment(&mut self){
        self.segments.push(Segment::new(self.order,self.settings.init_auxin, self.settings.init_pin));
    }

    fn segment_flow(&mut self,old_node:&Node){
        let dt = self.settings.dt;
        if self.segments.len()>0{
            for i in 0..(self.segments.len()-1){
                let data = &old_node.segments[i].data;
                let data_other = &old_node.segments[i+1].data;
                self.segments[i].data.auxin_update(dt*(outflow(&data)+inflow(&data_other)+decay(&data,&self.settings)+segment_production(&data, &self.settings)));
                self.segments[i].data.pin_update(dt*(pin_production(&data,&self.settings)+pin_decay(&data,&self.settings)));
                self.segments[i].data.auxin_flow=-outflow(&data);
                
            };
            let i = self.segments.len()-1;
            let data = &old_node.segments[i].data;
            let data_other = &old_node.data;
            self.segments[i].data.auxin_update(dt*(outflow(&data)+inflow(&data_other)+decay(&data,&self.settings)+segment_production(&data, &self.settings)));
            self.segments[i].data.pin_update(dt*(pin_production(&data,&self.settings)+pin_decay(&data,&self.settings)));
            self.segments[i].data.auxin_flow=-outflow(&data);
        }
    }

    fn gain_flow(&mut self,old_node:&Node) -> Result<(),()>{
        let dt = self.settings.dt;
        let gain = match self.bud_state{
            BudState::DormantBud => {
                self.settings.dormant_gain
            }
            BudState::ActiveBud => {
                self.settings.active_gain
            }
            BudState::BranchingSegment => {
                return Err(());
            }
        };
        self.segment_flow(old_node);
        self.data.auxin_update(dt*(outflow(&old_node.data)+production(&old_node.data,gain)+decay(&old_node.data,&self.settings)));
        self.data.pin_update(dt*(pin_production(&old_node.data,&self.settings)+pin_decay(&old_node.data,&self.settings)));
        self.data.auxin_flow=-outflow(&old_node.data);
        Ok(())
    }

    fn move_flow(&mut self, old_node: &Node,main_child: &Node, secondary_child: &Node) -> Result<(),()> {
        let dt = self.settings.dt;
        match self.bud_state{
            BudState::DormantBud => {
                return Err(());
            }
            BudState::ActiveBud => {
                return Err(());
            }
            BudState::BranchingSegment => {}
        };
        self.segment_flow(&old_node);
        self.data.auxin_update(dt*(outflow(&old_node.data)+inflow(&main_child.get_out_data())+inflow(&secondary_child.get_out_data())+decay(&old_node.data,&self.settings)+segment_production(&old_node.data, &self.settings)));
        self.data.pin_update(dt*(pin_production(&old_node.data,&self.settings)+pin_decay(&old_node.data,&self.settings)));
        self.data.auxin_flow=-outflow(&old_node.data);
        Ok(())
    }

}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut r = write!(f," index {}\n parent {}\n main child {} \n secondary child {}\n type {:?}\n segnents\n",self.index,self.parent,self.main_child,self.secondary_child,self.bud_state);
        for segment in &self.segments{
            r = write!(f,"{:#?}",segment.data);
        }
        r = write!(f,"{:#?}",self.data);
        return r;
    }
}
//use to render trees
#[derive(Clone)]
pub struct TreeRenderData{
    pub branching_angle:f32,
    pub divergence_angle:f32,
    pub semgent_length:i32,
    pub transformation: Mat4,
}
#[derive(Serialize, Deserialize,Clone)]
pub struct Tree{
    pub tip_index: i32,
    pub segments_amount: i32,
    #[serde(with = "Matrix4Def")]
    pub transformation: Mat4,
    pub settings: Settings,
    pub nodes : Vec<Node>,
    orders_indexed: Vec<Vec<usize>>,
    
}
impl Tree{
    pub fn new(settings: &Settings)-> Tree{
        let segments_amount= settings.segments_amount;
        let mut main =Node::new(0, -1, 0,segments_amount,BudState::ActiveBud,settings.clone());
        main.secondary_child=1;
        Tree{
            tip_index:0,
            segments_amount,
            transformation: Mat4::identity(),
            settings:settings.clone(),
            nodes: vec![main,Node::new(1, 0, 1,segments_amount,BudState::DormantBud,settings.clone()),],
            orders_indexed: vec![vec![0],vec![1],vec![]]
        }
    }
    pub fn new_settings(&mut self,settings:Settings){
        self.settings=settings.clone();
        for node in &mut self.nodes{
            node.settings=settings.clone();
        }
    }

    pub fn extend_node(&mut self,node_index:usize) -> NodeExtensionResult{
        match self.nodes[node_index].bud_state {
            BudState::ActiveBud =>{
                let mut segments_amount = self.settings.segments_amount;
                self.nodes[node_index].add_segment();
                if self.nodes[node_index].segments.len()==segments_amount as usize-1{
                    self.nodes[node_index].bud_state=BudState::BranchingSegment;
                    self.add_main_node(node_index);
                    NodeExtensionResult::ExtensionFinished
                }
                else{NodeExtensionResult::Extended}
            }
            _ =>{
                NodeExtensionResult::FullBefore
            }
        }
    }

    fn cache_index(&mut self, index: usize,order:usize){
        while order>=self.orders_indexed.len(){
            self.orders_indexed.push(vec![]);
        }
        self.orders_indexed[order].push(index);
    }
    pub fn extend_main(&mut self){
        let new_index = self.nodes.len() as i32;
        self.extend_node(self.tip_index as usize);
    }
    fn add_main_node(&mut self, node_index:usize) ->Result<(),&'static str>{
        let mut curr_index = node_index;
        match self.nodes[curr_index].bud_state {
            BudState::BranchingSegment => {
                if self.nodes[curr_index].main_child!=-1{
                    return Err("already added")
                }
                let new_index = self.nodes.len() as i32;
                self.nodes.push(Node::new(new_index, curr_index as i32, self.nodes[curr_index].order,self.segments_amount,BudState::ActiveBud,self.settings.clone()));
                self.nodes.push(Node::new(new_index+1, new_index as i32, self.nodes[curr_index].order+1,self.segments_amount,BudState::DormantBud,self.settings.clone()));
                self.nodes[new_index as usize].secondary_child = new_index+1;
                self.nodes[curr_index].main_child=new_index;
                if self.tip_index==curr_index as i32{
                    self.tip_index=new_index;
                }
                self.cache_index(new_index as usize,self.nodes[curr_index].order as usize);
                self.cache_index((new_index+1) as usize,(self.nodes[curr_index].order+1) as usize);
                return Ok(());
            }
            _ =>{return Err("still growing")}
        }

    }
    fn activate(&mut self, node_index:usize) -> Result<(),()>{
        match self.nodes[node_index].bud_state{
            BudState::DormantBud => {
                let new_index = self.nodes.len() as i32;
                self.nodes[node_index].bud_state=BudState::ActiveBud;
                self.nodes.push(Node::new(new_index, node_index as i32, self.nodes[node_index].order+1,self.segments_amount,BudState::DormantBud,self.settings.clone()));
                self.nodes[node_index].secondary_child=new_index;
                self.cache_index(new_index as usize,self.nodes[node_index].order as usize+1);
                Ok(())
            }
            _ => {Err(())}
        }

    }

    pub fn activate_secondary(&mut self, node_index:usize) -> Result<(),()>{
        let secondary_index = self.nodes[node_index].secondary_child;
        match self.nodes[node_index].bud_state{
            BudState::DormantBud => {Err(())},
            _ => {
                self.activate(secondary_index as usize)
            },
        }
    }
    pub fn extend_random_with_order(&mut self, order: usize) -> Result<(),()>{
        if order<self.orders_indexed.len(){
            let free = self.orders_indexed[order].iter().filter(|&x|
                match self.nodes[*x].bud_state{
                    BudState::ActiveBud => {true},
                    _ => {false},
                });
            let mut rng = rand::thread_rng();
            let _index=free.choose(&mut rng);
            match _index{
                None =>{
                    Err(())
                }
                Some(index)=>{
                    self.extend_node(*index);
                    Ok(())
                }
            }      
        }
        else{Err(())}
    }
    pub fn extend_random(&mut self){
        let free = self.nodes.iter().filter(
            |&x|match x.bud_state{
                BudState::ActiveBud => {true},
                _ => {false},
            });
        let mut rng = rand::thread_rng();
        let _node=free.choose(&mut rng);
        match _node{
            None =>{}
            Some(node)=>{
                self.extend_node(node.index as usize);
            }
        };  
    }
    pub fn activate_random_with_order(&mut self, order: usize) -> Result<(),()>{
        if order-1<self.orders_indexed.len(){
            let free= self.orders_indexed[order].iter().filter(
                |&x|match self.nodes[*x].bud_state{
                    BudState::DormantBud => {true},
                    _ => {false},
                });
                let mut rng = rand::thread_rng();
                let _index=free.choose(&mut rng);
                match _index{
                    None => {return Err(());}
                    Some(index)=>{
                        return self.activate(*index as usize)
                    }
                };           
        }
        Err(())
    }
    pub fn branch_random(&mut self) -> Result<(),()>{
        let free = self.nodes.iter().filter(
            |&x|match x.bud_state{
                BudState::DormantBud => {true},
                _ => {false},
            });
        let mut rng = rand::thread_rng();
        let _node=free.choose(&mut rng);
        match _node{
            None =>{return Err(());}
            Some(node)=>{
                return self.activate(node.index as usize);
            }
        };  
    }
    pub fn get_size(&self) -> usize{
        self.nodes.len()
    }
    pub fn update_transformations(&mut self, divergence_angle: f32,branching_angle: f32,segment_length: f32,initial_width: f32,order_width_influence: f32){
        let mut distance = segment_length*self.segments_amount as f32;
        self.nodes[0].transformation=Mat4::identity();
        for i in 0..self.get_size(){
            
            let base_transformation = self.nodes[i].transformation;
            let main_child = self.nodes[i].main_child;
            let secondary_child = self.nodes[i].secondary_child;
            let translation=Mat4::from_translation(Vector3 { x: 0., y: distance, z: 0. });
            
            if main_child!=-1{
                self.nodes[main_child as usize].transformation=
                base_transformation*translation;
            }
            if secondary_child!=-1{
                let s = initial_width/((self.nodes[i].order as f32)*order_width_influence+1.)*0.8;
                let mut distance= segment_length*(self.nodes[i].segments.len()) as f32;
                if main_child!=-1{distance+=segment_length;}
                let translation=Mat4::from_translation(Vector3 { x: 0., y: distance, z: 0. });
                self.nodes[secondary_child as usize].transformation=
                //Mat4::from_angle_z(radians(angle))*Mat4::from_angle_y(radians(divergence_angle*rng.gen_range(0..30) as f32))*base_transformation;
                base_transformation*translation*Mat4::from_angle_y(radians(divergence_angle*self.nodes[secondary_child as usize].index as f32))*
                Mat4::from_translation(vec3(-s,0.,0.))*
                Mat4::from_angle_z(radians(branching_angle));
            }
        }
        for i in 0..self.get_size(){
            let s = initial_width/((self.nodes[i].order as f32)*order_width_influence+1.);
            self.nodes[i].transformation=self.nodes[i].transformation*Mat4::from_nonuniform_scale(s,1.,s);
        }
    }
    pub fn update_tree_ref(tree:&Tree, new_tree: &mut Tree){
        let old_tree = tree;
        for i in 0..tree.get_size(){
            let node = &mut new_tree.nodes[i];
            match node.bud_state{
                BudState::DormantBud => {
                    node.gain_flow(&old_tree.nodes[i]);
                    if old_tree.nodes[node.parent as usize].activate_child(){
                        node.bud_state=BudState::ActiveBud;
                    }
                },
                BudState::ActiveBud => {
                    node.gain_flow(&old_tree.nodes[i]);
                },
                BudState::BranchingSegment =>{
                    node.move_flow(&old_tree.nodes[i],&old_tree.nodes[node.main_child as usize],&old_tree.nodes[node.secondary_child as usize]);
                }
            }
        }
    }
    pub fn update_tree_copy(tree:&Tree)->Tree{
        let mut new_tree = tree.clone();
        Self::update_tree_ref(&tree,&mut new_tree);
        new_tree
    }
    pub fn tree_difference(&self, other:&Tree) -> f32{
        let mut difference = 0.;
        for (node,other_node) in self.nodes.iter().zip(other.nodes.iter()){
            difference=((node.data.auxin-other_node.data.auxin).abs()+(node.data.pin-other_node.data.pin).abs()).max(difference);
            for (segment,other_segment) in node.segments.iter().zip(other_node.segments.iter()){
                difference=((segment.data.auxin-other_segment.data.auxin).abs()+(segment.data.pin-other_segment.data.pin).abs()).max(difference);
            }
        }
        difference
    }

    
    pub fn calculate_static_distribution(_tree: &Tree,precision: f32) -> Tree{
        let mut difference =100.;
        let mut i = 0;
        let mut old_tree = (*_tree).clone();
        let mut new_tree = (*_tree).clone();
        while difference>precision{
            difference=0.;
            for _ in 0..(100){
                new_tree=Tree::update_tree_copy(&old_tree);
                difference+=old_tree.tree_difference(&new_tree);
                old_tree=new_tree;
                i+=1;
            }
        }
        old_tree
    }

    pub fn main_stem_values(&self) -> Vec<(f32,f32)>{
        let mut result =vec![];

        let mut index = self.tip_index;
        while index !=-1{
            result.push((self.nodes[index as usize].data.auxin,self.nodes[index as usize].data.pin));
            for segment in self.nodes[index as usize].segments.iter().rev(){
                result.push((segment.data.auxin,segment.data.pin));

            }
            index=self.nodes[index as usize].parent;
        }
        result
    }
}
impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:#?}", self.nodes)
    }
}



pub fn random_growth(tree: &mut Tree, prob:f32){
    if rand::random::<f32>()>prob{
        tree.branch_random();
        tree.extend_random();
    }
    else{
        tree.extend_random();
    }

}

pub fn wild_type_week_11(_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=6;
    let mut tree = Tree::new(&settings);
    for _ in 0..10{
        tree.extend_main();
    }
    for i in 0..25{
        tree.activate_random_with_order(1);
        let do_it :f32 =random();
        if do_it<0.45 {tree.activate_random_with_order(1);}
        for _ in 0..3*i{
        tree.extend_random_with_order(1);
        }
        // if i==10{
        //     settings.segments_amount+=1;
        //     tree.new_settings(settings.clone())
        // }
        for _ in 0..5{
        tree.extend_main();
        }
    }

    tree
}
pub fn wild_type_week_11_2(_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=5;
    let mut tree = Tree::new(&settings);
    for _ in 0..(settings.segments_amount)*2{
        tree.extend_main();
    }
    for i in 0..20{
        tree.activate_random_with_order(1);
        for _ in 0..(((settings.segments_amount as f32)/2.)*i as f32).floor() as i32{
        tree.extend_random_with_order(1);
        }
        for _ in 0..(settings.segments_amount-1){
        tree.extend_main();
        }
    }

    tree
}
pub fn rnai60(_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=4;
    let mut tree = Tree::new(&settings);
    for _ in 0..(settings.segments_amount-1)*2{
        tree.extend_main();
    }
    for i in 0..25{
        tree.activate_random_with_order(1);
        if i>4{
            let do_it :f32 =random();
            if do_it<0.5 {tree.activate_random_with_order(2);}
            for _ in 0..(0.5*i as f32).ceil() as i32{
                tree.extend_random_with_order(2);
            }
        }
        // if i==10{
        //     settings.segments_amount+=1;
        //     tree.new_settings(settings.clone())
        // }
        let times = if random::<f32>()<0.6{ 3 } else{2};
        for _ in 0..times*i{
        tree.extend_random_with_order(1);
        }
        for _ in 0..(settings.segments_amount-1){
        tree.extend_main();
        }
    }
    tree
}
pub fn kanttarelli_week_11(_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=4;
    let mut tree = Tree::new(&settings);
    for _ in 0..4{
        tree.extend_main();
    }
    for i in 0..20{
        tree.activate_random_with_order(1);
        if i>4{
            for _ in 0..1*i{
                tree.extend_random_with_order(2);
            }
        }
        for _ in 0..2*i{
        tree.extend_random_with_order(1);
        }
        for _ in 0..2{
        tree.extend_main();
        }
    }
    tree
}
pub fn random_tree(nodes:i32,initial_size:i32,prob:f32,settings: &Settings) -> Tree{
    let mut tree = Tree::new(&settings);
    for  _ in 0..initial_size{    tree.extend_main();}
    //tree.branch_random();
    //print!("{:#?}",tree);
    for _ in 0..nodes{
        random_growth(&mut tree, prob);
    }
    for _ in 0..nodes/2{
        tree.extend_random();
    }
    let branching_angle = PI*0.2;
    let divergence_angle = PI*(137./180.);
    let segment_length = 2.;

    //tree.update_transformations(divergence_angle,branching_angle,segment_length);
    tree
}
pub fn pole(size:i32,_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=size+2;
    random_tree(0, size, 0.,&settings)
}
pub fn pole_internode_size(size:i32,internode_size:i32,_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=internode_size;
    random_tree(0, size, 0.,&settings)
}
pub fn pole_internode_size_active(size:i32,internode_size:i32,_settings: &Settings) -> Tree{
    let mut settings = _settings.clone();
    settings.segments_amount=internode_size;
    let mut tree=random_tree(0, size, 0.,&settings);
    for i in 1..tree.get_size(){
        if let BudState::DormantBud = tree.nodes[i].bud_state {
        match tree.nodes[tree.nodes[i].parent as usize].bud_state{
            BudState::ActiveBud => {},
            _=> {tree.activate(i);}
            }
        }
    }
    tree
}
