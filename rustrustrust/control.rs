//// ;;;; ;;;; ;;;; ;;;;
// Title: Contorl flow 
//// ;;;; ;;;; ;;;; ;;;;
fn main()
{
    if_flow();
    if_let_flow(false);
    loop_flow();
    loop_let_flow(20);
    tag_mulloop_flow();
    while_flow();
    while_array_flow();
    for_flow();
    for_array_flow();


}

// ;;;; ;;;; ;;;; ;;;;
// if
// ;;;; ;;;; ;;;; ;;;;

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

fn if_let_flow(condition:bool)
{
    // if and else type must same. else err!
    let rust = if condition {"A"} else {"B"};
    println!{"{rust}"};
}

// ;;;; ;;;; ;;;; ;;;; ;;
// for . loop . while 
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

fn loop_let_flow(x: i32)
{
    let mut poc = 0;
    println!{"you setting: {}", x};
    let rust = loop
    {
        poc += 1;
        if poc == 50
        {
            poc = poc * 2 - 1;
        }
        if poc == 100
        {
            break poc * 2 * x     //  result value
        }
        if poc == 20
        {
            for rust in 1..10
            {
                print!{"*{rust}"};
            }
        }
    };
    println!("\nThe result is {rust}");
}

fn tag_mulloop_flow()
{
    let mut poc = 1;
    'fuck:loop
    {
        println!("poc = {poc}");
        let mut rust = 10;
        loop
        {
            println!("rust = {rust}");
            if rust == 9 {break;}
            if poc == 2 {break 'fuck;}
            rust -= 1;
        }
        poc += 1;
    }
    println!("end poc: {poc}");
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

fn while_array_flow()
{
    let rustc = [ 30, 20, 50, 100, 200];
    let mut i = 0;
    while i < 5
    {
        print!("{}",rustc[i]);
        if i!=4 {print!(" ");}else{println!();break;} 
        i += 1;
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

