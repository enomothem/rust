// Title: Contorl flow 

fn main()
{
    if_flow();
    loop_flow();
    while_flow();
    for_array_flow();
    for_flow();

}


fn if_flow()
{
    let cat = 3;
    if cat < 5
    {
        println!("true .");
    }else
    {
        println!("false .");
    }
}

// ;;;; ;;;; ;;;; ;;;; ;;
// for . loop . while  ;;
// ;;;; ;;;; ;;;; ;;;; ;;

fn loop_flow()
{
    loop
    {
        println!("========================== .");  // still ` Ctl + C ` stop.
        // break;
        if_flow();
        let x = 12;
        if x < 13
        {
            break;
        }
    }
}

fn while_flow()
{
    let mut fk = 3;
    while fk != 0
    {
        println!("{fk}");
        fk -= 1;
    }
    println!("end!");
}

// for -0 //

fn for_array_flow()
{
    println!("This for_array_flow test zone !");
    let array =  [ 2, 3, 12, 13, 111, 134 ];
    let mut index = 0;

    while index < 6
    {
        println!("=========={}===========",array[index]);
        index += 1;
    }

    for rust in array
    {
        println!("=========={rust}===========");
    }
}

fn for_flow()
{
    // var ;;

    let year : i16 = 2022;
    // let seven = [25, 28, 29, 30];


    for rust in 1..13
    {   
        if rust == 1 || rust == 3 || rust == 5 || rust == 7 || rust == 8 || rust == 10 || rust == 12
        {
            if rust == 7
            {
                println!("==== {year}/{rust} ====");
                for rust in 1..32
                {
                    
                    if rust == 25 || rust == 26 || rust == 27 || rust == 28 || rust == 29 || rust == 30 || rust == 31
                    
                    {
                        println!("|=# {rust} -- hvv");

                    }
                    else
                    {
                        println!("|=# {rust}");
                    }

                    
                }
            }
            else if rust == 8
            {
                println!("==== {year}/{rust} ====");
                for rust in 1..32
                {
                    
                    if rust == 1 || rust == 2 || rust == 3 || rust == 4 || rust == 5 || rust == 6 || rust == 7 || rust == 8
                    
                    {
                        println!("|=# {rust} -- hvv");

                    }
                    else
                    {
                        println!("|=# {rust}");
                    }

                    
                }
            }
            else
            {
                println!("==== {year}/{rust} ====");
                for rust in 1..32
                {
                    println!("|=# {rust}");
                }
            }

        }
        else if  rust == 4 || rust == 6 || rust == 9 || rust == 11
        {
            println!("==== {year}/{rust} ====");
            for rust in 1..31
            {
                println!("|=# {rust}");
            }

        }
        else // two month
        {
            println!("==== {year}/{rust} ====");
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
            {
                for rust in 1..30
                {
                    print!("|=# {rust}");
                    if rust%2 != 0 && rust < 10
                    {
                        if rust == 3
                        {
                            println!(" <-- Get it!");
                        }
                        else if rust == 9
                        {
                            println!("\n|=# \\ --> hahahahahahahaaaa..jfaoiwjeoif");
                        }
                        else
                        {
                            println!(" <--- singular");
                        }
                    }
                    else if rust == 9
                    {
                        println!("fwas");
                    }
                    else if rust == 6
                    {
                        println!(" <-- enomothem");
                    }
                    else if rust == 4
                    {
                        println!(" <-- 😂不要太卷了，停下来，享受时间，虚度时间，就这样，享受这精致的宇宙!")
                    }
                    else
                    {
                        println!();
                    }
                }
            }
            else
            {
                for rust in 1..29
                {
                    print!("|=# {rust}");
                    if rust%2 != 0 && rust < 10
                    {
                        if rust == 3
                        {
                            println!(" <-- Get it!");
                        }
                        else if rust == 9
                        {
                            println!("\n|=# \\ --> hahahahahahahaaaa..jfaoiwjeoif");
                        }
                        else
                        {
                            println!(" <--- singular");
                        }
                    }
                    else if rust == 9
                    {
                        println!("fwas");
                    }
                    else if rust == 6
                    {
                        println!(" <-- enomothem");
                    }
                    else if rust == 4
                    {
                        println!(" <-- 😂不要太卷了，停下来，享受时间，虚度时间，就这样，享受这精致的宇宙!")
                    }
                    else
                    {
                        println!();
                    }
                }
            }
        }
    }
}