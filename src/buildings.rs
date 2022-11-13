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



// Implement getter functions

impl Buildings{
    pub fn total_heat_dmd__G__W_h_per_a(&self) -> &SectorsResult{
        return &self.results.total_heat_dmd__G__W_h_per_a;
    }
    pub fn elec_dmd__G__W_h_per_a(&self) -> &SectorsResult{
        return &self.results.elec_dmd__G__W_h_per_a;
    }
    pub fn cnsmp_elec_heat_pump__G__W_h_per_a(&self) -> &SectorsResult{
        return &self.results.cnsmp_elec_heat_pump__G__W_h_per_a;
    }

}



impl Buildings{

    pub fn calculate(&mut self, year: u32){
        let inputs = &self.inputs;
        let results = &mut self.results;

        let floor_A_building__m2 = self.inputs.floor_A_building__m2.get_year(year); //in qm
        let n_buildings = self.inputs.n_buildings.get_year(year);
        let n_inhabitants__k__ = self.inputs.n_inhabitants__k__.get_year(year);
        let heat_dmd__k__W_h_per_m2 = self.inputs.heat_dmd__k__W_h_per_m2.get_year(year);
        let hot_water_dmd__k__W_h_per_m2 = self.inputs.hot_water_dmd__k__W_h_per_m2.get_year(year); // in kWh/a
        let elec_dmd_capita__k_W_h_per_a = self.inputs.elec_dmd_capita__k_W_h_per_a.get_year(year);

        let floor_A__k__m2 = &floor_A_building__m2 * &n_buildings * 1e-3; // in kqm
        results.floor_A__k__m2.set_year_values(year, &floor_A__k__m2);

        let total_heat_dmd__G__W_h_per_a = (&n_inhabitants__k__ * &hot_water_dmd__k__W_h_per_m2
            + &heat_dmd__k__W_h_per_m2 * &floor_A__k__m2) * 1e-3;
        results.total_heat_dmd__G__W_h_per_a.set_year_values(year, &total_heat_dmd__G__W_h_per_a);

        let elec_dmd__G__W_h_per_a = &elec_dmd_capita__k_W_h_per_a
            * &n_inhabitants__k__ * 1e-3;
        results.elec_dmd__G__W_h_per_a.set_year_values(year, &elec_dmd__G__W_h_per_a);


        // Energy Consumption of different heating types
        let thermal_energy_per_floor_A = &total_heat_dmd__G__W_h_per_a / &floor_A__k__m2;

        let A_heat_oil__k__m2 = self.inputs.A_heat_oil__k__m2.get_year(year);
        let A_heat_oil_condensing__k__m2 = self.inputs.A_heat_oil_condensing__k__m2.get_year(year);
        let A_heat_gas__k__m2 = self.inputs.A_heat_gas__k__m2.get_year(year);
        let A_heat_heat_pump__k__m2 = self.inputs.A_heat_heat_pump__k__m2.get_year(year);
        let A_heating_other = &floor_A__k__m2 - &A_heat_oil__k__m2
            - &A_heat_oil_condensing__k__m2 - &A_heat_gas__k__m2
            - &A_heat_heat_pump__k__m2;


        let cnsmp_oil__G__W_h_per_a = &A_heat_oil__k__m2
            * &thermal_energy_per_floor_A / constants::oil_no_condensing.efficency;
        results.cnsmp_oil__G__W_h_per_a.set_year_values(year, &cnsmp_oil__G__W_h_per_a);

        let cnsmp_oil_condensing__G__W_h_per_a = &A_heat_oil_condensing__k__m2
            * &thermal_energy_per_floor_A / constants::oil_with_condensing.efficency;
        results.cnsmp_oil_condensing__G__W_h_per_a.set_year_values(year, &cnsmp_oil_condensing__G__W_h_per_a);

        let cnsmp_gas__G__W_h_per_a = &A_heat_gas__k__m2
            * &thermal_energy_per_floor_A / constants::gas.efficency;
        results.cnsmp_gas__G__W_h_per_a.set_year_values(year, &cnsmp_gas__G__W_h_per_a);

        let cnsmp_elec_heat_pump__G__W_h_per_a = &A_heat_heat_pump__k__m2
            * &thermal_energy_per_floor_A / constants::heat_pump.efficency;
        results.cnsmp_elec_heat_pump__G__W_h_per_a.set_year_values(year, &cnsmp_elec_heat_pump__G__W_h_per_a);

        let cnsmp_other__G__W_h_per_a = &A_heating_other * &thermal_energy_per_floor_A;
        results.cnsmp_other__G__W_h_per_a.set_year_values(year, &cnsmp_other__G__W_h_per_a);

        let cnsmp_oil__M__L =
            (&cnsmp_oil__G__W_h_per_a + &cnsmp_oil_condensing__G__W_h_per_a)
            / constants::EnergySource::oil.energy_density;
        results.cnsmp_oil__M__L.set_year_values(year, &cnsmp_oil__M__L);

        let cnsmp_gas__M__m3 = &cnsmp_gas__G__W_h_per_a / constants::EnergySource::gas.energy_density;
        results.cnsmp_gas__M__m3.set_year_values(year, &cnsmp_gas__M__m3);


        // Costs
        let costs_oil__M__eur =  &cnsmp_oil__M__L * constants::EnergySource::oil.price;
        results.costs_oil__M__eur.set_year_values(year, &costs_oil__M__eur);

        let costs_gas__M__eur =  &cnsmp_gas__M__m3 * constants::EnergySource::gas.price;
        results.costs_gas__M__eur.set_year_values(year, &costs_gas__M__eur);


        // Invests and Grants
        if year != self.start_year{

            // invest/grant heating
            macro_rules! implement_invest_calculation_heating{
                ($(($heat_type: ident, $heat_type_A: ident)),*) => {

                    let mut A_this_year: SectorsRawValues;
                    let mut A_prev_year: SectorsRawValues;


                    let mut invest_heat_sources__M__eur = SectorsRawValues::new();
                    let mut grant_heat_sources__M__eur = SectorsRawValues::new();

                    $(
                        // invest heatings
                        A_this_year = self.inputs.$heat_type_A.get_year(year);
                        A_prev_year = self.inputs.$heat_type_A.get_year(year-1);

                        invest_heat_sources__M__eur = invest_heat_sources__M__eur
                            + (&A_this_year - &A_prev_year)
                            * constants::$heat_type.invest
                            * A_this_year.is_greater(&A_prev_year);

                        grant_heat_sources__M__eur = grant_heat_sources__M__eur
                            + (&A_this_year - &A_prev_year)
                            * constants::$heat_type.invest
                            * constants::$heat_type.grant
                            * A_this_year.is_greater(&A_prev_year);
                     )*

                    invest_heat_sources__M__eur = invest_heat_sources__M__eur * &heat_dmd__k__W_h_per_m2;
                    grant_heat_sources__M__eur = grant_heat_sources__M__eur * &heat_dmd__k__W_h_per_m2;

                    results.invest_heat_sources__M__eur.set_year_values(year, &invest_heat_sources__M__eur);
                    results.grant_heat_sources__M__eur.set_year_values(year, &grant_heat_sources__M__eur);
                }
            }


            implement_invest_calculation_heating!{
                (oil_no_condensing, A_heat_oil__k__m2),
                (oil_with_condensing, A_heat_oil_condensing__k__m2),
                (gas, A_heat_gas__k__m2),
                (heat_pump, A_heat_heat_pump__k__m2)
                // TODO: check how A_heat_other must be added here
            }


            // invest/grant thermal heat
            let thermal_demand_this_year = inputs.heat_dmd__k__W_h_per_m2.get_year(year);
            let thermal_demand_prev_year = inputs.heat_dmd__k__W_h_per_m2.get_year(year - 1);

            let invest_renovation =
                (&thermal_demand_this_year - &thermal_demand_prev_year) // TODO: use heat demand
                                                                        // per A, not toal heat
                                                                        // demand
                * constants::energetic_restoration::invest
                * thermal_demand_this_year.is_greater(&thermal_demand_prev_year)
                * &floor_A__k__m2;
            results.invest_energetic_renovation__M__eur
                .set_year_values(year, &invest_renovation);

            let grant_energetic_renovation__M__eur =
                (&thermal_demand_this_year - &thermal_demand_prev_year)
                * constants::energetic_restoration::invest
                * constants::energetic_restoration::grant
                * thermal_demand_this_year.is_greater(&thermal_demand_prev_year)
                * &floor_A__k__m2;

            results.grant_energetic_renovation__M__eur
                .set_year_values(year, &grant_energetic_renovation__M__eur);

        }
    }

