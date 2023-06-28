#![allow(non_snake_case)]

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
}

impl Energy{

    pub fn new(start_year: u32, end_year: u32) -> Energy{
        return Energy{
            inputs: InputsEnergy::new("energy/inputs", start_year, end_year),
            results: ResultsEnergy::new("energy/results", start_year, end_year),
            start_year,
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
    pub fn nrg_own_mix_price__m__eur_per_W_h(&self) -> &SectorsResult{
        return &self.results.nrg_own_mix_price__m__eur_per_W_h;
    }
    pub fn sol_rf_invest__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_rf_invest__M__eur_per_a;
    }
    pub fn sol_os_invest__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_os_invest__M__eur_per_a;
    }
    pub fn sol_rf_grant__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_rf_grant__M__eur_per_a;
    }
    pub fn sol_os_grant__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_os_grant__M__eur_per_a;
    }
    pub fn sol_rf_om__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_rf_om__M__eur_per_a;
    }
    pub fn sol_os_om__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_rf_om__M__eur_per_a;
    }
    pub fn sol_rf_revenue__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_rf_revenue__M__eur_per_a;
    }
    pub fn sol_os_revenue__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.sol_rf_revenue__M__eur_per_a;
    }
    pub fn prchsd_renewable_nrg__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.prchsd_renewable_nrg__M__eur_per_a;
    }
    pub fn prchsd_nrg_mix_costs__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.prchsd_nrg_mix_costs__M__eur_per_a;
    }
}


