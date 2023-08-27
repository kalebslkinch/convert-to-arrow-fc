pub fn export_function(string: &String) -> String {
    let mut new_string = String::new();
  
    let mut temp_string = String::new();

    if string.contains("expor function") {
        new_string.push_str(&string.replace("export function", "const"));
    }

    if string.contains("({}) {") {
        temp_string.push_str(&new_string.replace("({}) {", ": FC = ({}) => {"));
        new_string.clear();
        new_string.push_str(&temp_string);
        temp_string.clear();
    }
    if string.contains("() {") {
        temp_string.push_str(&new_string.replace("() {", ": FC = () => {"));
        new_string.clear();
        new_string.push_str(&temp_string);
        temp_string.clear();
    }


    if string.contains("import React from 'react';") {
        temp_string.push_str(&new_string.replace("import React from 'react';", "import React, { FC } from 'react';\n"));
        new_string.clear();
        new_string.push_str(&temp_string);
        temp_string.clear();
    }

    if string.contains("import React from \"react\";") {
        temp_string.push_str(&new_string.replace("import React from \"react\";", "import React, { FC } from \"react\";\n"));
        new_string.clear();
        new_string.push_str(&temp_string);
        temp_string.clear();
    }   

    if new_string.contains("const"){
        let mut new_string_vec2: Vec<&str> = new_string.split(":").collect();
        let mut new_string_vec3: Vec<&str> = new_string_vec2[0].split(" ").collect();
        let mut new_string_vec4: Vec<&str> = new_string_vec3[new_string_vec3.len()-1].split("(").collect();
        new_string.push_str(&format!("\nexport default {};", new_string_vec4[0]));
    }

        
    new_string
    

}




