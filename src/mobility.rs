use crate::sectors::SectorsInputs;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;
use crate::result::Results;
use crate::input::Input;
use crate::input::InputFields;
use crate::constants::mobility as constants;
use crate::constants::buildings::EnergySource::oil;

pub struct Mobility {
    inputs: InputsMobility,
    pub results: ResultsMobility,
    start_year: u32,
    end_year: u32,
}

impl Mobility{

    pub fn new(start_year: u32, end_year: u32) -> Mobility{
        return Mobility{
            inputs: InputsMobility::new("mobility/inputs", start_year, end_year),
            results: ResultsMobility::new("mobility/results", start_year, end_year),
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


impl Mobility{
    pub fn calculate(&mut self, year: u32){

        let n_bev = self.inputs.n_bev.get_year(year);
        let n_cars = self.inputs.n_cars.get_year(year);
        let car_mean_traveld_distance = self.inputs.car_mean_traveld_distance
            .get_year(year);

        if year != self.start_year{
            let n_bev_this_year = self.inputs.n_bev.get_year(year);
            let n_bev_prev_year = self.inputs.n_bev.get_year(year - 1);

            let cars_grant = (&n_bev_this_year - &n_bev_prev_year)
                * constants::bev.grant
                * n_bev_this_year.is_greater(&n_bev_prev_year);
            self.results.cars_grant.set_year_values(year, &cars_grant);
        }

        let bev_electric_power_demand = &n_bev * &car_mean_traveld_distance
            * constants::bev.consumption * 1e-2;
        self.results.bev_electric_power_demand
            .set_year_values(year, &bev_electric_power_demand);

        let car_fuel_demand = (&n_cars - &n_bev) * &car_mean_traveld_distance
            * constants::combustor.consumption * oil.calories * 1e-2;
        self.results.car_fuel_demand.set_year_values(year, &car_fuel_demand);

        let car_fuel_demand = &car_fuel_demand * constants::price_fuel;
        self.results.car_fuel_demand.set_year_values(year, &car_fuel_demand);

    }

    pub fn calculate_second_stage(
        &mut self,
        year: u32,
        purchased_energy_mix: SectorsRawValues,
        aquisition_power_mix_price: f32,
        ){

        let bev_electric_power_demand = self.results.bev_electric_power_demand
            .get_year(year);

        let bev_electric_power_demand = &bev_electric_power_demand
            * &purchased_energy_mix;
        self.results.bev_electric_power_demand
            .set_year_values(year, &bev_electric_power_demand);

        // Street lighning
        let n_laterns = self.inputs.n_laterns.get_year(year);
        let power_consumption_per_latern = self.inputs
            .power_consumption_per_latern.get_year(year);
        let maintanence_costs_per_lantern = self.inputs
            .maintanence_costs_per_lantern.get_year(year);

        let street_lightning_power_costs = n_laterns
            * power_consumption_per_latern
            * aquisition_power_mix_price / 100.0;
        self.results.street_lightning_power_costs
            .set_year_value(year, street_lightning_power_costs);

        let street_lighting_maintenance_costs = n_laterns
            * maintanence_costs_per_lantern;
        self.results.street_lighting_maintenance_costs
            .set_year_value(year, street_lighting_maintenance_costs);

        let street_lightning_total_costs =
            street_lightning_power_costs + street_lighting_maintenance_costs;
        self.results.street_lightning_total_costs
            .set_year_value(year, street_lightning_total_costs);

    }
}



macro_rules! implement_inputs_mobility{
    ($($field:ident),*) => {

        struct InputsMobility{
            $(
                $field: SectorsInputs,
             )*
            n_laterns: Input,
            power_consumption_per_latern: Input,
            maintanence_costs_per_lantern: Input,
        }

        impl InputsMobility{
            fn new(id: &str, start_year: u32, end_year: u32) -> InputsMobility {
                return InputsMobility{
                    $(
                        $field: SectorsInputs::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                        n_laterns: Input::new(
                            id.to_owned()+"/n_laterns",
                            start_year,
                            end_year,
                        ),
                        power_consumption_per_latern: Input::new(
                            id.to_owned()+"/power_consumption_per_latern",
                            start_year,
                            end_year,
                        ),
                        maintanence_costs_per_lantern: Input::new(
                            id.to_owned()+"/maintanence_costs_per_lantern",
                            start_year,
                            end_year,
                        ),
                }
            }
        }

        impl InputFields for InputsMobility{

            fn get_inputs(& self) -> Vec<&Input>{
                let mut inputs: Vec<&Input> = Vec::from([]);
                $(
                    inputs.extend(self.$field.get_inputs());
                 )*
                inputs.push(&self.n_laterns);
                inputs.push(&self.power_consumption_per_latern);
                inputs.push(&self.maintanence_costs_per_lantern);
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
                    n_laterns=> Some(&mut self.n_laterns),
                    power_consumption_per_latern=> Some(
                        &mut self.power_consumption_per_latern),
                    maintanence_costs_per_lantern=> Some(
                        &mut self.maintanence_costs_per_lantern),
                    _ => None,

                }

            }
        }
    }
}


implement_inputs_mobility!{
    n_cars,
    n_bev,
    car_mean_traveld_distance
}


macro_rules! implement_results_mobility{
    ($($field:ident),*) => {

        pub struct ResultsMobility{
            $(
                pub $field: SectorsResult,
             )*
            street_lightning_power_costs: Results,
            street_lighting_maintenance_costs: Results,
            street_lightning_total_costs: Results,
        }

        impl ResultsMobility{
            fn new(id: &str, start_year: u32, end_year: u32) -> ResultsMobility{
                return ResultsMobility{
                    $(
                        $field: SectorsResult::new(
                            id.to_owned()+"/"+stringify!($field),
                            start_year,
                            end_year
                        ),
                     )*
                        street_lightning_power_costs: Results::new(
                            id.to_owned()+"/street_lightning_power_costs",
                            start_year,
                            end_year,
                        ),
                        street_lighting_maintenance_costs: Results::new(
                            id.to_owned()+"/street_lighting_maintenance_costs",
                            start_year,
                            end_year,
                        ),
                        street_lightning_total_costs: Results::new(
                            id.to_owned()+"/street_lightning_total_costs",
                            start_year,
                            end_year,
                        ),
                }
            }

            pub fn get_results(& self) -> Vec<&Results>{
                let mut results: Vec<&Results> = Vec::from([]);
                $(
                    results.extend(self.$field.get_results());
                 )*
                results.push(&self.street_lightning_power_costs);
                results.push(&self.street_lighting_maintenance_costs);
                results.push(&self.street_lightning_total_costs);
                return results
            }

            fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
                let binding = id.to_string();
                let splitted_id: Vec<&str> = binding.split("/").collect();
                let binding_b = &splitted_id[1..].join("/");
                let remaining_id = binding_b.as_str();

                match splitted_id[0]{
                    $(
                        stringify!($field)=> self.$field
                            .get_results_by_id(remaining_id),
                     )*
                    "street_lightning_power_costs"=> Some(
                        &mut self.street_lightning_power_costs),
                    "street_lighting_maintenance_costs"=> Some(
                        &mut self.street_lighting_maintenance_costs),
                    "street_lightning_total_costs"=> Some(
                        &mut self.street_lightning_total_costs),
                    _ => None,

                }

            }
        }


    }
}

implement_results_mobility!{
    cars_grant,
    bev_electric_power_demand,
    car_fuel_demand,
    car_fuel_costs,
    bev_electric_power_costs
}
