pub fn export_function(string: &String) -> String {
    let first_string = &string.replace("export function", "const");
    let mut second_string = String::new();
    if string.contains("({}) {") {
        second_string.push_str(&first_string.replace("({}) {", ": FC = ({}) => {"));
    }
    if string.contains("() {") {
        second_string.push_str(&first_string.replace("() {", " = () => {"));
    }
    let third_str = String::from("import type { FC } from react; \n") + &second_string;
    let fourth_str = third_str + &String::from("\n export default ");
    fourth_str
}
