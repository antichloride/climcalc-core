#![allow(non_snake_case)]

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
}

impl Buildings{

    pub fn new(start_year: u32, end_year: u32) -> Buildings{
        return Buildings{
            inputs: InputsBuildings
                ::new("buildings/inputs", start_year, end_year),
            results: ResultsBuildings
                ::new("buildings/results", start_year, end_year),
            start_year: start_year,
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
    pub fn elec_dmd__G__W_h_per_a(&self) -> &SectorsResult{
        return &self.results.elec_dmd__G__W_h_per_a;
    }
    pub fn cnsmp_elec_heat_pump__G__W_h_per_a(&self) -> &SectorsResult{
        return &self.results.cnsmp_elec_heat_pump__G__W_h_per_a;
    }
    pub fn invest_heat_sources__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.invest_heat_sources__M__eur_per_a;
    }
    pub fn invest_energetic_renovation__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.invest_energetic_renovation__M__eur_per_a;
    }
    pub fn grant_heat_sources__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.grant_heat_sources__M__eur_per_a;
    }
    pub fn grant_energetic_renovation__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.grant_energetic_renovation__M__eur_per_a;
    }
    pub fn costs_oil__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.costs_oil__M__eur_per_a;
    }
    pub fn costs_gas__M__eur_per_a(&self) -> &SectorsResult{
        return &self.results.costs_gas__M__eur_per_a;
    }
}



impl Buildings{

