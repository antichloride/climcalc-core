
use crate::sectors::SectorsInputs;
use crate::sectors::SectorsResult;
use crate::result::Results;
use crate::input::Input;
use crate::input::InputFields;
use crate::constants::buildings as constants;
use wasm_bindgen::prelude::*;
use std::ptr;
use std::ops;

pub struct Buildings {
    inputs: InputsBuildings,
    results: ResultsBuildings,
}

impl Buildings{

    pub fn new(start_year: u32, end_year: u32) -> Buildings{
        return Buildings{
            inputs: InputsBuildings::new("buildings/inputs", start_year, end_year),
            results: ResultsBuildings::new("buildings/results", start_year, end_year),
        }
    }


    pub fn get_inputs(& self) -> Vec<&Input>{
        return self.inputs.get_inputs();
    }

    pub fn get_input_by_id(&mut self, id: &str) -> Option<&mut Input>{
        return self.inputs.get_input_by_id(id);
    }


    pub fn get_results(& self) -> Vec<&Results>{
        return self.results.get_results();
    }

    pub fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
        return self.results.get_results_by_id(id);
    }
}


impl Buildings{

    pub fn calculate(&mut self, year: u32){
        let inputs = &self.inputs;
        let results = &mut self.results;

        let floor_area_per_building = self.inputs.floor_area_per_building.get_year(year); //in qm
        let n_buildings = self.inputs.n_buildings.get_year(year);
        let n_inhabitants = self.inputs.n_inhabitants.get_year(year);
        let heat_demand_per_area = self.inputs.heat_demand_per_area.get_year(year);
        let hot_water_demand_per_capita = self.inputs.hot_water_demand_per_capita.get_year(year); // in kWh/a
        let electric_power_demand_per_capita = self.inputs.electric_power_demand_per_capita.get_year(year);

        let floor_area = &floor_area_per_building * &n_buildings * 1e-3; // in kqm
        results.floor_area.set_year_values(year, &floor_area);

        let thermal_energy_demand = ((&n_inhabitants * &hot_water_demand_per_capita)
            + (&heat_demand_per_area * &floor_area) * 1e-3);
        results.thermal_energy_demand.set_year_values(year, &thermal_energy_demand);

        let electric_power_demand = &electric_power_demand_per_capita
            * &n_inhabitants * 1e-3;
        results.electric_power_demand.set_year_values(year, &electric_power_demand);


        // Energy Consumption of different heating types
        let thermal_energy_per_floor_area = thermal_energy_demand * floor_area;

        let area_heating_oil_no_condensing = self.inputs.area_heating_oil_no_condensing.get_year(year);
        let area_heating_oil_with_condensing = self.inputs.area_heating_oil_with_condensing.get_year(year);
        let area_heating_gas = self.inputs.area_heating_gas.get_year(year);
        let area_heating_heat_pump = self.inputs.area_heating_heat_pump.get_year(year);
        let area_heating_other = self.inputs.area_heating_other.get_year(year);


        let energy_heating_oil_no_condendsing = &area_heating_oil_no_condensing
            * &thermal_energy_per_floor_area / constants::oil_no_condensing.efficency;
        results.energy_heating_oil_no_condendsing.set_year_values(year, &energy_heating_oil_no_condendsing);

        let energy_heating_oil_with_condendsing = &area_heating_oil_with_condensing
            * &thermal_energy_per_floor_area / constants::oil_with_condensing.efficency;
        results.energy_heating_oil_with_condendsing.set_year_values(year, &energy_heating_oil_with_condendsing);

        let energy_heating_gas = &area_heating_gas
            * &thermal_energy_per_floor_area / constants::gas.efficency;
        results.energy_heating_gas.set_year_values(year, &energy_heating_gas);

        let energy_heating_heat_pump = &area_heating_heat_pump
            * &thermal_energy_per_floor_area / constants::heat_pump.efficency;
        results.energy_heating_heat_pump.set_year_values(year, &energy_heating_heat_pump);

        let energy_heating_other = &area_heating_other * &thermal_energy_per_floor_area;
        results.energy_heating_other.set_year_values(year, &energy_heating_other);

    }
}





macro_rules! implement_inputs_builidngs{
    ($($field:ident),*) => {

        struct InputsBuildings{
            $(
                $field: SectorsInputs,
             )*
        }

        impl InputsBuildings{
            fn new(id: &str, start_year: u32, end_year: u32) -> InputsBuildings {
                return InputsBuildings{
                    $(
                        $field: SectorsInputs::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                }
            }
        }

        impl InputFields for InputsBuildings{

            fn get_inputs(& self) -> Vec<&Input>{
                let mut inputs: Vec<&Input> = Vec::from([]);
                $(
                    inputs.extend(self.$field.get_inputs());
                 )*
                return inputs
            }

            fn get_input_by_id(&mut self, id: &str) -> Option<&mut Input>{
                let binding = id.to_string();
                let splitted_id: Vec<&str> = binding.split("/").collect();
                let binding_b = &splitted_id[1..].join("/");
                let remaining_id = binding_b.as_str();

                match splitted_id[0]{
                    $(
                        stringify!($field) => self.$field.get_input_by_id(remaining_id),
                     )*
                    _ => None,

                }

            }
        }
    }
}

implement_inputs_builidngs!{
    n_buildings,
    floor_area_per_building,
    heat_demand_per_area,
    n_inhabitants,
    hot_water_demand_per_capita,
    heat_demand,
    electric_power_demand_per_capita,
    area_heating_oil_no_condensing,
    area_heating_oil_with_condensing,
    area_heating_gas,
    area_heating_heat_pump,
    area_heating_other
}

macro_rules! implement_results_builidngs{
    ($($field:ident),*) => {

        struct ResultsBuildings{
            $(
                $field: SectorsResult,
             )*
        }

        impl ResultsBuildings{
            fn new(id: &str, start_year: u32, end_year: u32) -> ResultsBuildings{
                return ResultsBuildings{
                    $(
                        $field: SectorsResult::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                }
            }

            pub fn get_results(& self) -> Vec<&Results>{
                let mut results: Vec<&Results> = Vec::from([]);
                results.extend(self.floor_area.get_results());
                return results
            }

            fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
                let binding = id.to_string();
                let splitted_id: Vec<&str> = binding.split("/").collect();
                let binding_b = &splitted_id[1..].join("/");
                let remaining_id = binding_b.as_str();

                match splitted_id[0]{
                    $(
                        stringify!($field)=> self.$field.get_results_by_id(remaining_id),
                     )*
                    _ => None,

                }

            }
        }


    }
}

implement_results_builidngs!{
    floor_area,
    thermal_energy_demand,
    electric_power_demand,
    energy_heating_oil_no_condendsing,
    energy_heating_oil_with_condendsing,
    energy_heating_gas,
    energy_heating_heat_pump,
    energy_heating_other,
    consumption_heating_oil,
    consupmtion_heating_gas,
    costs_heating_oil,
    costs_heating_gas,
    invest_heat,
    invest_thermal_energy_demand,
    invest_grant_heat,
    invest_grant_thermal_enery_demand
}





#[cfg(test)]
mod tests{
    use super::*;
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_input_and_measure() {
        let start_year = 2022 as u32;
        let end_year = 2025 as u32;
        let input = Input::new(start_year, end_year);

        assert_eq!(input.values, Vec::from([0.0,0.0,0.0,0.0]));
    }
}