impl Energy{
    pub fn calculate(&mut self, year: u32){

        // Solar Roof
        let rf_A__k__m2 = self.inputs.rf_A__k__m2.get_year(year);
        let sol_rf_suitable_A_part = self.inputs.sol_rf_suitable_A_part
            .get_year(year);
        let sol_rf_installed__M__Wp = self.inputs
            .sol_rf_installed__M__Wp.get_year(year);
        let sol_rf_self_cnsmp_part = self.inputs
            .sol_rf_self_cnsmp_part.get_year(year);

        let sol_rf_potential__M__Wp = &rf_A__k__m2 * &sol_rf_suitable_A_part
            * constants::solar_roof.power_per_area__k__Wp_per_m2;
        self.results.sol_rf_potential__M__Wp
            .set_year_values(year, &sol_rf_potential__M__Wp);

        let sol_rf_nrg__G__W_h_per_a = &sol_rf_installed__M__Wp
            * constants::solar_roof.Wp_to_W_h_per_a * 1e-3;
        self.results.sol_rf_nrg__G__W_h_per_a
            .set_year_values(year, &sol_rf_nrg__G__W_h_per_a);

        let sol_rf_self_cnsmp__G__W_h_per_a = &sol_rf_nrg__G__W_h_per_a
            * &sol_rf_self_cnsmp_part;
        self.results.sol_rf_self_cnsmp__G__W_h_per_a
            .set_year_values(year, &sol_rf_self_cnsmp__G__W_h_per_a);

        if year != self.start_year{

            let sol_rf_installed__M__Wp_this_year = self.inputs
                .sol_rf_installed__M__Wp.get_year(year);
            let sol_rf_installed__M__Wp_prev_year = self.inputs
                .sol_rf_installed__M__Wp.get_year(year - 1);

            let sol_rf_invest__M__eur_per_a =
                (&sol_rf_installed__M__Wp_this_year
                 - &sol_rf_installed__M__Wp_prev_year)
                * constants::solar_roof.invest__m__eur_per_Wp * 1e-3
                * sol_rf_installed__M__Wp_this_year
                    .is_greater(&sol_rf_installed__M__Wp_prev_year);
            self.results.sol_rf_invest__M__eur_per_a
                .set_year_values(year, &sol_rf_invest__M__eur_per_a);

            // The grant is calculated in euro per power peak
            let sol_rf_grant__M__eur_per_a =
                (&sol_rf_installed__M__Wp_this_year
                 - &sol_rf_installed__M__Wp_prev_year)
                * constants::solar_roof.grant__m__eur_per_Wp * 1e-3
                * sol_rf_installed__M__Wp_this_year
                    .is_greater(&sol_rf_installed__M__Wp_prev_year);
            self.results.sol_rf_grant__M__eur_per_a
                .set_year_values(year, &sol_rf_grant__M__eur_per_a);


            let sol_rf_om__M__eur_per_a_last_year =
                self.results.sol_rf_om__M__eur_per_a.get_year(year-1);
            let sol_rf_om__M__eur_per_a = &sol_rf_om__M__eur_per_a_last_year
                + &(
                    &sol_rf_invest__M__eur_per_a
                    * constants::solar_roof.operation_costs
                    );
            self.results.sol_rf_om__M__eur_per_a
                .set_year_values(year, &sol_rf_om__M__eur_per_a)

        }

        let sol_rf_revenue__M__eur_per_a =
            (&sol_rf_nrg__G__W_h_per_a
            - &sol_rf_self_cnsmp__G__W_h_per_a)
            * constants::solar_roof.buyback_price__m__eur_per_W_h;
        self.results.sol_rf_revenue__M__eur_per_a
            .set_year_values(year, &sol_rf_revenue__M__eur_per_a);


        // Solar Open Space

        let sol_os_installed_A__ha = self.inputs
            .sol_os_installed_A__ha.get_year(year);
        let sol_os_self_cnsmp_part = self.inputs
            .sol_os_self_cnsmp_part.get_year(year);

        let sol_os_installed__M__Wp = 1e-1
            * &sol_os_installed_A__ha
            * constants::solar_landscape.power_per_area__k__Wp_per_m2;
        self.results.sol_os_installed__M__Wp
            .set_year_values(year, &sol_os_installed__M__Wp);

        let sol_os_nrg__G__W_h_per_a = 1e-3
            * &sol_os_installed__M__Wp
            * constants::solar_landscape.Wp_to_W_h_per_a;
        self.results.sol_os_nrg__G__W_h_per_a
            .set_year_values(year, &sol_os_nrg__G__W_h_per_a);

        let sol_os_self_cnsmp__G__W_h_per_a = &sol_os_nrg__G__W_h_per_a
            * &sol_os_self_cnsmp_part;
        self.results.sol_os_self_cnsmp__G__W_h_per_a
            .set_year_values(year, &sol_os_self_cnsmp__G__W_h_per_a);

        if year != self.start_year{

            let sol_os_installed__M__Wp_this_year =
                self.results.sol_os_installed__M__Wp.get_year(year);
            let sol_os_installed__M__Wp_prev_year =
                self.results.sol_os_installed__M__Wp
                .get_year(year - 1);

            let sol_os_invest__M__eur_per_a = 1e-3
               * (&sol_os_installed__M__Wp_this_year
                - &sol_os_installed__M__Wp_prev_year)
               * constants::solar_landscape.invest__m__eur_per_Wp
               * sol_os_installed__M__Wp_this_year
                    .is_greater(&sol_os_installed__M__Wp_prev_year);
            self.results.sol_os_invest__M__eur_per_a
                .set_year_values(year, &sol_os_invest__M__eur_per_a);

            let sol_os_grant__M__eur_per_a = 1e-3
               * (&sol_os_installed__M__Wp_this_year
                - &sol_os_installed__M__Wp_prev_year)
               * constants::solar_landscape.grant__m__eur_per_Wp
               * sol_os_installed__M__Wp_this_year
                    .is_greater(&sol_os_installed__M__Wp_prev_year);
            self.results.sol_os_grant__M__eur_per_a
                .set_year_values(year, &sol_os_grant__M__eur_per_a);



            let sol_os_om__M__eur_per_a_last_year =
                self.results.sol_os_om__M__eur_per_a.get_year(year-1);
            let sol_os_om__M__eur_per_a = &sol_os_om__M__eur_per_a_last_year
                + &(
                    &sol_os_invest__M__eur_per_a
                    * constants::solar_landscape.operation_costs
                    );
            self.results.sol_os_om__M__eur_per_a
                .set_year_values(year, &sol_os_om__M__eur_per_a)
        }

        let sol_os_revenue__M__eur_per_a =
            (&sol_os_nrg__G__W_h_per_a
                - &sol_os_self_cnsmp__G__W_h_per_a
            )
            * constants::solar_landscape.buyback_price__m__eur_per_W_h;
        self.results.sol_os_revenue__M__eur_per_a
            .set_year_values(year, &sol_os_revenue__M__eur_per_a);


        // Purchase of renewable energy

        let prchsd_renewable_nrg__G__W_h_per_a = self.inputs.
            prchsd_renewable_nrg__G__W_h_per_a.get_year(year);
        let renewable_nrg_price__m__eur_per_W_h = self.inputs.
            renewable_nrg_price__m__eur_per_W_h.get_year(year);

        let prchsd_renewable_nrg__M__eur_per_a =
            &prchsd_renewable_nrg__G__W_h_per_a
            * &renewable_nrg_price__m__eur_per_W_h;
        self.results.prchsd_renewable_nrg__M__eur_per_a
            .set_year_values(year, &prchsd_renewable_nrg__M__eur_per_a);

    }

