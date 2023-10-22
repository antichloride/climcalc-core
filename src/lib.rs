use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

mod buildings;
use buildings::Buildings;
mod energy;
use energy::Energy;
mod mobility;
use mobility::Mobility;
mod input;
mod result;
use result::Results;
use input::Input;
mod sectors;
mod constants;
mod economy;
use economy::Economy;
mod stakeholders;
use stakeholders::Steakholders;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[wasm_bindgen]
pub struct Calculator {
    buildings: Buildings,
    mobility: Mobility,
    energy: Energy,
    economy: Economy,
    stakeholders: Steakholders,
    pub start_year: u32,
    pub end_year: u32,
}


/// This struct serves as interface. The wasm bridge is not optimized for
/// objects yet, therefore ids must be used instead.
///
/// The procedure works as follows:
/// 1. Initialize the calculator using the "new" function.
/// 2. Fill the input parameters using "set_input". This must be done for each
/// parameter individually.
/// 3. Add measures using "new_measure".
/// 4. Trigger the calculation using "calculate_over_years".
/// 5. Get result values (list with one value per year) using "get_results".
///
#[wasm_bindgen]
impl Calculator{
    /// A calculator is initialized by defining the year range. Note that
    /// start and end year are included. All input parameters will be
    /// initialized as zero.
    pub fn new(start_year: u32, end_year: u32) -> Calculator {
        let calculator = Calculator {
            buildings: Buildings::new(start_year, end_year),
            mobility: Mobility::new(start_year, end_year),
            energy: Energy::new(start_year, end_year),
            economy: Economy::new(start_year, end_year),
            stakeholders: Steakholders::new(start_year, end_year),
            start_year: start_year,
            end_year: end_year,
        };
        return calculator;
    }

    /// This function serves to trigger the calculation process.
    pub fn calculate_over_years(&mut self){
        for year in self.start_year..self.end_year + 1{
            self.calculate_first_stage(year);
            self.calculate_second_stage(year);
            self.calculate_emissions(year);
            self.economy.calculate(
                year,
                &self.buildings,
                &self.energy,
                &self.mobility,
            );
            self.stakeholders.calculate(
                year,
                &self.buildings,
                &self.energy,
                &self.mobility,
                &self.economy,
            )
        }
    }

    fn calculate_first_stage(&mut self, year:u32){
        self.buildings.calculate(year);
        self.mobility.calculate(year);
        self.energy.calculate(year);
    }

    fn calculate_second_stage(&mut self, year:u32){
        self.energy.calculate_second_stage(
            year,
            self.buildings.elec_dmd__G__W_h_per_a().get_year(year),
            self.buildings.cnsmp_elec_heat_pump__G__W_h_per_a().get_year(year),
            self.mobility.bev_elec_nrg_dmd__G__W_h_per_a().get_year(year),
            self.mobility.sl_nrg_dmd__G__W_h_per_a().get_year(year),
        );
        self.mobility.calculate_second_stage(
            year,
            self.energy.nrg_own_mix_price__m__eur_per_W_h().get_year(year),
        );
        self.buildings.calculate_second_stage(
            year,
            &self.energy.nrg_own_mix_price__m__eur_per_W_h()
        );
    }

    fn calculate_emissions(&mut self, year: u32){
        self.energy.calculate_emissions(year);
        self.buildings.calculate_emissions(year);
        self.mobility.calculate_emissions(year);
    }

    fn get_inputs(&self) -> Vec<&Input> {
        let mut list: Vec<&Input> = Vec::from([]);
        list.extend(self.buildings.get_inputs());
        list.extend(self.mobility.get_inputs());
        list.extend(self.energy.get_inputs());
        return list;
    }

    /// Returns the ids of all input parameters.
    pub fn list_input_ids(&self) -> JsValue{
        let vec: Vec<_> = self.get_inputs().iter().map(|&a| &a.id).collect();
        return to_value(&vec).unwrap();
    }

    fn get_input_by_id(&mut self, id: &str) -> Option<&mut Input>{
        let binding = id.to_string();
        let splitted_id: Vec<&str> = binding.split("/").collect();
        let binding_b = &splitted_id[2..].join("/");
        let remaining_id = binding_b.as_str();

        match splitted_id[0]{
            "buildings" => self.buildings.get_input_by_id(&remaining_id),
            "mobility" => self.mobility.get_input_by_id(&remaining_id),
            "energy" => self.energy.get_input_by_id(&remaining_id),
            _ => None,
        }
    }

    /// This function serves to populate the input parameters.
    ///
    /// Note that "set input" means the start year value of an input parameter.
    pub fn set_input(&mut self, id: &str, value: f64){
        let res = self.get_input_by_id(id);
        match res{
            Some(input) => {
                input.set_values(value);
            },
            None => (),
        }

    }

    /// This function returns the start year value of an input parameter.
    pub fn get_input(&mut self, id: &str) -> Option<f64>{
        let res = self.get_input_by_id(id);
        match res{
            Some(input) => {
                return Some(input.get_value());
            },
            None => None,
        }

    }

