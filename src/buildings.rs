use crate::sectors::SectorsInputs;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;
use crate::result::Results;
use crate::input::Input;
use crate::input::InputFields;
use crate::constants::buildings as constants;
use crate::constants::energy::evu_discount_heat_pump;

pub struct Buildings {
    inputs: InputsBuildings,
    pub results: ResultsBuildings,
    start_year: u32,
    end_year: u32,
}

impl Buildings{

    pub fn new(start_year: u32, end_year: u32) -> Buildings{
        return Buildings{
            inputs: InputsBuildings::new("buildings/inputs", start_year, end_year),
            results: ResultsBuildings::new("buildings/results", start_year, end_year),
            start_year: start_year,
            end_year: end_year,
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

        let thermal_energy_demand = (&n_inhabitants * &hot_water_demand_per_capita
            + &heat_demand_per_area * &floor_area) * 1e-3;
        results.thermal_energy_demand.set_year_values(year, &thermal_energy_demand);

        let electric_power_demand = &electric_power_demand_per_capita
            * &n_inhabitants * 1e-3;
        results.electric_power_demand.set_year_values(year, &electric_power_demand);


        // Energy Consumption of different heating types
        let thermal_energy_per_floor_area = &thermal_energy_demand * &floor_area;

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

        let consumption_heating_oil =
            (&energy_heating_oil_no_condendsing + &energy_heating_oil_with_condendsing)
            / constants::EnergySource::oil.calories;
        results.consumption_heating_oil.set_year_values(year, &consumption_heating_oil);

        let consumption_heating_gas = &energy_heating_gas / constants::EnergySource::gas.calories;
        results.consumption_heating_gas.set_year_values(year, &consumption_heating_gas);


        // Costs
        let costs_heating_oil =  &consumption_heating_oil * constants::EnergySource::oil.price;
        results.costs_heating_oil.set_year_values(year, &costs_heating_oil);

        let costs_heating_gas =  &consumption_heating_gas * constants::EnergySource::gas.price;
        results.costs_heating_gas.set_year_values(year, &costs_heating_gas);


        // Invests and Grants
        if year != self.start_year{

            // invest/grant heating
            macro_rules! implement_invest_calculation_heating{
                ($(($heat_type: ident, $heat_type_area: ident)),*) => {

                    let mut area_this_year: SectorsRawValues;
                    let mut area_prev_year: SectorsRawValues;


                    let mut invest_heat = SectorsRawValues::new();
                    let mut invest_grant_heat = SectorsRawValues::new();

                    $(
                        // invest heatings
                        area_this_year = self.inputs.$heat_type_area.get_year(year);
                        area_prev_year = self.inputs.$heat_type_area.get_year(year-1);

                        invest_heat = invest_heat
                            + (&area_this_year - &area_prev_year)
                            * constants::$heat_type.invest
                            * area_this_year.is_greater(&area_prev_year);

                        invest_grant_heat = invest_grant_heat
                            + (&area_this_year - &area_prev_year)
                            * constants::$heat_type.invest
                            * constants::$heat_type.grant
                            * area_this_year.is_greater(&area_prev_year);
                     )*

                    results.invest_heat.set_year_values(year, &invest_heat);
                    results.invest_grant_heat.set_year_values(year, &invest_grant_heat);
                }
            }


            implement_invest_calculation_heating!{
                (oil_no_condensing, area_heating_oil_no_condensing),
                (oil_with_condensing, area_heating_oil_with_condensing),
                (gas, area_heating_gas),
                (heat_pump, area_heating_heat_pump),
                (other, area_heating_other)
            }


            // invest/grant thermal heat
            let thermal_demand_this_year = inputs.heat_demand_per_area.get_year(year);
            let thermal_demand_prev_year = inputs.heat_demand_per_area.get_year(year - 1);

            let invest_thermal_energy_demand =
                (&thermal_demand_this_year - &thermal_demand_prev_year)
                * constants::energetic_restoration::invest
                * thermal_demand_this_year.is_greater(&thermal_demand_prev_year)
                * &floor_area;
            results.invest_thermal_energy_demand
                .set_year_values(year, &invest_thermal_energy_demand);

            let invest_grant_thermal_enery_demand =
                (&thermal_demand_this_year - &thermal_demand_prev_year)
                * constants::energetic_restoration::invest
                * constants::energetic_restoration::grant
                * thermal_demand_this_year.is_greater(&thermal_demand_prev_year)
                * &floor_area;

            results.invest_grant_thermal_enery_demand
                .set_year_values(year, &invest_grant_thermal_enery_demand);

        }
    }

    pub fn calculate_second_stage(
        &mut self,
        year: u32,
        aquisition_power_mix_price: &Results,
        ){

        let energy_heating_heat_pump = self.results.energy_heating_heat_pump
            .get_year(year);
        let aquisition_power_mix_price = aquisition_power_mix_price
            .get_year(year);

        let costs_heat_pump = &energy_heating_heat_pump
            * aquisition_power_mix_price * (1.0 - evu_discount_heat_pump);
        self.results.costs_heat_pump.set_year_values(year, &costs_heat_pump);
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

        pub struct ResultsBuildings{
            $(
                pub $field: SectorsResult,
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
                $(
                    results.extend(self.$field.get_results());
                 )*
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
    consumption_heating_gas,
    costs_heating_oil,
    costs_heating_gas,
    invest_heat,
    invest_thermal_energy_demand,
    invest_grant_heat,
    invest_grant_thermal_enery_demand,
    costs_heat_pump
}





#[cfg(test)]
mod tests{
    use super::*;
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;

    #[test]
    fn test_calculation() {
        let start_year = 2022 as u32;
        let end_year = 2025 as u32;
        let mut buildings = Buildings::new(start_year, end_year);

        buildings.inputs.floor_area_per_building.private.set_values(720.0);
        buildings.inputs.floor_area_per_building.industry.set_values(3000.0);
        buildings.inputs.floor_area_per_building.schools.set_values(8000.0);
        buildings.inputs.floor_area_per_building.public.set_values(300.0);

        buildings.inputs.n_buildings.private.set_values(5000.0);
        buildings.inputs.n_buildings.industry.set_values(200.0);
        buildings.inputs.n_buildings.schools.set_values(10.0);
        buildings.inputs.n_buildings.public.set_values(20.0);

        buildings.calculate(start_year);

        assert_eq!(buildings.results.floor_area.get_year_values(start_year),
                   [3600.0002,600.0,80.0,6.0000005]);
    }
}



