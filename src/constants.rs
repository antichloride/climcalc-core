
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
}
