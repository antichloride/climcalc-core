use crate::input::Input;
use crate::result::Results;
use std::ops;


pub struct SectorsInputs {
    pub private: Input,
    pub industry: Input,
    pub schools: Input,
    pub public: Input,
}

impl SectorsInputs{

    pub fn new(id: String, start_year: u32, end_year: u32) -> SectorsInputs{
        return SectorsInputs{
            private: Input::new((id.to_owned()+"/private").to_string(), start_year, end_year),
            industry: Input::new((id.to_owned()+"/industry").to_string(), start_year, end_year),
            schools: Input::new((id.to_owned()+"/schools").to_string(), start_year, end_year),
            public: Input::new((id.to_owned()+"/public").to_string(), start_year, end_year),
        }
    }

    pub fn get_year(&self, year: u32) -> SectorsRawValues{
        return SectorsRawValues{
            private: self.private.get_year(year),
            industry: self.industry.get_year(year),
            schools: self.schools.get_year(year),
            public: self.public.get_year(year),
        };
    }

    pub fn get_inputs(& self) -> Vec<&Input>{
        return Vec::from([&self.private, &self.industry, &self.schools, &self.public])
    }

    pub fn get_input_by_id(&mut self, id: &str) -> Option<&mut Input>{
        match id{
            "private" => Some(&mut self.private),
            "industry" => Some(&mut self.industry),
            "schools" => Some(&mut self.schools),
            "public" => Some(&mut self.public),
            _ => None,

        }

    }

    pub fn apply_measures(&mut self){
        self.private.apply_measures();
        self.industry.apply_measures();
        self.schools.apply_measures();
        self.public.apply_measures();
    }
}

#[derive(Copy, Clone)]
pub struct SectorsRawValues {
    pub private: f32,
    pub industry: f32,
    pub schools: f32,
    pub public: f32,
}

impl SectorsRawValues{
    pub fn new() -> SectorsRawValues{
        return SectorsRawValues{
            private: 0.0,
            industry: 0.0,
            schools: 0.0,
            public: 0.0,
        };
    }

    pub fn is_greater(&self, other: &SectorsRawValues) -> SectorsRawValues{
        return SectorsRawValues{
            private: (self.private > other.private) as usize as f32,
            industry: (self.industry > other.industry) as usize as f32,
            schools: (self.schools > other.schools) as usize as f32,
            public: (self.public > other.public) as usize as f32,
        };
    }

    pub fn sum(&self) -> f32{
        return self.private + self.industry + self.schools + self.public;
    }

    pub fn copy(&self) -> SectorsRawValues{
        return SectorsRawValues{
            private: self.private,
            industry: self.industry,
            schools: self.schools,
            public: self.public,
        };
    }
}

impl ops::Mul<SectorsRawValues> for SectorsRawValues{
    type Output = SectorsRawValues;
    fn mul(self, _rhs: SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private * _rhs.private,
            industry: self.industry * _rhs.industry,
            schools: self.schools * _rhs.schools,
            public: self.public * _rhs.public,
        }
    }
}
impl ops::Add<SectorsRawValues> for SectorsRawValues{
    type Output = SectorsRawValues;
    fn add(self, _rhs: SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private + _rhs.private,
            industry: self.industry + _rhs.industry,
            schools: self.schools + _rhs.schools,
            public: self.public + _rhs.public,
        }
    }
}

impl ops::Mul<&SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn mul(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private * _rhs.private,
            industry: self.industry * _rhs.industry,
            schools: self.schools * _rhs.schools,
            public: self.public * _rhs.public,
        }
    }
}

impl ops::Add<&SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn add(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private + _rhs.private,
            industry: self.industry + _rhs.industry,
            schools: self.schools + _rhs.schools,
            public: self.public + _rhs.public,
        }
    }
}

impl ops::Sub<&SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn sub(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private - _rhs.private,
            industry: self.industry - _rhs.industry,
            schools: self.schools - _rhs.schools,
            public: self.public - _rhs.public,
        }
    }
}

impl ops::Sub<SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn sub(self, _rhs: SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private - _rhs.private,
            industry: self.industry - _rhs.industry,
            schools: self.schools - _rhs.schools,
            public: self.public - _rhs.public,
        }
    }
}

impl ops::Sub<&SectorsRawValues> for SectorsRawValues{
    type Output = SectorsRawValues;
    fn sub(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private - _rhs.private,
            industry: self.industry - _rhs.industry,
            schools: self.schools - _rhs.schools,
            public: self.public - _rhs.public,
        }
    }
}

impl ops::Sub<SectorsRawValues> for SectorsRawValues{
    type Output = SectorsRawValues;
    fn sub(self, _rhs: SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private - _rhs.private,
            industry: self.industry - _rhs.industry,
            schools: self.schools - _rhs.schools,
            public: self.public - _rhs.public,
        }
    }
}

impl ops::Div<&SectorsRawValues> for &SectorsRawValues{
    type Output = SectorsRawValues;
    fn div(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private / _rhs.private,
            industry: self.industry / _rhs.industry,
            schools: self.schools / _rhs.schools,
            public: self.public / _rhs.public,
        }
    }
}


impl ops::Mul<&SectorsRawValues> for SectorsRawValues{
    type Output = SectorsRawValues;
    fn mul(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
        SectorsRawValues {
            private: self.private * _rhs.private,
            industry: self.industry * _rhs.industry,
            schools: self.schools * _rhs.schools,
            public: self.public * _rhs.public,
        }
    }
}