    fn get_results_list(&self) -> Vec<&Results> {
        let mut list: Vec<&Results> = Vec::from([]);
        list.extend(self.buildings.get_results());
        list.extend(self.mobility.get_results());
        list.extend(self.energy.get_results());
        list.extend(self.economy.get_results());
        list.extend(self.stakeholders.get_results());
        return list;
    }

    /// Returns the ids of all results.
    pub fn list_result_ids(&self) -> JsValue{
        let vec: Vec<_> = self.get_results_list().iter().map(|&a| &a.id).collect();
        return to_value(&vec).unwrap();
    }

    fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
        let binding = id.to_string();
        let splitted_id: Vec<&str> = binding.split("/").collect();
        let binding_b = &splitted_id[2..].join("/");
        let remaining_id = binding_b.as_str();
        println!("FIrst:{0}, Second:{1}", splitted_id[0], remaining_id);
        println!("{0}", id);

        match splitted_id[0]{
            "buildings" => self.buildings.get_results_by_id(&remaining_id),
            "mobility" => self.mobility.get_results_by_id(&remaining_id),
            "energy" => self.energy.get_results_by_id(&remaining_id),
            "economy" => self.economy.get_results_by_id(&remaining_id),
            "stakeholders" => self.stakeholders.get_results_by_id(&remaining_id),
            _ => None,
        }
    }

    /// Get a list of values (for each year) in a result.
    pub fn get_results(&mut self, id: &str) -> Vec<f64>{
        let res = self.get_results_by_id(id);
        match res{
            Some(results) => {
                return results.get_values().clone();
            },
            None => Vec::from([0.0,0.0,3.1415, 42.0]),
        }

    }

    /// Add a new measure to a input (given by an input id).
    /// start and end year defines the year range for which the measure applies.
    ///
    /// Note that start year is included.
    ///
    /// The delta defines by how much the input parameter is changed over the given range of years.
    /// The change is applied linear.
    ///
    /// I.e. if the number of cars is about to be reduced by 200 from 2025 to
    /// 2026 and there are 1200 cars in the calculator start year 2023 the
    /// values for n_cars will be:
    ///
    /// 2023->1200, 2024->1200, 2025->1100, 2026->1000, 2027->1000...
    pub fn new_measure(
        &mut self,
        id: &str,
        input_id: &str,
        start_year: u32,
        end_year: u32,
        delta: f64,
        ){
        let res = self.get_input_by_id(input_id);
        match res{
            Some(input) => {
                input.add_measure(id, start_year, end_year, delta);
            },
            None => (),
        }
    }

    /// For changing the start_year, end_year or delta of a given measure.
    /// Note that even if one of the does not change, all must be given.
    pub fn update_measure(
        &mut self,
        id: &str,
        input_id: &str,
        start_year: u32,
        end_year: u32,
        delta: f64,
        ){
        let res = self.get_input_by_id(input_id);
        match res{
            Some(input) => {
                input.update_measure(id, start_year, end_year, delta);
            },
            None => (),
        }

    }

    /// Deleting a measure. Note that this updates the values of the
    /// corresponding input.
    pub fn delete_measure(
        &mut self,
        id: &str,
        input_id: &str,
        ){
        let res = self.get_input_by_id(input_id);
        match res{
            Some(input) => {
                input.delete_measure(id);
            },
            None => (),
        }
    }

    /// List all measure ids that are present in the current calculator.
    pub fn list_measure_ids(&self) -> JsValue{
        let vec: Vec<_> = self.get_inputs().iter().map(|&a| a.list_measure_ids()).collect();
        return to_value(&vec).unwrap();
    }
}


#[cfg(test)]
mod integration_tests{
    use super::*;
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use crate::buildings::tests::buildings_test_case::create_buildings;
    use crate::energy::tests::energy_test_case::create_energy;
    use crate::mobility::tests::mobility_test_case::create_mobility;
    use web_sys::console;

    #[wasm_bindgen_test]
    fn test_calculator(){
        let start_year: u32 = 2022 as u32;
        let end_year: u32 = 2045 as u32;
        let buildings = create_buildings(start_year, end_year);
        let energy = create_energy(start_year, end_year);
        let mobility = create_mobility(start_year, end_year);
        let mut calculator = Calculator::new(start_year, end_year);
        calculator.buildings = buildings;
        calculator.energy = energy;
        calculator.mobility = mobility;
        calculator.calculate_over_years();
        console::log_1(&calculator.list_result_ids());
        console::log_1(&to_value(&calculator.get_results("economy/results/invest_heating_material")).unwrap());
        console::log_1(&to_value(&calculator.economy.invest_heating_material.values).unwrap());
        console::log_1(&to_value(&calculator.get_results("buildings/results/floor_A__k__m2/private")).unwrap());
        console::log_1(&to_value(&calculator.buildings.results.floor_A__k__m2.private.values).unwrap());
    }
}

#[cfg(test)]
mod unit_tests{
    use super::*;

    #[test]
    fn test_calculator_get_results_by_id() {
        let mut calculator: Calculator = Calculator::new(2022, 2025);
        assert!(!calculator.get_results_by_id("buildings/results/floor_A__k__m2/private").is_none());
        assert!(!calculator.get_results_by_id("economy/results/invest_heating_material").is_none());
    }
}
