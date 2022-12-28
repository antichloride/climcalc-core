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

    buildings.inputs.n_inhabitants__k__.set(83200.0, 44746.0, 10735.0, 1556.0);
    buildings.inputs.n_buildings.set(19273286.0, 1986122.0, 56498.0, 128207.0);
    buildings.inputs.floor_A_building__m2.set(198.0, 737.0, 1843.0, 337.0);

    buildings.inputs.heat_dmd__k__W_h_per_m2_a.set(114.0, 96.0, 50.0, 50.0);
    buildings.inputs.hot_water_dmd__k__W_h_per_m2_a.set(1238.0, 0.0, 10.0, 0.0);
    buildings.inputs.elec_dmd_capita__k_W_h_per_a.set(1253.0, 2058.0, 50.0, 2058.0);

    buildings.inputs.A_heat_oil__k__m2.set(448022.0, 267238.0, 31240.0, 7886.0);
    buildings.inputs.A_heat_oil_condensing__k__m2.set(496858.0, 0.0, 52066.0, 0.0);
    buildings.inputs.A_heat_gas__k__m2.set(1885950.0, 766857.0, 15620.0, 22629.0);
    buildings.inputs.A_heat_heat_pump__k__m2.set(106680.0, 0.0, 0.0, 0.0);
    buildings.inputs.A_heat_other__k__m2.set(335.280, 116190.0, 5207.0, 3429.0);

    return buildings;
}


