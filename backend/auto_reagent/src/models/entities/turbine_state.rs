use serde::{Deserialize, Serialize};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug,sqlx::FromRow,Deserialize,Serialize,Default)]
pub struct TurbineState{
    pub status: bool,
    pub id: i32,
    pub outlet_pressure: f64,
    pub pre_pressure: f64,
    pub frequency: f64,
    pub current: f64,
    pub safe_pressure: f64,
    pub power: bool,
    pub flow_rate: f64,
    pub flux: f64,
    pub open: f64,
}

impl TurbineState {
    pub fn new_rng(status:bool) -> TurbineState{
        let mut rng = rand::thread_rng();
        TurbineState{
            status,
            id: 0,
            outlet_pressure: rng.gen_range(0.0..50.0),
            pre_pressure: rng.gen_range(0.0..50.0),
            frequency: rng.gen_range(0.0..50.0),
            current: rng.gen_range(0.0..50.0),
            safe_pressure: rng.gen_range(0.0..50.0),
            power: true,
            flow_rate: rng.gen_range(0.0..50.0),
            flux: rng.gen_range(0.0..50.0),
            open: rng.gen_range(0.0..50.0),
        }
    }
    pub fn new(status:bool) -> TurbineState{
        TurbineState{
            status,
            id: 0,
            outlet_pressure: 0.0,
            pre_pressure: 0.0,
            frequency: 0.0,
            current: 0.0,
            safe_pressure: 0.0,
            power: true,
            flow_rate: 0.0,
            flux: 0.0,
            open: 0.0,
        }
    }
    pub fn to_turbine_state(hm:HashMap<String,String>)-> TurbineState {
        return TurbineState {
            status: true,
            outlet_pressure: hm.get("outlet_pressure").unwrap().parse().unwrap(),
            pre_pressure: hm.get("pre_pressure").unwrap().parse().unwrap(),
            frequency: hm.get("frequency").unwrap().parse().unwrap(),
            current: hm.get("current").unwrap().parse().unwrap(),
            safe_pressure: hm.get("safe_pressure").unwrap().parse().unwrap(),
            power: match hm.get("power").unwrap(){a if &a[..0]=="1" => true,_ => false},
            flow_rate: hm.get("flow_rate").unwrap().parse().unwrap(),
            flux: hm.get("flux").unwrap().parse().unwrap(),
            open: hm.get("open").unwrap().parse().unwrap(),
            id: hm.get("id").unwrap().parse().unwrap(),
        }
    }
}