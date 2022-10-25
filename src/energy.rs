use crate::sectors::SectorsInputs;
use crate::sectors::SectorsResult;
use crate::sectors::SectorsRawValues;
use crate::result::Results;
use crate::input::Input;
use crate::input::InputFields;
use crate::constants::energy as constants;

pub struct Energy {
    pub inputs: InputsEnergy,
    pub results: ResultsEnergy,
    start_year: u32,
    end_year: u32,
}

impl Energy{

    pub fn new(start_year: u32, end_year: u32) -> Energy{
        return Energy{
            inputs: InputsEnergy::new("energy/inputs", start_year, end_year),
            results: ResultsEnergy::new("energy/results", start_year, end_year),
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


impl Energy{
    pub fn calculate(&mut self, year: u32){

        let roof_area = self.inputs.roof_area.get_year(year);
        let roof_solar_sutable_area = self.inputs.roof_solar_sutable_area
            .get_year(year);
        let solar_roof_installed_power_peak = self.inputs
            .solar_roof_installed_power_peak.get_year(year);
        let solar_roof_portion_self_consumption = self.inputs
            .solar_roof_portion_self_consumption.get_year(year);

        let solar_roof_power_peak_potential = &roof_area
            * &roof_solar_sutable_area * constants::solar_roof.power_per_area;
        self.results.solar_roof_power_peak_potential
            .set_year_values(year, &solar_roof_power_peak_potential);

        let solar_roof_installed_power = &solar_roof_installed_power_peak
            * constants::solar_roof.peak_power_to_mean_power * 1e-3;
        self.results.solar_roof_installed_power
            .set_year_values(year, &solar_roof_installed_power);

        let solar_roof_self_consumption = &solar_roof_installed_power
            * &solar_roof_portion_self_consumption;
        self.results.solar_roof_self_consumption
            .set_year_values(year, &solar_roof_self_consumption);

        if year != self.start_year{

            let solar_roof_installed_power_peak_this_year = self.inputs
                .solar_roof_installed_power_peak.get_year(year);
            let solar_roof_installed_power_peak_prev_year = self.inputs
                .solar_roof_installed_power_peak.get_year(year - 1);

            let solar_roof_invest =
                (&solar_roof_installed_power_peak_this_year
                 - &solar_roof_installed_power_peak_prev_year)
                * constants::solar_roof.invest * 1e-3
                * solar_roof_installed_power_peak_this_year
                    .is_greater(&solar_roof_installed_power_peak_prev_year);
            self.results.solar_roof_invest
                .set_year_values(year, &solar_roof_invest);

            let solar_roof_grant =
                (&solar_roof_installed_power_peak_this_year
                 - &solar_roof_installed_power_peak_prev_year)
                * constants::solar_roof.grant * 1e-3
                * solar_roof_installed_power_peak_this_year
                    .is_greater(&solar_roof_installed_power_peak_prev_year);
            self.results.solar_roof_grant
                .set_year_values(year, &solar_roof_grant);

            let mut solar_roof_operation_costs = SectorsRawValues::new();

            for year_i in self.start_year..year{
                solar_roof_operation_costs = solar_roof_operation_costs
                    + self.results.solar_roof_operation_costs.get_year(year_i);
            }

            solar_roof_operation_costs = solar_roof_operation_costs
                + &solar_roof_invest * constants::solar_roof.operation_costs;
            self.results.solar_roof_operation_costs
                .set_year_values(year, &solar_roof_operation_costs)

        }

        let solar_roof_buyback = (&solar_roof_installed_power
            - &solar_roof_self_consumption)
            * constants::solar_roof.buyback_price;
        self.results.solar_roof_buyback
            .set_year_values(year, &solar_roof_buyback);

    }
}



macro_rules! implement_inputs_energy{
    ($($field:ident),*) => {

        pub struct InputsEnergy{
            $(
                $field: SectorsInputs,
             )*
        }

        impl InputsEnergy{
            fn new(id: &str, start_year: u32, end_year: u32) -> InputsEnergy {
                return InputsEnergy{
                    $(
                        $field: SectorsInputs::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                }
            }
        }

        impl InputFields for InputsEnergy{

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


implement_inputs_energy!{
    roof_area,
    roof_solar_sutable_area,
    solar_roof_installed_power_peak,
    solar_roof_portion_self_consumption
}


macro_rules! implement_results_energy{
    ($($field:ident),*) => {

        pub struct ResultsEnergy{
            $(
                $field: SectorsResult,
             )*
            pub aquisition_power_mix_price: Results,
        }

        impl ResultsEnergy{
            fn new(id: &str, start_year: u32, end_year: u32) -> ResultsEnergy{
                return ResultsEnergy{
                    $(
                        $field: SectorsResult::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                    aquisition_power_mix_price: Results::new(id.to_owned()+"/aquisition_power_mix_price", start_year, end_year),
                }
            }

            pub fn get_results(& self) -> Vec<&Results>{
                let mut results: Vec<&Results> = Vec::from([]);
                $(
                    results.extend(self.$field.get_results());
                 )*
                results.push(&self.aquisition_power_mix_price);
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
                    "aquisition_power_mix_price"=> Some(&mut self.aquisition_power_mix_price),
                    _ => None,

                }

            }
        }


    }
}

implement_results_energy!{
    solar_roof_power_peak_potential,
    solar_roof_installed_power,
    solar_roof_self_consumption,
    solar_roof_invest,
    solar_roof_grant,
    solar_roof_operation_costs,
    solar_roof_buyback,
    solar_roof_costs,
    solar_roof_costs_per_kwh,
    solar_landscape_installed_power_peak,
    solar_landscape_area,
    solar_landscape_installed_power,
    solar_landscape_invest,
    solar_landscape_grant,
    solar_landscape_operation_costs,
    solar_landscape_production_costs,
    direct_aquisition_renable_energies_costs
}
