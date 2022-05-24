use std::io::Write;
use std::io::prelude::Read;

fn main() {
    
    let res_main = error_checker_for_main(false);
    if let Err(e) = res_main
    {
        println!("Error in Main: {}", e);
    }

    print!("\n Press 'Enter' or 'Return' to continue...");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read(&mut [0u8]).unwrap();
    print!("\n");
}

fn menu_options(mut exit_value:bool) -> bool {
    while !exit_value {
        let mut input_option = String::new();

        print!("\n 1) Decimal -> Binary & Hexa\n 2) Binary -> Decimal & Hexa\n 3) Hexa -> Decimal & Binary\n 4) Exit\n");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input_option).unwrap();
        input_option = input_option.trim().to_string();
    
        if input_option == String::from("1")
        {
            let mut dec_input = String::new();
    
            print!("\n Enter a Decimal value\n");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut dec_input).unwrap();
            dec_input = dec_input.trim().to_string();

            if dec_input.is_empty()
            {
                println!("Oy! The input buffer is empty!\n");
                continue;
            }
            dec_input = rid_of_spaces(dec_input);

            let dec_value = dec_input.parse::<i32>().unwrap();
            let mut binary_dec_value = format!("{:b}", dec_value);

            let mut d_binary_data_size = binary_dec_value.chars().count();
            let mut d_b_index_modifier = 4;

            while d_binary_data_size % 4 != 0
            {
                binary_dec_value.insert(0, '0');
                d_binary_data_size = binary_dec_value.chars().count();
            }

            let mut d_b_loop_counter = (d_binary_data_size / 4) - 1;

            while d_b_loop_counter != 0
            {
                binary_dec_value.insert(d_binary_data_size - d_b_index_modifier,' ');
                d_b_index_modifier += 4;
                d_b_loop_counter -= 1;
            }
            
            println!("-------------------------------------");
            println!("    Decimal:   {}\n", dec_input);
            println!("    Binary:    {}\n", binary_dec_value);
            println!("    Hexa:      {:X}", dec_value);
            println!("-------------------------------------");
        }
        else if input_option == String::from("2")
        {
            let mut binary_input = String::new();
            let mut b_exponent_modifier = 1;
            let mut b_decimal_value = 0;
    
            print!("\n Enter a Binary value\n");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut binary_input).unwrap();
            binary_input = binary_input.trim().to_string();

            if binary_input.is_empty()
            {
                println!("Oy! The input buffer is empty!\n");
                continue;
            }
            binary_input = rid_of_spaces(binary_input);

            for d_index_value in binary_input.chars().rev()
            {
                if d_index_value == '1'
                {
                    b_decimal_value += b_exponent_modifier;
                }
                b_exponent_modifier *= 2;
            }

            let mut binary_data_size = binary_input.chars().count();
            let mut b_index_modifier = 4;

            while binary_data_size % 4 != 0
            {
                binary_input.insert(0, '0');
                binary_data_size = binary_input.chars().count();
            }

            let mut b_loop_counter = (binary_data_size / 4) - 1;

            while b_loop_counter != 0
            {
                binary_input.insert(binary_data_size - b_index_modifier,' ');
                b_index_modifier += 4;
                b_loop_counter -= 1;
            }
            
            println!("-------------------------------------");
            println!("    Binary:    {}\n", binary_input);
            println!("    Decimal:   {}\n", b_decimal_value);
            println!("    Hexa:      {:X}", b_decimal_value);
            println!("-------------------------------------");
        }
        else if input_option == String::from("3")
        {
            let mut abort = false;
            let mut hexa_input = String::new();
            let mut b_exponent_modifier = 1;
            let mut h_decimal_value = 0;
    
            print!("\n Enter a Hexa value\n");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut hexa_input).unwrap();
            hexa_input = hexa_input.trim().to_string().to_uppercase();

            if hexa_input.is_empty()
            {
                println!("Oy! The input buffer is empty!\n");
                continue;
            }
            hexa_input = rid_of_spaces(hexa_input);

            for h_index_value in hexa_input.chars().rev()
            {
                if h_index_value >= '0' && h_index_value <= '9' {
                    h_decimal_value += (h_index_value as u32 - 0x30 as u32) * b_exponent_modifier;
                }
                else if !(h_index_value > 'F') {
                    h_decimal_value += (h_index_value as u32 - 0x37 as u32) * b_exponent_modifier;
                }
                else
                {
                    println!("\nWait, hold up, illegal characters in this buffer!\n");
                    abort = true;
                    break;
                }
                b_exponent_modifier *= 16;
            }

            if abort
            {
                continue;
            }

            let mut binary_hexa_value = format!("{:b}", h_decimal_value);

            let mut h_binary_data_size = binary_hexa_value.chars().count();
            let mut h_b_index_modifier = 4;

            while h_binary_data_size % 4 != 0
            {
                binary_hexa_value.insert(0, '0');
                h_binary_data_size = binary_hexa_value.chars().count();
            }

            let mut h_b_loop_counter = (h_binary_data_size / 4) - 1;

            while h_b_loop_counter != 0
            {
                binary_hexa_value.insert(h_binary_data_size - h_b_index_modifier,' ');
                h_b_index_modifier += 4;
                h_b_loop_counter -= 1;
            }
            
            println!("-------------------------------------");
            println!("    Hexa:      {}\n", hexa_input);
            println!("    Decimal:   {}\n", h_decimal_value);
            println!("    Binary:    {}", binary_hexa_value);
            println!("-------------------------------------");
        }
        else if input_option == String::from("4")
        {
            println!("\n Exiting the Program\n");
            exit_value = true;
            break;
        }
        else
        {
            println!("\n Invalid input my dood!\n");
        }
    }
    return exit_value;
}

fn rid_of_spaces(mut input_value:String) -> String  {
    if input_value.find(" ") != None
    {
        input_value = input_value.replace(" ", "");
    }
    return input_value;
}

fn error_checker_for_main(exit_value:bool) -> std::io::Result<()> {
    menu_options(exit_value);
    Ok(())
}