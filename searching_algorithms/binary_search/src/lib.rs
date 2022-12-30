struct BinarySearch();

impl BinarySearch {
    fn find<T>(data: &[T], item: T) -> bool
    where T: PartialEq + PartialOrd{

        let middle = data.len() / 2;

        if data.len() == 0 {
            return false;
        }
    
 
        if data[middle] == item {
            return true
        }

        if middle == 0 {
            return false;
        }

        if item < data[middle] {
            return BinarySearch::find(&data[..middle], item);
        } 

        if item > data[middle] {
            return BinarySearch::find(&data[middle + 1..], item);
        }

        return false
    }
}

#[cfg(test)]
mod find_tests {
    use super::*;

    #[test]
    fn empty() {
        let data = [];
        let is_found = BinarySearch::find(&data, 200);
        assert_eq!(is_found, false);
    }

    #[test]
    fn specific_string() {
        let data = ["Apple","Bannana", "Carrot", "Dragon Fruit", "Elderberry", "Fig", "Grape"];
        let is_found = BinarySearch::find(&data, "Carrot");
        assert_eq!(is_found, true);
    }

    #[test]
    fn specific_integer() {
        let data = [1,3, 3, 4, 12345, 9999999];
        let is_found = BinarySearch::find(&data, 4);
        assert_eq!(is_found, true);
    }

    #[test]
    fn specific_integer_to_left() {
        let data = [1,3, 3, 4, 12345, 88888, 99999];
        let is_found = BinarySearch::find(&data, 3);
        assert_eq!(is_found, true);
    }

    #[test]
    fn specific_integer_to_right() {
        let data = [1,3, 3, 4, 12345, 88888, 99999];
        let is_found = BinarySearch::find(&data, 99999);
        assert_eq!(is_found, true);
    }

    #[test]
    fn specific_integer_in_middle() {
        let data = [1,3, 3, 4, 12345, 88888, 99999];
        let is_found = BinarySearch::find(&data, 12345);
        assert_eq!(is_found, true);
    }

    #[test]
    fn specific_float() {
        let data = [1.2,3.2, 3.4, 4.5331, 4.6, 4.75];
        let is_found = BinarySearch::find(&data, 4.5331);
        assert_eq!(is_found, true);
    }

    #[test]
    fn does_not_exist() {
        let data = [1.2,3.2, 3.4, 4.5331, 4.6, 4.75];
        let is_found = BinarySearch::find(&data, 12.3);
        assert_eq!(is_found, false);
    }
}