macro_rules! impl_math_ops_for_sector {
    ($($t:ty),*) => {
        $(
            impl ops::Mul<SectorsRawValues> for $t{
                type Output = SectorsRawValues;
                fn mul(self, _rhs: SectorsRawValues) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self * _rhs.private as f32,
                        industry: self * _rhs.industry as f32,
                        schools: self * _rhs.schools as f32,
                        public: self * _rhs.public as f32,
                    }
                }
            }
            impl ops::Mul<$t> for SectorsRawValues{
                type Output = SectorsRawValues;
                fn mul(self, _rhs: $t) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self.private * _rhs as f32,
                        industry: self.industry * _rhs as f32,
                        schools: self.schools * _rhs as f32,
                        public: self.public * _rhs as f32,
                    }
                }
            }
            impl ops::Div<$t> for SectorsRawValues{
                type Output = SectorsRawValues;
                fn div(self, _rhs: $t) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self.private / _rhs as f32,
                        industry: self.industry / _rhs as f32,
                        schools: self.schools / _rhs as f32,
                        public: self.public / _rhs as f32,
                    }
                }
            }
            impl ops::Mul<&SectorsRawValues> for $t{
                type Output = SectorsRawValues;
                fn mul(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self * _rhs.private as f32,
                        industry: self * _rhs.industry as f32,
                        schools: self * _rhs.schools as f32,
                        public: self * _rhs.public as f32,
                    }
                }
            }
            impl ops::Sub<&SectorsRawValues> for $t{
                type Output = SectorsRawValues;
                fn sub(self, _rhs: &SectorsRawValues) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self - _rhs.private as f32,
                        industry: self - _rhs.industry as f32,
                        schools: self - _rhs.schools as f32,
                        public: self - _rhs.public as f32,
                    }
                }
            }
            impl ops::Mul<$t> for &SectorsRawValues{
                type Output = SectorsRawValues;
                fn mul(self, _rhs: $t) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self.private * _rhs as f32,
                        industry: self.industry * _rhs as f32,
                        schools: self.schools * _rhs as f32,
                        public: self.public * _rhs as f32,
                    }
                }
            }
            impl ops::Div<$t> for &SectorsRawValues{
                type Output = SectorsRawValues;
                fn div(self, _rhs: $t) -> SectorsRawValues {
                    SectorsRawValues {
                        private: self.private / _rhs as f32,
                        industry: self.industry / _rhs as f32,
                        schools: self.schools / _rhs as f32,
                        public: self.public / _rhs as f32,
                    }
                }
            }
        )*
    }
}

impl_math_ops_for_sector!{f32}

pub struct SectorsResult{
    private: Results,
    industry: Results,
    schools: Results,
    public: Results,
}

impl SectorsResult{

    pub fn new(id: String, start_year: u32, end_year: u32) -> SectorsResult{
        return SectorsResult{
            private: Results::new((id.to_owned()+"/private").to_string(), start_year, end_year),
            industry: Results::new((id.to_owned()+"/industry").to_string(), start_year, end_year),
            schools: Results::new((id.to_owned()+"/schools").to_string(), start_year, end_year),
            public: Results::new((id.to_owned()+"/public").to_string(), start_year, end_year),
        }
    }

    pub fn get_year(&self, year: u32) -> SectorsRawValues{
        return SectorsRawValues{
            private: self.private.get_year(year),
            industry: self.industry.get_year(year),
            schools: self.schools.get_year(year),
            public: self.public.get_year(year),
        };
    }

    pub fn sum_years(
        &self,
        start_year: u32,
        end_year: u32
    ) -> SectorsRawValues{

        return SectorsRawValues{
            private: self.private.sum_years(start_year, end_year),
            industry: self.industry.sum_years(start_year, end_year),
            schools: self.schools.sum_years(start_year, end_year),
            public: self.public.sum_years(start_year, end_year),
        };

    }

    pub fn set_year_values(&mut self, year: u32, values: &SectorsRawValues){
        self.private.set_year_value(year, values.private);
        self.industry.set_year_value(year, values.industry);
        self.schools.set_year_value(year, values.schools);
        self.public.set_year_value(year, values.public);
    }

    pub fn get_results(& self) -> Vec<&Results>{
        return Vec::from([&self.private, &self.industry, &self.schools, &self.public])
    }

    pub fn get_results_by_id(&mut self, id: &str) -> Option<&mut Results>{
        match id{
            "private" => Some(&mut self.private),
            "industry" => Some(&mut self.industry),
            "schools" => Some(&mut self.schools),
            "public" => Some(&mut self.public),
            _ => None,

        }

    }
}


#[cfg(test)]
impl SectorsResult{

    pub fn get_year_values(&self, year: u32) -> [f32; 4]{
        return [
            self.private.get_year(year),
            self.industry.get_year(year),
            self.schools.get_year(year),
            self.public.get_year(year),
        ];
    }
}

#[cfg(test)]
impl SectorsInputs{

    pub fn get_year_values(&self, year: u32) -> [f32; 4]{
        return [
            self.private.get_year(year),
            self.industry.get_year(year),
            self.schools.get_year(year),
            self.public.get_year(year),
        ];
    }
}

#[cfg(test)]
impl SectorsRawValues{
    pub fn set(&mut self, private: f32, industry:f32, schools: f32, public: f32){
        self.private = private;
        self.industry = industry;
        self.schools = schools;
        self.public = public;
    }

}


#[cfg(test)]
mod tests{
    use crate::sectors::SectorsRawValues;

    #[test]
    fn test_is_greater() {
        let mut raw_a = SectorsRawValues::new();
        raw_a.set(1.1, 2.2, 1.0, 2.0);
        let mut raw_b = SectorsRawValues::new();
        raw_b.set(1.0, 2.0, 1.1, 2.2);
        let comp = raw_a.is_greater(&raw_b);
        assert!(comp.private == 1.0);
        assert!(comp.industry == 1.0);
        assert!(comp.schools == 0.0);
        assert!(comp.public == 0.0);
    }
}
