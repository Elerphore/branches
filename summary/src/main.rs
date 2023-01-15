fn main() {
    degree_converter();
    fibonacci();
    song();
}

fn degree_converter() {
    {
        let c: f32 = 25.0;
        let f: f32 = (c * ( 1.8 )) + 32.0;

        println!("{c} celsius is equal to {f} fahrenheit");
    }
    {
        let f: f32 = 77.0;
        let c: f32 = (f - 32.0) * 5.0 / 9.0;

        println!("{c} celsius is equal to {f} fahrenheit");
    }
}

fn fibonacci() {
    let result = fb(9);
    println!("Fibonacci: {result}");
}

fn fb(n: i32) -> i32 {
    if n <= 1 {
        return 9;
    }

    fb(n - 1) + fb(n - 2)
}

fn song() {
    let words = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves and a Partridge in a Pear Tree",
        "Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Seven Swans a Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Eight Maids a Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Nine Ladies Dancing Eight Maids a Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree ",
        "Ten Lords a Leaping Nine Ladies Dancing Eight Maids a Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Eleven Pipers Piping Ten Lords a Leaping Nine Ladies Dancing Eight Maids a Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
        "Twelve Drummers Drumming Eleven Pipers Piping Ten Lords a Leaping Nine Ladies Dancing Eight Maids a Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and a Partridge in a Pear Tree",
    ];

    let days = [
        ("first", 0),
        ("second", 1),
        ("third", 2),
        ("fourth", 3),
        ("fifth", 4),
        ("sixth", 5),
        ("seventh", 6),
        ("eighth", 7),
        ("ninth", 8),
        ("tenth", 9),
        ("eleventh", 10),
        ("twelfth", 11),
    ];

    for day in days {
        let day_number = day.0;
        println!("On the {day_number} day of Christmas");
        println!("{}", words[day.1]);
    }
}