    pub fn calculate(&mut self, year: u32){
        let inputs = &self.inputs;
        let results = &mut self.results;

        let floor_A_building__m2 =
            self.inputs.floor_A_building__m2.get_year(year);
        let n_buildings = self.inputs.n_buildings.get_year(year);
        let n_inhabitants__k__ = self.inputs.n_inhabitants__k__.get_year(year);
        let heat_dmd__k__W_h_per_m2_a =
            self.inputs.heat_dmd__k__W_h_per_m2_a.get_year(year);
        let hot_water_dmd__k__W_h_per_m2_a =
            self.inputs.hot_water_dmd__k__W_h_per_m2_a.get_year(year);
        let elec_dmd_capita__k_W_h_per_a =
            self.inputs.elec_dmd_capita__k_W_h_per_a.get_year(year);

        let floor_A__k__m2 = &floor_A_building__m2 * &n_buildings * 1e-3;
        results.floor_A__k__m2.set_year_values(year, &floor_A__k__m2);

        let total_heat_dmd__G__W_h_per_a =
            (&n_inhabitants__k__ * &hot_water_dmd__k__W_h_per_m2_a
            + &heat_dmd__k__W_h_per_m2_a * &floor_A__k__m2) * 1e-3;
        results.total_heat_dmd__G__W_h_per_a
            .set_year_values(year, &total_heat_dmd__G__W_h_per_a);

        let elec_dmd__G__W_h_per_a = &elec_dmd_capita__k_W_h_per_a
            * &n_inhabitants__k__ * 1e-3;
        results.elec_dmd__G__W_h_per_a
            .set_year_values(year, &elec_dmd__G__W_h_per_a);


        // Energy Consumption of different heating types
        let total_heat_dmd__M__W_h_per_m2_a =
            &total_heat_dmd__G__W_h_per_a / &floor_A__k__m2;

        macro_rules! implement_heat_types_input_consumption{
                ($((
                    $heat_type: ident,
                    $heat_type_A: ident,
                    $heat_type_cnsmp_in: ident
                )),*) => {
                    $(
                        let $heat_type_A =
                            self.inputs.$heat_type_A.get_year(year);
                        let $heat_type_cnsmp_in = &$heat_type_A
                            * &total_heat_dmd__M__W_h_per_m2_a
                            / constants::$heat_type.efficency;
                        results.$heat_type_cnsmp_in
                            .set_year_values(year,&$heat_type_cnsmp_in);
                     )*
            }
        }

        implement_heat_types_input_consumption!{
            (
                oil_no_condensing,
                A_heat_oil__k__m2,
                cnsmp_oil__G__W_h_per_a
            ),
            (
                oil_with_condensing,
                A_heat_oil_condensing__k__m2,
                cnsmp_oil_condensing__G__W_h_per_a
            ),
            (
                gas,
                A_heat_gas__k__m2,
                cnsmp_gas__G__W_h_per_a
            ),
            (
                heat_pump,
                A_heat_heat_pump__k__m2,
                cnsmp_elec_heat_pump__G__W_h_per_a
            ),
            (
                other,
                A_heat_other__k__m2,
                cnsmp_other__G__W_h_per_a
            )
        }

        let cnsmp_oil__M__L_per_a =
            (&cnsmp_oil__G__W_h_per_a + &cnsmp_oil_condensing__G__W_h_per_a)
            / constants::EnergySource::oil::energy_density__k__W_h_per_L;
        results.cnsmp_oil__M__L_per_a
            .set_year_values(year, &cnsmp_oil__M__L_per_a);

        let cnsmp_gas__M__m3_per_a = &cnsmp_gas__G__W_h_per_a
            / constants::EnergySource::gas::energy_density__k__W_h_per_m3;
        results.cnsmp_gas__M__m3_per_a
            .set_year_values(year, &cnsmp_gas__M__m3_per_a);


        // Costs
        let costs_oil__M__eur_per_a = &cnsmp_oil__M__L_per_a
            * constants::EnergySource::oil::price__eur_per_L;
        results.costs_oil__M__eur_per_a
            .set_year_values(year, &costs_oil__M__eur_per_a);

        let costs_gas__M__eur_per_a = &cnsmp_gas__M__m3_per_a
            * constants::EnergySource::gas::price__eur_per_m3;
        results.costs_gas__M__eur_per_a
            .set_year_values(year, &costs_gas__M__eur_per_a);


        // Invests and Grants
        // invest and grants are computed by the differences of certain kpis
        // from one year to another. Therefore, they are not computed for the
        // start year.
        if year != self.start_year{

            // invest/grant heat sources
            // this is calculated by the change of the area of the corresponding
            // heat types from the year before to the current one.
            // This calculaiton uses the total heat demand (heat and hot water),
            // because it is assumed that the heater also produces the hot
            // water.
            let mut invest_heat_sources__M__eur_per_a = SectorsRawValues::new();
            let mut grant_heat_sources__M__eur_per_a = SectorsRawValues::new();

            macro_rules! implement_invest_calculation_heating{
                ($(($heat_type: ident, $heat_type_A__k__m2: ident)),*) => {

                    let mut A_this_year__k__m2: SectorsRawValues;
                    let mut A_prev_year__k__m2: SectorsRawValues;

                    let mut invest_heat_source__M__eur_per_a: SectorsRawValues;
                    let mut grant_heat_source__M__eur_per_a: SectorsRawValues;

                    $(

                        A_this_year__k__m2 =
                            self.inputs.$heat_type_A__k__m2.get_year(year);
                        A_prev_year__k__m2 =
                            self.inputs.$heat_type_A__k__m2.get_year(year-1);

                        invest_heat_source__M__eur_per_a =
                            (&A_this_year__k__m2 - &A_prev_year__k__m2)
                            * constants::$heat_type.invest__m__eur_per_W_h
                            * &total_heat_dmd__M__W_h_per_m2_a
                            * A_this_year__k__m2
                                .is_greater(&A_prev_year__k__m2);

                        grant_heat_source__M__eur_per_a =
                            &invest_heat_source__M__eur_per_a
                            * constants::$heat_type.grant;

                        invest_heat_sources__M__eur_per_a =
                            invest_heat_sources__M__eur_per_a +
                            invest_heat_source__M__eur_per_a;

                        grant_heat_sources__M__eur_per_a =
                            grant_heat_sources__M__eur_per_a +
                            grant_heat_source__M__eur_per_a;


                     )*

                }
            }

            implement_invest_calculation_heating!{
                (oil_no_condensing, A_heat_oil__k__m2),
                (oil_with_condensing, A_heat_oil_condensing__k__m2),
                (gas, A_heat_gas__k__m2),
                (heat_pump, A_heat_heat_pump__k__m2),
                (other, A_heat_other__k__m2)
            }

            results.invest_heat_sources__M__eur_per_a
                .set_year_values(year, &invest_heat_sources__M__eur_per_a);
            results.grant_heat_sources__M__eur_per_a
                .set_year_values(year, &grant_heat_sources__M__eur_per_a);

            // invest/grant energetic renovation
            // This calculation looks for the change of heat demand from one
            // year to the next.
            let heat_dmd_this_year__k__W_h_per_m2_a =
                inputs.heat_dmd__k__W_h_per_m2_a.get_year(year);
            let heat_dmd_prev_year__k__W_h_per_m2_a =
                inputs.heat_dmd__k__W_h_per_m2_a.get_year(year - 1);

            let invest_energetic_renovation__k__eur_per_a =
                (&heat_dmd_this_year__k__W_h_per_m2_a
                 - &heat_dmd_prev_year__k__W_h_per_m2_a)
                * constants::energetic_restoration::invest__m__eur_per_W_h
                * &floor_A__k__m2
                * heat_dmd_this_year__k__W_h_per_m2_a
                    .is_greater(&heat_dmd_prev_year__k__W_h_per_m2_a);
            results.invest_energetic_renovation__M__eur_per_a
                .set_year_values(
                    year,
                    &invest_energetic_renovation__k__eur_per_a
                );

            let grant_energetic_renovation__M__eur_per_a =
                &invest_energetic_renovation__k__eur_per_a
                * constants::energetic_restoration::grant;

            results.grant_energetic_renovation__M__eur_per_a
                .set_year_values(
                    year,
                    &grant_energetic_renovation__M__eur_per_a
                );

        }
    }

