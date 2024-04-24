pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

fn compare_license_types<T: Licensed>(software: T, software_two: T) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};
        assert!(some_software.licensing_info() == other_software.licensing_info());
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};
        assert!(some_software.licensing_info() == other_software.licensing_info());
    }
}