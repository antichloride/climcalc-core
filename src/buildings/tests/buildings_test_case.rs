use crate::buildings::Buildings;
use crate::sectors::SectorsInputs;

impl SectorsInputs{
    fn set(&mut self, private: f32, industry:f32, schools: f32, public: f32){
        self.private.set_values(private);
        self.industry.set_values(industry);
        self.schools.set_values(schools);
        self.public.set_values(public);
    }
}

pub fn create_buildings(start_year: u32, end_year: u32) -> Buildings{

    let mut buildings = Buildings::new(start_year, end_year);

    //
    // Define Inputs
    //

    buildings.inputs.n_inhabitants__k__.set(44000.0, 5180.149927884616, 147.35656250000002, 334.386045673077);
    buildings.inputs.n_buildings.set(197.68294830471567, 1.9225230799595314, 4.807188524606554, 0.8788371689773329);
    buildings.inputs.floor_A_building__m2.set(8698.049725407489, 9.958957794008876, 0.7083707762754685, 0.2938708857248521);

    buildings.inputs.heat_dmd__k__W_h_per_m2_a.set(1237.9807692307693, 10.0, 10.0, 10.0);
    buildings.inputs.hot_water_dmd__k__W_h_per_m2_a.set(1264.009197074695, 1.665001014700444, 0.31541496898204263, 0.055287150055473376);
    buildings.inputs.elec_dmd_capita__k_W_h_per_a.set(271.94638221153843, 5.835265625, 1.3999821508413461, 0.20296802884615386);

    buildings.inputs.A_heat_oil__k__m2.set(1295.8923033707865, 366.51404494382024, 26.069779981285112, 10.815168539325846);
    buildings.inputs.A_heat_oil_condensing__k__m2.set(4918.884014423077, 2000.0961538461538, 142.26485285216347, 59.01923076923079);
    buildings.inputs.A_heat_gas__k__m2.set(278.2399038461538, 0.0, 0.0, 0.0);
    buildings.inputs.A_heat_heat_pump__k__m2.set(1401.136658653846, 818.221153846154, 58.199257984975965, 24.144230769230777);
    buildings.inputs.A_heat_other__k__m2.set(1253.2091346153845, 50.0, 50.0, 50.0);

    //
    // Add measures
    //

    buildings.inputs.heat_dmd__k__W_h_per_m2_a.private.add_measure("heat_dmd_private", 2024, 2025, 20.0);
    buildings.inputs.heat_dmd__k__W_h_per_m2_a.industry.add_measure("heat_dmd_industry", 2024, 2025, 20.0);
    buildings.inputs.heat_dmd__k__W_h_per_m2_a.public.add_measure("heat_dmd_public", 2024, 2025, 20.0);
    buildings.inputs.heat_dmd__k__W_h_per_m2_a.schools.add_measure("heat_dmd_schools", 2024, 2025, 20.0);
    buildings.inputs.heat_dmd__k__W_h_per_m2_a.apply_measures();

    buildings.inputs.A_heat_oil__k__m2.private.add_measure("a_heat_oil_private", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil__k__m2.industry.add_measure("a_heat_oil_industry", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil__k__m2.public.add_measure("a_heat_oil_public", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil__k__m2.schools.add_measure("a_heat_oil_schools", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil__k__m2.apply_measures();

    buildings.inputs.A_heat_oil_condensing__k__m2.private.add_measure("a_heat_oil_condensing_private", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil_condensing__k__m2.industry.add_measure("a_heat_oil_condensing_industry", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil_condensing__k__m2.public.add_measure("a_heat_oil_condensing_public", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil_condensing__k__m2.schools.add_measure("a_heat_oil_condensing_schools", 2024, 2025, 20.0);
    buildings.inputs.A_heat_oil_condensing__k__m2.apply_measures();

    buildings.inputs.A_heat_gas__k__m2.private.add_measure("a_heat_gas_private", 2024, 2025, 20.0);
    buildings.inputs.A_heat_gas__k__m2.industry.add_measure("a_heat_gas_industry", 2024, 2025, 20.0);
    buildings.inputs.A_heat_gas__k__m2.public.add_measure("a_heat_gas_public", 2024, 2025, 20.0);
    buildings.inputs.A_heat_gas__k__m2.schools.add_measure("a_heat_gas_schools", 2024, 2025, 20.0);
    buildings.inputs.A_heat_gas__k__m2.apply_measures();

    buildings.inputs.A_heat_heat_pump__k__m2.private.add_measure("a_heat_heat_pump_private", 2024, 2025, 20.0);
    buildings.inputs.A_heat_heat_pump__k__m2.industry.add_measure("a_heat_heat_pump_industry", 2024, 2025, 20.0);
    buildings.inputs.A_heat_heat_pump__k__m2.public.add_measure("a_heat_heat_pump_public", 2024, 2025, 20.0);
    buildings.inputs.A_heat_heat_pump__k__m2.schools.add_measure("a_heat_heat_pump_schools", 2024, 2025, 20.0);
    buildings.inputs.A_heat_heat_pump__k__m2.apply_measures();

    return buildings;
}


