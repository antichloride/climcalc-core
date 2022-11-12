use crate::result::Results;
use crate::buildings::ResultsBuildings;
use crate::energy::ResultsEnergy;
use crate::mobility::ResultsMobility;
use crate::constants::economy as constants;
use crate::constants::energy as constants_energy;

macro_rules! implement_economy{
    ($($field:ident),*) => {

        struct Economy{
            start_year: u32,
            $(
                $field: Results,
             )*
        }

        impl Economy{
            fn new(id: &str, start_year: u32, end_year: u32) -> Economy {
                return Economy{
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

implement_economy!{
    invest_heating_material,
    invest_heating_work,
    turnover_heating_crafting_local,
    turnover_heating_crafting_national,
    turnover_heating_production_national,
    invest_heat_demand_material,
    invest_heat_demand_work,
    turnover_heat_demand_crafting_local,
    turnover_heat_demand_crafting_national,
    turnover_heat_demand_production_national,
    invest_solar_roof_material,
    invest_solar_roof_work,
    maintenance_solar_roof_work,
    turnover_solar_roof_crafting_local,
    turnover_solar_roof_crafting_national,
    turnover_solar_roof_production_national,
    invest_solar_landscape_material,
    invest_solar_landscape_work,
    maintenance_solar_landscape_work,
    turnover_solar_landscape_crafting_local,
    turnover_solar_landscape_crafting_national,
    turnover_solar_landscape_production_national,
    jobs_crafting_local,
    jobs_crafting_national,
    jobs_production_national,
    income_local,
    income_national,
    income_tax_local,
    income_tax_national,
    turnover_local_with_local_part_of_material,
    turnover_national,
    turnover_tax_local,
    turnover_tax_national,
    business_tax_local,
    business_tax_national,
    corporate_tax_national
}


impl Economy{
    pub fn calculate(
        &mut self,
        year: u32,
        results_buildings: ResultsBuildings,
        results_energy: ResultsEnergy,
    ){

        // heating
        let buildings_invest_heat = results_buildings.invest_heat
            .get_year(year).sum();

        let invest_heating_material = buildings_invest_heat *
            constants::material::invest_local::heating_heatpump;
        self.invest_heating_material
            .set_year_value(year, invest_heating_material);

        let invest_heating_work = buildings_invest_heat *
            (1.0 - constants::material::invest_local::heating_heatpump);
        self.invest_heating_work
            .set_year_value(year, invest_heating_work);

        let turnover_heating_crafting_local = invest_heating_work *
            constants::invest_work::community_added_value::heating_heatpump;
        self.turnover_heating_crafting_local
            .set_year_value(year, turnover_heating_crafting_local);

        let turnover_heating_crafting_national = invest_heating_work *
            constants::invest_work::national_added_value::heating_heatpump;
        self.turnover_heating_crafting_national
            .set_year_value(year, turnover_heating_crafting_national);

        let turnover_heating_production_national = invest_heating_material *
            constants::material::invest_national::heating_heatpump;
        self.turnover_heating_production_national
            .set_year_value(year, turnover_heating_production_national);


        // thermal heat demand
        let invest_thermal_energy_demand = results_buildings
            .invest_thermal_energy_demand.get_year(year).sum();

        let invest_heat_demand_material = invest_thermal_energy_demand *
            constants::material::invest_local::energetic_restoration;
        self.invest_heat_demand_material
            .set_year_value(year, invest_heat_demand_material);

        let invest_heat_demand_work = invest_thermal_energy_demand *
            (1.0 - constants::material::invest_local::energetic_restoration);
        self.invest_heat_demand_work
            .set_year_value(year, invest_heat_demand_work);

        let turnover_heat_demand_crafting_local = invest_heat_demand_work *
            constants::invest_work::community_added_value::energetic_restoration;
        self.turnover_heat_demand_crafting_local
            .set_year_value(year, turnover_heat_demand_crafting_local);

        let turnover_heat_demand_crafting_national = invest_heat_demand_work *
            constants::invest_work::national_added_value::energetic_restoration;
        self.turnover_heat_demand_crafting_national
            .set_year_value(year, turnover_heat_demand_crafting_national);

        let turnover_heat_demand_production_national = invest_heat_demand_material *
            constants::material::invest_national::energetic_restoration;
        self.turnover_heat_demand_production_national
            .set_year_value(year, turnover_heat_demand_production_national);


        // solar roof
        let invest_solar_roof = results_energy
            .solar_roof_invest.get_year(year).sum();

        let invest_solar_roof_material = invest_solar_roof *
            constants::material::invest_local::solar_roof;
        self.invest_solar_roof_material
            .set_year_value(year, invest_solar_roof_material);

        let invest_solar_roof_work = invest_solar_roof *
            (1.0 - constants::material::invest_local::solar_roof);
        self.invest_solar_roof_work
            .set_year_value(year, invest_solar_roof_work);

        let maintenance_solar_roof_work = results_energy.solar_roof_invest
            .sum_years(self.start_year, year).sum()
            * constants_energy::solar_roof.operation_costs;

        let turnover_solar_roof_crafting_local = invest_solar_roof_work *
            constants::invest_work::community_added_value::solar_roof;
        self.turnover_solar_roof_crafting_local
            .set_year_value(year, turnover_solar_roof_crafting_local);

        let turnover_solar_roof_crafting_national = invest_solar_roof_work *
            constants::invest_work::national_added_value::solar_roof;
        self.turnover_solar_roof_crafting_national
            .set_year_value(year, turnover_solar_roof_crafting_national);

        let turnover_solar_roof_production_national = invest_solar_roof_material *
            constants::material::invest_national::solar_roof;
        self.turnover_solar_roof_production_national
            .set_year_value(year, turnover_solar_roof_production_national);



        // solar landscape
        let invest_solar_landscape = results_energy
            .solar_landscape_invest.get_year(year).sum();

        let invest_solar_landscape_material = invest_solar_landscape *
            constants::material::invest_local::solar_landscape;
        self.invest_solar_landscape_material
            .set_year_value(year, invest_solar_landscape_material);

        let invest_solar_landscape_work = invest_solar_landscape *
            (1.0 - constants::material::invest_local::solar_landscape);
        self.invest_solar_landscape_work
            .set_year_value(year, invest_solar_landscape_work);

        let maintenance_solar_landscape_work = results_energy.solar_landscape_invest
            .sum_years(self.start_year, year).sum()
            * constants_energy::solar_landscape.operation_costs;

        let turnover_solar_landscape_crafting_local = invest_solar_landscape_work *
            constants::invest_work::community_added_value::solar_landscape;
        self.turnover_solar_landscape_crafting_local
            .set_year_value(year, turnover_solar_landscape_crafting_local);

        let turnover_solar_landscape_crafting_national = invest_solar_landscape_work *
            constants::invest_work::national_added_value::solar_landscape;
        self.turnover_solar_landscape_crafting_national
            .set_year_value(year, turnover_solar_landscape_crafting_national);

        let turnover_solar_landscape_production_national = invest_solar_landscape_material *
            constants::material::invest_national::solar_landscape;
        self.turnover_solar_landscape_production_national
            .set_year_value(year, turnover_solar_landscape_production_national);


        // jobs
        let jobs_crafting_local =
            &turnover_heating_crafting_local
            / constants::vzw_per_million_turnover::work::heating_heatpump
            + &turnover_heat_demand_crafting_local
            / constants::vzw_per_million_turnover::work::energetic_restoration
            + &turnover_solar_roof_crafting_local
            / constants::vzw_per_million_turnover::work::solar_roof
            + &turnover_solar_landscape_crafting_local
            / constants::vzw_per_million_turnover::work::solar_landscape;
        self.jobs_crafting_local.set_year_value(year, jobs_crafting_local);

        let jobs_crafting_national =
            &turnover_heating_crafting_national
            / constants::vzw_per_million_turnover::work::heating_heatpump
            + &turnover_heat_demand_crafting_national
            / constants::vzw_per_million_turnover::work::energetic_restoration
            + &turnover_solar_roof_crafting_national
            / constants::vzw_per_million_turnover::work::solar_roof
            + &turnover_solar_landscape_crafting_national
            / constants::vzw_per_million_turnover::work::solar_landscape;
        self.jobs_crafting_national
            .set_year_value(year, jobs_crafting_national);

        let jobs_production_national =
            &turnover_heating_production_national
            / constants::vzw_per_million_turnover::material::heating_heatpump
            + &turnover_heat_demand_production_national
            / constants::vzw_per_million_turnover::material::energetic_restoration
            + &turnover_solar_roof_production_national
            / constants::vzw_per_million_turnover::material::solar_roof
            + &turnover_solar_landscape_production_national
            / constants::vzw_per_million_turnover::material::solar_landscape;
        self.jobs_production_national
            .set_year_value(year, jobs_production_national);


        // taxes
        let income_local = jobs_crafting_local
            * constants::tax::income_per_fte;
        self.income_local.set_year_value(year, income_local);

        let income_national = jobs_crafting_national + jobs_production_national
            * constants::tax::income_per_fte;
        self.income_national.set_year_value(year, income_national);

        let income_tax_local = income_local
            * constants::tax::income_tax_local_per_fte;
        self.income_tax_local.set_year_value(year, income_tax_local);

        let income_tax_national = income_national
            * constants::tax::income_tax_per_fte;
        self.income_tax_national.set_year_value(year, income_tax_national);

        let turnover_local_with_local_part_of_material =
            turnover_heating_crafting_local
            + invest_heating_material
            * constants::invest_work::community_added_value::heating_heatpump
            + turnover_heat_demand_crafting_local
            + invest_heat_demand_material
            * constants::invest_work::community_added_value::energetic_restoration
            + turnover_solar_roof_crafting_local
            + invest_solar_roof_material
            * constants::invest_work::community_added_value::solar_roof
            + turnover_solar_landscape_crafting_local
            + invest_solar_landscape_material
            * constants::invest_work::community_added_value::solar_landscape;
        self.turnover_local_with_local_part_of_material
            .set_year_value(year, turnover_local_with_local_part_of_material);

        let turnover_national = buildings_invest_heat
            + invest_thermal_energy_demand
            + invest_solar_roof + invest_solar_landscape;
        self.turnover_national.set_year_value(year, turnover_national);

        let turnover_tax_local = turnover_local_with_local_part_of_material
            * constants::tax::turnover_tax_local_part
            * constants::tax::turnover_tax;
        self.turnover_tax_local.set_year_value(year, turnover_tax_local);

        let turnover_tax_national = turnover_national
            * constants::tax::turnover_tax;
        self.turnover_tax_national.set_year_value(year, turnover_tax_national);

        let business_tax_local = turnover_local_with_local_part_of_material
            * constants::tax::business_tax;
        self.business_tax_local.set_year_value(year, business_tax_local);

        let business_tax_national = turnover_national
            * constants::tax::business_tax;
        self.business_tax_national.set_year_value(year, business_tax_national);

        let corporate_tax_national = turnover_national
            * constants::tax::corporate_tax;
        self.corporate_tax_national.set_year_value(year, corporate_tax_national);

    }
}
