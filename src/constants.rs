
pub mod buildings{
    pub struct HeatType {
        pub efficency: f32,
        pub invest: f32,
        pub grant: f32,
    }

    pub const oil_no_condensing: HeatType = HeatType {
        efficency: 0.7,
        invest: 0.5,
        grant: 0.0,
    };
    pub const oil_with_condensing: HeatType = HeatType {
        efficency: 0.9,
        invest: 1.0,
        grant: 0.1,
    };
    pub const gas: HeatType = HeatType {
        efficency: 0.9,
        invest: 0.5,
        grant: 0.2,
    };
    pub const heat_pump: HeatType = HeatType {
        efficency: 3.0,
        invest: 2.0,
        grant: 0.3,
    };
    pub const other: HeatType = HeatType {
        efficency: 0.5,
        invest: 0.0,
        grant: 0.5,
    };

    pub mod EnergySource{

        pub struct EnergySource {
            pub calories: f32,
            pub price: f32,
            pub emission: f32,
        }

        pub const oil: EnergySource = EnergySource{
            calories: 10.0,
            price: 0.9,
            emission: 3.0,
        };

        pub const gas: EnergySource = EnergySource{
            calories: 5.0,
            price: 0.9,
            emission: 2.0,
        };

        pub const other: EnergySource = EnergySource{
            calories: 0.0,
            price: 0.9,
            emission: 0.0,
        };

    }

    pub mod energetic_restoration{
        pub const invest: f32 = 2.5;
        pub const grant: f32 = 0.3;
    }



}

pub mod mobility{

        pub const price_fuel: f32 = 1.7;

        pub struct Vehicle {
            pub price: f32,
            pub grant: f32,
            pub consumption: f32,
            pub fuel_costs: f32,
            pub operation_costs: f32,
            pub taxes_and_checks: f32,
            pub lifecycle: f32,
        }

        pub const bev: Vehicle = Vehicle{
            price: 35.0,
            grant: 9.0,
            consumption: 24.0,
            fuel_costs: 0.3,
            operation_costs: 0.05,
            taxes_and_checks: 1.0,
            lifecycle: 10.0,
        };


        pub const combustor: Vehicle = Vehicle{
            price: 30.0,
            grant: 0.0,
            consumption: 70.0,
            fuel_costs: 0.17,
            operation_costs: 0.05,
            taxes_and_checks: 1.0,
            lifecycle: 10.0,
        };
}
