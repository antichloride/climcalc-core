use wasm_bindgen::prelude::*;
use std::ops;

mod buildings;
use buildings::buildings::Buildings;
use buildings::buildings::Input;

#[wasm_bindgen]
pub struct Calculator {
    buildings: Buildings,
    start_year: u32,
    end_year: u32,
}



#[wasm_bindgen]
impl Calculator{
    pub fn new(start_year: u32, end_year: u32) -> Calculator {
        let calculator = Calculator {
            buildings: Buildings::new(start_year, end_year),
            start_year: start_year,
            end_year: end_year,
        };
        calculator
    }

    pub fn calculate_over_years(&mut self){
        for year in self.start_year..self.end_year + 1{
            self.buildings.calculate(year);
        }
    }

    fn get_inputs(&self) -> Vec<&Input> {
        return self.buildings.get_inputs();
    }

}