    pub fn calculate_second_stage(
        &mut self,
        year: u32,
        custom_power_mix_price: &Results,
        ){

        let cnsmp_elec_heat_pump__G__W_h_per_a = self.results.cnsmp_elec_heat_pump__G__W_h_per_a
            .get_year(year);
        let custom_power_mix_price = custom_power_mix_price
            .get_year(year);

        let costs_heat_pump__M__eur = &cnsmp_elec_heat_pump__G__W_h_per_a
            * custom_power_mix_price * (1.0 - evu_discount_heat_pump);
        self.results.costs_heat_pump__M__eur.set_year_values(year, &costs_heat_pump__M__eur);
    }

    pub fn calculate_emissions(&mut self, year: u32){
        let cnsmp_oil__M__L = self.results.cnsmp_oil__M__L
            .get_year(year);
        let cnsmp_gas__M__m3 = self.results.cnsmp_gas__M__m3
            .get_year(year);

        let ems_oil__k__to_coe = cnsmp_oil__M__L
            * constants::EnergySource::oil.emission;
        self.results.ems_oil__k__to_coe.set_year_values(year, &ems_oil__k__to_coe);

        let ems_gas__k__to_coe = cnsmp_gas__M__m3
            * constants::EnergySource::gas.emission;
        self.results.ems_gas__k__to_coe.set_year_values(year, &ems_gas__k__to_coe);

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
    floor_A_building__m2,
    heat_dmd__k__W_h_per_m2,
    n_inhabitants__k__,
    hot_water_dmd__k__W_h_per_m2,
    elec_dmd_capita__k_W_h_per_a,
    A_heat_oil__k__m2,
    A_heat_oil_condensing__k__m2,
    A_heat_gas__k__m2,
    A_heat_heat_pump__k__m2
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
    floor_A__k__m2,
    total_heat_dmd__G__W_h_per_a,
    elec_dmd__G__W_h_per_a,
    cnsmp_oil__G__W_h_per_a,
    cnsmp_oil_condensing__G__W_h_per_a,
    cnsmp_gas__G__W_h_per_a,
    cnsmp_elec_heat_pump__G__W_h_per_a,
    cnsmp_other__G__W_h_per_a,
    cnsmp_oil__M__L,
    cnsmp_gas__M__m3,
    costs_oil__M__eur,
    costs_gas__M__eur,
    invest_heat_sources__M__eur,
    invest_energetic_renovation__M__eur,
    grant_heat_sources__M__eur,
    grant_energetic_renovation__M__eur,
    costs_heat_pump__M__eur,
    ems_oil__k__to_coe,
    ems_gas__k__to_coe
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

        buildings.inputs.floor_A_building__m2.private.set_values(720.0);
        buildings.inputs.floor_A_building__m2.industry.set_values(3000.0);
        buildings.inputs.floor_A_building__m2.schools.set_values(8000.0);
        buildings.inputs.floor_A_building__m2.public.set_values(300.0);

        buildings.inputs.n_buildings.private.set_values(5000.0);
        buildings.inputs.n_buildings.industry.set_values(200.0);
        buildings.inputs.n_buildings.schools.set_values(10.0);
        buildings.inputs.n_buildings.public.set_values(20.0);

        buildings.calculate(start_year);

        assert_eq!(buildings.results.floor_A.get_year_values(start_year),
                   [3600.0002,600.0,80.0,6.0000005]);
    }
}



