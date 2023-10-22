use crate::buildings::tests::buildings_test_case::create_buildings;
use crate::energy::tests::energy_test_case::create_energy;
use crate::mobility::tests::mobility_test_case::create_mobility;
use crate::Calculator;

#[test]
fn test_economy_calculate() {

    let start_year: u32 = 2022 as u32;
    let end_year: u32 = 2045 as u32;
    let buildings = create_buildings(start_year, end_year);
    let energy = create_energy(start_year, end_year);
    let mobility = create_mobility(start_year, end_year);
    let mut calculator = Calculator::new(start_year, end_year);
    calculator.buildings = buildings;
    calculator.energy = energy;
    calculator.mobility = mobility;
    calculator.calculate_over_years();

    let economy = calculator.economy;
    let stakeholders = calculator.stakeholders;

    // [start:assert_measures]

	// Invest Heizung Material  (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_heating_material.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heating_material.get_year(2024),
		1.0966754155730447,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heating_material.get_year(2025),
		1.069294463192118,
		max_relative=1e-3,
	);

	// Invest Heizung Arbeit  (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_heating_work.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heating_work.get_year(2024),
		1.0966754155730447,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heating_work.get_year(2025),
		1.069294463192118,
		max_relative=1e-3,
	);

	// Umsatz Heizung Handwerk lokal  (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_heating_crafting_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heating_crafting_local.get_year(2024),
		0.3290026246719134,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heating_crafting_local.get_year(2025),
		0.32078833895763537,
		max_relative=1e-3,
	);

	// Umsatz Heizung Handwerk national  (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_heating_crafting_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heating_crafting_national.get_year(2024),
		1.0966754155730447,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heating_crafting_national.get_year(2025),
		1.069294463192118,
		max_relative=1e-3,
	);

	// Umsatz Heizung Herstellung national  (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_heating_production_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heating_production_national.get_year(2024),
		0.5483377077865224,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heating_production_national.get_year(2025),
		0.534647231596059,
		max_relative=1e-3,
	);

	// Invest Wärmebedarf Material  (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_heat_demand_material.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heat_demand_material.get_year(2024),
		18.638677983016073,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heat_demand_material.get_year(2025),
		18.63867798301598,
		max_relative=1e-3,
	);

	// Invest Wärmebedarf Arbeit  (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_heat_demand_work.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heat_demand_work.get_year(2024),
		43.4902486270375,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_heat_demand_work.get_year(2025),
		43.49024862703729,
		max_relative=1e-3,
	);

	// Umsatz Wärmebedarf Handwerk lokal (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_heat_demand_crafting_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heat_demand_crafting_local.get_year(2024),
		13.04707458811125,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heat_demand_crafting_local.get_year(2025),
		13.047074588111187,
		max_relative=1e-3,
	);

	// Umsatz Wärmebedarf Handwerk national (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_heat_demand_crafting_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heat_demand_crafting_national.get_year(2024),
		43.4902486270375,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heat_demand_crafting_national.get_year(2025),
		43.49024862703729,
		max_relative=1e-3,
	);

	// Umsatz Wärmebedarf Herstellung national (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_heat_demand_production_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heat_demand_production_national.get_year(2024),
		9.319338991508037,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_heat_demand_production_national.get_year(2025),
		9.31933899150799,
		max_relative=1e-3,
	);

	// Invest PV auf Dach Material (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_solar_roof_material.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_roof_material.get_year(2024),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_roof_material.get_year(2025),
		0.0,
		max_relative=1e-3,
	);

	// Invest PV auf Dach Arbeit (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_solar_roof_work.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_roof_work.get_year(2024),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_roof_work.get_year(2025),
		0.0,
		max_relative=1e-3,
	);

	// Wartung PV auf Dach Arbeit (in Mio. €/a)
	assert_relative_eq!(
		economy.maintenance_solar_roof_work.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.maintenance_solar_roof_work.get_year(2024),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.maintenance_solar_roof_work.get_year(2025),
		0.0,
		max_relative=1e-3,
	);

	// Umsatz PV auf Dach Handwerk lokal (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_solar_roof_crafting_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_roof_crafting_local.get_year(2024),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_roof_crafting_local.get_year(2025),
		0.0,
		max_relative=1e-3,
	);

	// Umsatz PV auf Dach Handwerk national (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_solar_roof_crafting_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_roof_crafting_national.get_year(2024),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_roof_crafting_national.get_year(2025),
		0.0,
		max_relative=1e-3,
	);

	// Invest PV Freifläche Material (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_solar_landscape_material.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_landscape_material.get_year(2024),
		1.822230392156862,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_landscape_material.get_year(2025),
		1.8222303921568634,
		max_relative=1e-3,
	);

	// Invest PV Freifläche Arbeit  (in Mio. €/a)
	assert_relative_eq!(
		economy.invest_solar_landscape_work.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_landscape_work.get_year(2024),
		0.9812009803921564,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.invest_solar_landscape_work.get_year(2025),
		0.9812009803921572,
		max_relative=1e-3,
	);

	// Wartung PV Freifläche Arbeit (in Mio. €/a)
	assert_relative_eq!(
		economy.maintenance_solar_landscape_work.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.maintenance_solar_landscape_work.get_year(2024),
		0.056068627450980374,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.maintenance_solar_landscape_work.get_year(2025),
		0.1121372549019608,
		max_relative=1e-3,
	);

	// Umsatz PV Freifläche Handwerk lokal (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_solar_landscape_crafting_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_landscape_crafting_local.get_year(2024),
		0.008410294117647056,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_landscape_crafting_local.get_year(2025),
		0.01682058823529412,
		max_relative=1e-3,
	);

	// Umsatz PV Freifläche Handwerk national (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_solar_landscape_crafting_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_landscape_crafting_national.get_year(2024),
		0.056068627450980374,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_landscape_crafting_national.get_year(2025),
		0.1121372549019608,
		max_relative=1e-3,
	);

	// Umsatz PV Freifläche Herstellung national (in Mio. €/a)
	assert_relative_eq!(
		economy.turnover_solar_landscape_production_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_landscape_production_national.get_year(2024),
		0.4906004901960782,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_solar_landscape_production_national.get_year(2025),
		0.4906004901960786,
		max_relative=1e-3,
	);

	// Beschäftigte Handwerk lokal
	assert_relative_eq!(
		economy.n_jobs_crafting_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.n_jobs_crafting_local.get_year(2024),
		103.30900836164757,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.n_jobs_crafting_local.get_year(2025),
		103.31052126483236,
		max_relative=1e-3,
	);

	// Beschäftigte Handwerk national
	assert_relative_eq!(
		economy.n_jobs_crafting_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.n_jobs_crafting_national.get_year(2024),
		344.57974581861856,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.n_jobs_crafting_national.get_year(2025),
		344.8011734423613,
		max_relative=1e-3,
	);

	// Beschäftigte Herstellung national
	assert_relative_eq!(
		economy.n_jobs_production_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.n_jobs_production_national.get_year(2024),
		50.775868575934496,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.n_jobs_production_national.get_year(2025),
		50.708758398530044,
		max_relative=1e-3,
	);

	// Einkommen lokal
	assert_relative_eq!(
		economy.income_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_local.get_year(2024),
		4.097770264701959,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_local.get_year(2025),
		4.097830274276944,
		max_relative=1e-3,
	);

	// Einkommen national
	assert_relative_eq!(
		economy.income_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_national.get_year(2024),
		2.638188678923799,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_national.get_year(2025),
		2.639883984168079,
		max_relative=1e-3,
	);

	// Einkommenssteuer lokal
	assert_relative_eq!(
		economy.income_tax_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_tax_local.get_year(2024),
		0.11864393929032963,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_tax_local.get_year(2025),
		0.1186456767650809,
		max_relative=1e-3,
	);

	// Einkommenssteuer national
	assert_relative_eq!(
		economy.income_tax_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_tax_national.get_year(2024),
		0.06547015170553754,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.income_tax_national.get_year(2025),
		0.06551222295404864,
		max_relative=1e-3,
	);

	// Umsatz lokal mit lokalem Anteil Material (in Mio. €)
	assert_relative_eq!(
		economy.turnover_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_local.get_year(2024),
		19.578428085301073,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_local.get_year(2025),
		19.570409807990075,
		max_relative=1e-3,
	);

	// Gewinn lokal mit lokalem Anteil Material (in Mio. €)
	assert_relative_eq!(
		economy.revenue_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.revenue_local.get_year(2024),
		1.4292252502269782,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.revenue_local.get_year(2025),
		1.4286399159832754,
		max_relative=1e-3,
	);

	// Umsatz national
	assert_relative_eq!(
		economy.turnover_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_national.get_year(2024),
		67.12570881374869,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_national.get_year(2025),
		67.07094690898651,
		max_relative=1e-3,
	);

	// Umsatzsteuer national
	assert_relative_eq!(
		economy.turnover_tax_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_tax_national.get_year(2024),
		0.5012558489955218,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_tax_national.get_year(2025),
		0.501577956991935,
		max_relative=1e-3,
	);

	// Umsatzsteuer lokal
	assert_relative_eq!(
		economy.turnover_tax_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_tax_local.get_year(2024),
		0.014135625077587375,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.turnover_tax_local.get_year(2025),
		0.014129835881368834,
		max_relative=1e-3,
	);

	// Gewerbesteuer lokal
	assert_relative_eq!(
		economy.business_tax_local.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.business_tax_local.get_year(2024),
		0.21759954434705744,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.business_tax_local.get_year(2025),
		0.21751042720845368,
		max_relative=1e-3,
	);

	// Gewerbesteuer national
	assert_relative_eq!(
		economy.business_tax_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.business_tax_national.get_year(2024),
		0.7460519091832063,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.business_tax_national.get_year(2025),
		0.7454432716832033,
		max_relative=1e-3,
	);

	// Körperschaftssteuer national
	assert_relative_eq!(
		economy.corporate_tax_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.corporate_tax_national.get_year(2024),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.corporate_tax_national.get_year(2025),
		0.0,
		max_relative=1e-3,
	);

	// Energiesteuer national (in Mio. €)
	assert_relative_eq!(
		economy.energy_tax_national.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.energy_tax_national.get_year(2024),
		-11.727419747189568,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		economy.energy_tax_national.get_year(2025),
		-23.409515473654498,
		max_relative=1e-3,
	);

    // [end:assert_measures]


    // [start:assert_measures_stakeholders]

	// Investition
	assert_relative_eq!(
		stakeholders.private_invest.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.private_invest.get_year(2024),
		45.68359945818361,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.private_invest.get_year(2025),
		45.62883755342152,
		max_relative=1e-3,
	);

	// Effekte eigener Maßnahmen
	assert_relative_eq!(
		stakeholders.private_effect_of_measures.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.private_effect_of_measures.get_year(2024),
		12.738349676554986,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.private_effect_of_measures.get_year(2025),
		25.456277786750817,
		max_relative=1e-3,
	);

	// Cash Flow Netto
	assert_relative_eq!(
		stakeholders.private_cash_flow_netto.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.private_cash_flow_netto.get_year(2024),
		58.4219491347386,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.private_cash_flow_netto.get_year(2025),
		71.08511534017234,
		max_relative=1e-3,
	);

	// Investition
	assert_relative_eq!(
		stakeholders.industry_invest.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_invest.get_year(2024),
		1.6299019607843126,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_invest.get_year(2025),
		1.629901960784315,
		max_relative=1e-3,
	);

	// Effekte eigener Maßnahmen
	assert_relative_eq!(
		stakeholders.industry_effect_of_measures.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_effect_of_measures.get_year(2024),
		0.1204227941176469,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_effect_of_measures.get_year(2025),
		0.24084558823529412,
		max_relative=1e-3,
	);

	// Gewinne aus Maßnahmen anderer
	assert_relative_eq!(
		stakeholders.industry_profit_from_measures.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_profit_from_measures.get_year(2024),
		1.4292252502269782,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_profit_from_measures.get_year(2025),
		1.4286399159832754,
		max_relative=1e-3,
	);

	// Cash Flow Netto
	assert_relative_eq!(
		stakeholders.industry_cash_flow_netto.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_cash_flow_netto.get_year(2024),
		3.179550005128938,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.industry_cash_flow_netto.get_year(2025),
		3.2993874650028845,
		max_relative=1e-3,
	);

	// Investition
	assert_relative_eq!(
		stakeholders.community_invest.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_invest.get_year(2024),
		1.173529411764706,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_invest.get_year(2025),
		1.173529411764706,
		max_relative=1e-3,
	);

	// Effekte eigener Maßnahmen
	assert_relative_eq!(
		stakeholders.community_effect_of_measures.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_effect_of_measures.get_year(2024),
		0.08670441176470585,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_effect_of_measures.get_year(2025),
		0.17340882352941175,
		max_relative=1e-3,
	);

	// Einnahmen aus Maßnahmen anderer
	assert_relative_eq!(
		stakeholders.community_tax_income_from_measures.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_tax_income_from_measures.get_year(2024),
		0.3362434836373871,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_tax_income_from_measures.get_year(2025),
		0.33615610397353457,
		max_relative=1e-3,
	);

	// Cash Flow Netto
	assert_relative_eq!(
		stakeholders.community_cash_flow_netto.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_cash_flow_netto.get_year(2024),
		1.5964773071667988,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.community_cash_flow_netto.get_year(2025),
		1.6830943392676523,
		max_relative=1e-3,
	);

	// Mehr-Ausgaben
	assert_relative_eq!(
		stakeholders.federal_additional_expenses.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.federal_additional_expenses.get_year(2024),
		-108.63867798301607,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.federal_additional_expenses.get_year(2025),
		-108.638677983016,
		max_relative=1e-3,
	);

	// Mehr-Einnahmen aus Steuern und Abgaben
	assert_relative_eq!(
		stakeholders.federal_additional_tax_income.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.federal_additional_tax_income.get_year(2024),
		-10.414641837305302,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.federal_additional_tax_income.get_year(2025),
		-22.096982022025312,
		max_relative=1e-3,
	);

	// Cash Flow Netto
	assert_relative_eq!(
		stakeholders.federal_cash_flow_netto.get_year(2023),
		0.0,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.federal_cash_flow_netto.get_year(2024),
		-119.05331982032136,
		max_relative=1e-3,
	);
	assert_relative_eq!(
		stakeholders.federal_cash_flow_netto.get_year(2025),
		-130.7356600050413,
		max_relative=1e-3,
	);

    // [end:assert_measures_stakeholders]
}
