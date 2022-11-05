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

        // Solar Roof
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

        let solar_roof_costs = &solar_roof_self_consumption
            * constants::solar_roof.costs - &solar_roof_buyback;
        self.results.solar_roof_costs.set_year_values(year, &solar_roof_costs);

        let solar_roof_costs_per_kwh = &solar_roof_costs
            / &solar_roof_installed_power;
        self.results.solar_roof_costs_per_kwh
            .set_year_values(year, &solar_roof_costs_per_kwh);


        // Solar Landscape

        let solar_landscape_suitable_area = self.inputs
            .solar_landscape_suitable_area.get_year(year);

        let solar_landscape_installed_power_peak = 1e-1
            * &solar_landscape_suitable_area
            * constants::solar_landscape.power_per_area;
        self.results.solar_landscape_installed_power_peak
            .set_year_values(year, &solar_landscape_installed_power_peak);

        let solar_landscape_installed_power = 1e-3
            * &solar_landscape_installed_power_peak
            * constants::solar_landscape.power_per_area;
        self.results.solar_landscape_installed_power
            .set_year_values(year, &solar_landscape_installed_power);

        if year != self.start_year{

            let solar_landscape_installed_power_peak_this_year =
                self.results.solar_landscape_installed_power_peak.get_year(year);
            let solar_landscape_installed_power_peak_prev_year =
                self.results.solar_landscape_installed_power_peak
                .get_year(year - 1);

            let solar_landscape_invest = 1e-3
               * (&solar_landscape_installed_power_peak_this_year
                - &solar_landscape_installed_power_peak_prev_year)
               * constants::solar_landscape.invest
               * solar_landscape_installed_power_peak_this_year
                    .is_greater(&solar_landscape_installed_power_peak_prev_year);
            self.results.solar_landscape_invest
                .set_year_values(year, &solar_landscape_invest);

            let solar_landscape_grant = 1e-3
               * (&solar_landscape_installed_power_peak_this_year
                - &solar_landscape_installed_power_peak_prev_year)
               * constants::solar_landscape.grant
               * solar_landscape_installed_power_peak_this_year
                    .is_greater(&solar_landscape_installed_power_peak_prev_year);
            self.results.solar_landscape_grant
                .set_year_values(year, &solar_landscape_grant);


            let mut solar_landscape_operation_costs = SectorsRawValues::new();

            for year_i in self.start_year..year{
                solar_landscape_operation_costs =
                    solar_landscape_operation_costs
                    + self.results.solar_landscape_operation_costs
                    .get_year(year_i);
            }

            let solar_landscape_operation_costs =
                solar_landscape_operation_costs
                + &solar_landscape_invest
                * constants::solar_landscape.grant;
            self.results.solar_landscape_operation_costs
                .set_year_values(year, &solar_landscape_operation_costs);
        }

        let solar_landscape_production_costs =
            &solar_landscape_installed_power * constants::solar_landscape.costs;
        self.results.solar_landscape_production_costs
            .set_year_values(year, &solar_landscape_production_costs);

        let solar_landscape_buyback = &solar_landscape_installed_power
            * constants::solar_landscape.buyback_price;
        self.results.solar_landscape_buyback
            .set_year_values(year, &solar_landscape_buyback);

        let solar_landscape_profit = &solar_landscape_production_costs
            - &solar_landscape_buyback;
        self.results.solar_landscape_profit
            .set_year_values(year, &solar_landscape_profit);


        // Purchase of renewable energy

        let direct_aquisition_reneable_enrgies = self.inputs.
            direct_aquisition_reneable_enrgies.get_year(year);
        let direct_aquisition_reneable_enrgies_price = self.inputs.
            direct_aquisition_reneable_enrgies_price.get_year(year);

        let direct_aquisition_renable_energies_costs =
            &direct_aquisition_reneable_enrgies
            * &direct_aquisition_reneable_enrgies_price;
        self.results.direct_aquisition_renable_energies_costs
            .set_year_values(year, &direct_aquisition_renable_energies_costs);


        // power injection
        let solar_roof_electric_power_injection =
            &solar_roof_installed_power * &(1.0-&solar_roof_self_consumption);
        self.results.solar_roof_electric_power_injection
            .set_year_values(year, &solar_roof_electric_power_injection);

    }

    pub fn calculate_second_stage(
        &mut self,
        year: u32,
        electric_power_demand_buildings: SectorsRawValues,
        energy_heating_heat_pump: SectorsRawValues,
        bev_electric_power_demand: SectorsRawValues,
        ){


        let mut electric_energy_demand = &electric_power_demand_buildings
            + &energy_heating_heat_pump + bev_electric_power_demand;
        electric_energy_demand.public += electric_energy_demand.private;
        self.results.electric_energy_demand
            .set_year_values(year, &electric_energy_demand);

        let solar_roof_self_consumption = self.results
            .solar_roof_self_consumption.get_year(year);
        let direct_aquisition_reneable_enrgies = self.inputs
            .direct_aquisition_reneable_enrgies.get_year(year);

        let purchased_energy_mix = &electric_energy_demand
            - &solar_roof_self_consumption
            - &direct_aquisition_reneable_enrgies;
        self.results.purchased_energy_mix
            .set_year_values(year, &purchased_energy_mix);

        let aquisition_power_mix = self.inputs.aquisition_power_mix
            .get_year(year);

        let purchased_energy_mix_costs = &purchased_energy_mix
            * &aquisition_power_mix
            * purchased_energy_mix.is_greater(&SectorsRawValues::new());
        self.results.purchased_energy_mix_costs
            .set_year_values(year, &purchased_energy_mix_costs);

        let purchased_energy_mix_emissions =
            constants::evu_power_mix::coal
            * constants::evu_emissions::coal
            + constants::evu_power_mix::gas
            * constants::evu_emissions::gas;

        let solar_roof_costs = self.results.solar_roof_costs.get_year(year);
        let direct_aquisition_renable_energies_costs = self.results
            .direct_aquisition_renable_energies_costs.get_year(year);
        let direct_aquisition_reneable_enrgies = self.inputs
            .direct_aquisition_reneable_enrgies.get_year(year);

        let aquisition_power_mix_price = (solar_roof_costs.private
            + direct_aquisition_renable_energies_costs.private
            + purchased_energy_mix_costs.private)
            / (solar_roof_self_consumption.private
            + direct_aquisition_reneable_enrgies.private
            + purchased_energy_mix.private);

    }
    pub fn calculate_emissions(&mut self, year: u32){

        let purchased_energy_mix = self.results
            .purchased_energy_mix.get_year(year);
        let aquisition_power_mix_emissions = self.inputs
            .aquisition_power_mix_emissions.get_year(year);

        let emissions_purchased_energy_mix = &purchased_energy_mix
            * aquisition_power_mix_emissions * 1e-3;
        self.results.emissions_purchased_energy_mix
            .set_year_values(year, &emissions_purchased_energy_mix);
    }
}



