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
    results: ResultsMobility,
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
}



macro_rules! implement_inputs_mobility{
    ($($field:ident),*) => {

        struct InputsMobility{
            $(
                $field: SectorsInputs,
             )*
        }

        impl InputsMobility{
            fn new(id: &str, start_year: u32, end_year: u32) -> InputsMobility {
                return InputsMobility{
                    $(
                        $field: SectorsInputs::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                }
            }
        }

        impl InputFields for InputsMobility{

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


implement_inputs_mobility!{
    n_cars,
    n_bev,
    car_mean_traveld_distance
}


macro_rules! implement_results_mobility{
    ($($field:ident),*) => {

        struct ResultsMobility{
            $(
                $field: SectorsResult,
             )*
        }

        impl ResultsMobility{
            fn new(id: &str, start_year: u32, end_year: u32) -> ResultsMobility{
                return ResultsMobility{
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

implement_results_mobility!{
    cars_grant,
    bev_electric_power_demand,
    car_fuel_demand,
    car_fuel_costs
}