    pub fn calculate_second_stage(
        &mut self,
        year: u32,
        electric_power_demand_buildings: SectorsRawValues,
        energy_heating_heat_pump: SectorsRawValues,
        bev_electric_power_demand: SectorsRawValues,
        sl_nrg_dmd__G__W_h_per_a: f32,
        ){

        let mut elec_nrg_dmd__G__W_h_per_a = &electric_power_demand_buildings
            + &energy_heating_heat_pump + bev_electric_power_demand;
        elec_nrg_dmd__G__W_h_per_a.public +=
            elec_nrg_dmd__G__W_h_per_a.schools + sl_nrg_dmd__G__W_h_per_a;
        self.results.elec_nrg_dmd__G__W_h_per_a
            .set_year_values(year, &elec_nrg_dmd__G__W_h_per_a);

        let sol_rf_self_cnsmp__G__W_h_per_a =
            self.results.sol_rf_self_cnsmp__G__W_h_per_a.get_year(year);
        let sol_os_self_cnsmp__G__W_h_per_a =
            self.results.sol_os_self_cnsmp__G__W_h_per_a.get_year(year);
        let prchsd_renewable_nrg__G__W_h_per_a =
            self.inputs.prchsd_renewable_nrg__G__W_h_per_a.get_year(year);

        let prchsd_nrg_mix__G__W_h_per_a = &elec_nrg_dmd__G__W_h_per_a
            - &sol_rf_self_cnsmp__G__W_h_per_a
            - &sol_os_self_cnsmp__G__W_h_per_a
            - &prchsd_renewable_nrg__G__W_h_per_a;
        self.results.prchsd_nrg_mix__G__W_h_per_a
            .set_year_values(year, &prchsd_nrg_mix__G__W_h_per_a);

        let nrg_mix_price__m__eur_per_W_h =
            self.inputs.nrg_mix_price__m__eur_per_W_h.get_year(year);

        let prchsd_nrg_mix_costs__M__eur_per_a = &prchsd_nrg_mix__G__W_h_per_a
            * &nrg_mix_price__m__eur_per_W_h
            * prchsd_nrg_mix__G__W_h_per_a.is_greater(&SectorsRawValues::new());
        self.results.prchsd_nrg_mix_costs__M__eur_per_a
            .set_year_values(year, &prchsd_nrg_mix_costs__M__eur_per_a);

        let prchsd_renewable_nrg__M__eur_per_a = self.results
            .prchsd_renewable_nrg__M__eur_per_a.get_year(year);

        let sol_rf_om__M__eur_per_a =
            self.results.sol_rf_om__M__eur_per_a.get_year(year);
        let sol_rf_revenue__M__eur_per_a =
            self.results.sol_rf_revenue__M__eur_per_a.get_year(year);

        let total_nrg_costs__M__eur_per_a = &prchsd_renewable_nrg__M__eur_per_a
            + &prchsd_nrg_mix_costs__M__eur_per_a + sol_rf_om__M__eur_per_a
            - &sol_rf_revenue__M__eur_per_a;

        let nrg_own_mix_price__m__eur_per_W_h = &total_nrg_costs__M__eur_per_a
            / &elec_nrg_dmd__G__W_h_per_a;
        self.results.nrg_own_mix_price__m__eur_per_W_h
            .set_year_values(year, &nrg_own_mix_price__m__eur_per_W_h)

    }
    pub fn calculate_emissions(&mut self, year: u32){

        let prchsd_nrg_mix__G__W_h_per_a = self.results
            .prchsd_nrg_mix__G__W_h_per_a.get_year(year);

        let nrg_mix_ems__k__to_coe_per_a =
            constants::evu_power_mix::coal
            * constants::evu_emissions::coal
            + constants::evu_power_mix::gas
            * constants::evu_emissions::gas;

        let prchsd_nrg_mix_ems__k__to_coe_per_a = &prchsd_nrg_mix__G__W_h_per_a
            * nrg_mix_ems__k__to_coe_per_a;

        self.results.prchsd_nrg_mix_ems__k__to_coe_per_a
            .set_year_values(year, &prchsd_nrg_mix_ems__k__to_coe_per_a);
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
    rf_A__k__m2,
    sol_rf_suitable_A_part,
    sol_rf_installed__M__Wp,
    sol_rf_self_cnsmp_part,
    sol_os_installed_A__ha,
    sol_os_self_cnsmp_part,
    prchsd_renewable_nrg__G__W_h_per_a,
    renewable_nrg_price__m__eur_per_W_h,
    nrg_mix_price__m__eur_per_W_h
}


macro_rules! implement_results_energy{
    ($($field:ident),*) => {

        pub struct ResultsEnergy{
            $(
                pub $field: SectorsResult,
             )*
            // Note that is only to easily add a single value atribute again if needed
            // pub nrg_own_mix_price__m__eur_per_W_h: Results,
        }

        impl ResultsEnergy{
            fn new(id: &str, start_year: u32, end_year: u32) -> ResultsEnergy{
                return ResultsEnergy{
                    $(
                        $field: SectorsResult::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                    // nrg_own_mix_price__m__eur_per_W_h: Results::new(id.to_owned()+"/nrg_own_mix_price__m__eur_per_W_h", start_year, end_year),
                }
            }

            pub fn get_results(& self) -> Vec<&Results>{
                let mut results: Vec<&Results> = Vec::from([]);
                $(
                    results.extend(self.$field.get_results());
                 )*
                // results.push(&self.nrg_own_mix_price__m__eur_per_W_h);
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
                    // "nrg_own_mix_price__m__eur_per_W_h"=> Some(&mut self.nrg_own_mix_price__m__eur_per_W_h),
                    _ => None,

                }

            }
        }


    }
}

implement_results_energy!{
    sol_rf_potential__M__Wp,
    sol_rf_nrg__G__W_h_per_a,
    sol_rf_self_cnsmp__G__W_h_per_a,
    sol_rf_invest__M__eur_per_a,
    sol_rf_grant__M__eur_per_a,
    sol_rf_om__M__eur_per_a,
    sol_rf_revenue__M__eur_per_a,
    sol_os_installed__M__Wp,
    sol_os_nrg__G__W_h_per_a,
    sol_os_self_cnsmp__G__W_h_per_a,
    sol_os_invest__M__eur_per_a,
    sol_os_grant__M__eur_per_a,
    sol_os_om__M__eur_per_a,
    sol_os_revenue__M__eur_per_a,
    sol_os_prod_costs__M__eur_per_a,
    prchsd_renewable_nrg__M__eur_per_a,
    elec_nrg_dmd__G__W_h_per_a,
    prchsd_nrg_mix__G__W_h_per_a,
    prchsd_nrg_mix_costs__M__eur_per_a,
    prchsd_nrg_mix_ems__k__to_coe_per_a,
    nrg_own_mix_price__m__eur_per_W_h
}



#[cfg(test)]
mod tests{
    use super::*;
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    mod energy_test_case;
    use energy_test_case::create_energy;
    mod compare_with_excel;

}
