use crate::vec_tree::*;

pub fn outflow(data: &Data) -> f32{
    -data.auxin*data.pin
}

pub fn inflow(data: &Data) -> f32{
    data.auxin*data.pin
}
pub fn production(data: &Data,gain:f32) -> f32{
    gain
}
pub fn segment_production(data: &Data,settings: &Settings) -> f32{
    settings.segment_gain
}
pub fn decay(data:&Data,settings: &Settings) -> f32{
    -data.auxin*settings.decay
}


pub fn pin_production(data: &Data,settings: &Settings) -> f32{
    let (flow_production,static_production) = settings.pin_production;
    //data.auxin_flow/(10.+data.auxin_flow)*flow_production+static_production
    data.auxin_flow/(10.+data.auxin_flow)*flow_production+static_production
}
pub fn pin_decay(data:&Data,settings: &Settings) -> f32{
    -data.pin*settings.pin_decay
}
/*
dA/dt = -A*T_0+A*T_1 + p-A*d_A
dT/dt = dA/(10+dA)*P_T+P2_T-P*d_T


 */