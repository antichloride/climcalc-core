use crate::buildings::tests::buildings_test_case::create_buildings;
use crate::energy::tests::energy_test_case::create_energy;
use crate::mobility::tests::mobility_test_case::create_mobility;
use crate::Calculator;

#[test]
fn test_economy_calculate() {

    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let mut buildings = create_buildings(start_year, end_year);
    let mut energy = create_energy(start_year, end_year);
    let mut mobility = create_mobility(start_year, end_year);
    let mut calculator = Calculator::new(start_year, end_year);
    calculator.buildings = buildings;
    calculator.energy = energy;
    calculator.mobility = mobility;
    calculator.calculate_over_years();

    assert_relative_eq!(calculator.economy.invest_heating_material.get_year(2024), 2.0, max_relative=0.1);
}