macro_rules! implement_inputs_energy{
    ($($field:ident),*) => {

        pub struct InputsEnergy{
            $(
                $field: SectorsInputs,
             )*
            pub aquisition_power_mix_emissions: Input,
        }

        impl InputsEnergy{
            fn new(id: &str, start_year: u32, end_year: u32) -> InputsEnergy {
                return InputsEnergy{
                    $(
                        $field: SectorsInputs::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                    aquisition_power_mix_emissions: Input::new(id.to_owned()+"/aquisition_power_mix_emissions", start_year, end_year),
                }
            }
        }

        impl InputFields for InputsEnergy{

            fn get_inputs(& self) -> Vec<&Input>{
                let mut inputs: Vec<&Input> = Vec::from([]);
                $(
                    inputs.extend(self.$field.get_inputs());
                 )*
                inputs.push(&self.aquisition_power_mix_emissions);
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
                    "aquisition_power_mix_emissions"=> Some(&mut self.aquisition_power_mix_emissions),
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
    solar_roof_portion_self_consumption,
    solar_landscape_suitable_area,
    direct_aquisition_reneable_enrgies,
    direct_aquisition_reneable_enrgies_price,
    aquisition_power_mix
}


macro_rules! implement_results_energy{
    ($($field:ident),*) => {

        pub struct ResultsEnergy{
            $(
                pub $field: SectorsResult,
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
    solar_landscape_installed_power,
    solar_landscape_invest,
    solar_landscape_grant,
    solar_landscape_operation_costs,
    solar_landscape_production_costs,
    solar_landscape_buyback,
    solar_landscape_profit,
    direct_aquisition_renable_energies_costs,
    electric_energy_demand,
    purchased_energy_mix,
    purchased_energy_mix_costs,
    purchased_energy_mix_emissions,
    emissions_purchased_energy_mix,
    solar_roof_electric_power_injection
}
