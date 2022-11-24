use crate::result::Results;
use crate::buildings::Buildings;
use crate::energy::Energy;
use crate::mobility::Mobility;

macro_rules! implement_steakholders{
    ($($field:ident),*) => {

        struct Steakholders{
            start_year: u32,
            $(
                $field: Results,
             )*
        }

        impl Steakholders{
            fn new(id: &str, start_year: u32, end_year: u32) -> Steakholders {
                return Steakholders{
                    start_year: start_year,
                    $(
                        $field: Results::new(id.to_owned()+"/"+stringify!($field), start_year, end_year),
                     )*
                }
            }

            pub fn get_results(& self) -> Vec<&Results>{
                let mut results: Vec<&Results> = Vec::from([]);
                $(
                    results.push(&self.$field);
                 )*
                return results;
            }

            fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
                let binding = id.to_string();
                let splitted_id: Vec<&str> = binding.split("/").collect();
                let binding_b = &splitted_id[1..].join("/");
                let remaining_id = binding_b.as_str();

                match splitted_id[0]{
                    $(
                        stringify!($field)=> Some(&mut self.$field),
                     )*
                    _ => None,
                }
            }
        }
    }
}

implement_steakholders!{
    private_invest,
    private_effect_of_measures,
    private_cash_flow_netto,
    industry_invest,
    industry_effect_of_measures,
    industry_profit_from_measures,
    industry_cash_flow_netto,
    community_invest,
    community_effect_of_measures,
    community_profit_from_measures,
    community_cash_flow_netto,
    federal_additional_expenses,
    federal_additional_tax_income,
    federal_cash_flow_netto
}

impl Steakholders{
    pub fn calculate(
        &mut self,
        year: u32,
        buildings: Buildings,
        energy: Energy,
        mobility: Mobility,
    ){
        let private_invest_buildings =
            &buildings.invest_heat_sources__M__eur_per_a
                .get_year(year).private
            + &buildings.invest_energetic_renovation__M__eur_per_a
                .get_year(year).private
            - &buildings.grant_heat_sources__M__eur_per_a
                .get_year(year).private;
            - &buildings.grant_energetic_renovation__M__eur_per_a
                .get_year(year).private;

        let private_invest_energy =
            &results_energy.solar_roof_invest.get_year(year).private
            + &results_energy.solar_landscape_invest.get_year(year).private
            - &results_energy.solar_roof_grant.get_year(year).private
            - results_energy.solar_landscape_grant.get_year(year).private;

        let private_invest = private_invest_buildings + private_invest_energy;
        self.private_invest.set_year_value(year, private_invest);

        if year > self.start_year{

            let costs_heating_oil_diff =
                (&results_buildings.costs_heating_oil.get_year(year)
                - &results_buildings.costs_heating_oil.get_year(year-1)
                ).private;
            let costs_heating_gas_diff =
                (&results_buildings.costs_heating_gas.get_year(year)
                - &results_buildings.costs_heating_gas.get_year(year-1)
                ).private;
            let bev_electric_power_costs_diff =
                (&results_mobility.bev_electric_power_costs.get_year(year)
                - &results_mobility.bev_electric_power_costs.get_year(year-1)
                ).private;
            let car_fuel_costs_diff =
                (&results_mobility.car_fuel_costs.get_year(year)
                - &results_mobility.car_fuel_costs.get_year(year-1)
                ).private;
            let solar_roof_buyback_diff =
                (&results_energy.solar_roof_buyback.get_year(year)
                - &results_energy.solar_roof_buyback.get_year(year-1)
                ).private;
            let direct_aquisition_renable_energies_costs_diff =
                (&results_energy
                    .direct_aquisition_renable_energies_costs.get_year(year)
                - &results_energy
                    .direct_aquisition_renable_energies_costs.get_year(year-1)
                ).private;
            let purchased_energy_mix_costs_diff =
                (&results_energy.purchased_energy_mix_costs.get_year(year)
                - &results_energy.purchased_energy_mix_costs.get_year(year-1)
                ).private;

            let private_effect_of_measures = costs_heating_oil_diff
                + costs_heating_gas_diff + bev_electric_power_costs_diff
                + car_fuel_costs_diff - solar_roof_buyback_diff
                + direct_aquisition_renable_energies_costs_diff
                + purchased_energy_mix_costs_diff;
            self.private_effect_of_measures
                .set_year_value(year, private_effect_of_measures);

            let private_cash_flow_netto = private_invest
                + private_effect_of_measures;
            self.private_cash_flow_netto
                .set_year_value(year, private_cash_flow_netto);

        }
    }
}