    pub fn calculate_second_stage(
        &mut self,
        year: u32,
        nrg_own_mix_price__m__eur_per_W_h: &SectorsResult,
        ){

        let cnsmp_elec_heat_pump__G__W_h_per_a =
            self.results.cnsmp_elec_heat_pump__G__W_h_per_a.get_year(year);
        let nrg_own_mix_price__m__eur_per_W_h =
            nrg_own_mix_price__m__eur_per_W_h.get_year(year);

        let costs_heat_pump__M__eur = &cnsmp_elec_heat_pump__G__W_h_per_a
            * &nrg_own_mix_price__m__eur_per_W_h * (1.0 - evu_discount_heat_pump);
        self.results.costs_heat_pump__M__eur
            .set_year_values(year, &costs_heat_pump__M__eur);
    }

    pub fn calculate_emissions(&mut self, year: u32){
        let cnsmp_oil__M__L_per_a = self.results.cnsmp_oil__M__L_per_a
            .get_year(year);
        let cnsmp_gas__M__m3_per_a = self.results.cnsmp_gas__M__m3_per_a
            .get_year(year);

        let ems_oil__k__to_coe_per_a = cnsmp_oil__M__L_per_a
            * constants::EnergySource::oil::emission__kg_coe_per_L;
        self.results.ems_oil__k__to_coe_per_a
            .set_year_values(year, &ems_oil__k__to_coe_per_a);

        let ems_gas__k__to_coe_per_a = cnsmp_gas__M__m3_per_a
            * constants::EnergySource::gas::emission__kg_coe_per_m3;
        self.results.ems_gas__k__to_coe_per_a
            .set_year_values(year, &ems_gas__k__to_coe_per_a);

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
            fn new(
                id: &str,
                start_year: u32,
                end_year: u32
            ) -> InputsBuildings {
                return InputsBuildings{
                    $(
                        $field: SectorsInputs::new(
                            id.to_owned()+"/"+stringify!($field),
                            start_year,
                            end_year
                        ),
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
                        stringify!($field)
                            => self.$field.get_input_by_id(remaining_id),
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
    heat_dmd__k__W_h_per_m2_a,
    n_inhabitants__k__,
    hot_water_dmd__k__W_h_per_m2_a,
    elec_dmd_capita__k_W_h_per_a,
    A_heat_oil__k__m2,
    A_heat_oil_condensing__k__m2,
    A_heat_gas__k__m2,
    A_heat_heat_pump__k__m2,
    A_heat_other__k__m2
}

macro_rules! implement_results_builidngs{
    ($($field:ident),*) => {

        pub struct ResultsBuildings{
            $(
                pub $field: SectorsResult,
             )*
        }

        impl ResultsBuildings{
            fn new(
                id: &str,
                start_year: u32,
                end_year: u32
            ) -> ResultsBuildings{
                return ResultsBuildings{
                    $(
                        $field: SectorsResult::new(
                            id.to_owned()+"/"+stringify!($field),
                            start_year,
                            end_year
                        ),
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
                        stringify!($field)
                            => self.$field.get_results_by_id(remaining_id),
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
    cnsmp_oil__M__L_per_a,
    cnsmp_gas__M__m3_per_a,
    costs_oil__M__eur_per_a,
    costs_gas__M__eur_per_a,
    invest_heat_sources__M__eur_per_a,
    invest_energetic_renovation__M__eur_per_a,
    grant_heat_sources__M__eur_per_a,
    grant_energetic_renovation__M__eur_per_a,
    costs_heat_pump__M__eur,
    ems_oil__k__to_coe_per_a,
    ems_gas__k__to_coe_per_a
}





#[cfg(test)]
mod tests{
    use super::*;
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    mod buildings_test_case;
    use buildings_test_case::create_buildings;


    fn assert(a: [f32; 4], b: [f32; 4]){
        assert_relative_eq!(a[0], b[0]);
        assert_relative_eq!(a[1], b[1]);
        assert_relative_eq!(a[2], b[2]);
        assert_relative_eq!(a[3], b[3]);
    }

    #[test]
    fn test_buildings_calculate() {
        let start_year: u32 = 2022 as u32;
        let end_year: u32 = 2025 as u32;
        let mut buildings = create_buildings(start_year, end_year);

        buildings.calculate(start_year);

        assert(
            buildings.results.floor_A__k__m2.get_year_values(start_year),
            [3816111.0, 1463772.0, 104125.82, 43205.76],
        );
    }
}



