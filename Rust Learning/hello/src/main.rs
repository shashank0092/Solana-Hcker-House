use std::collections::HashMap;
fn main() {
    // let a=10;
    // let b=20;
    
    // println!("THIS IS SOME VALUES->{},{}",a,b);
    // println!("Hello, world!");

    // let singed:i8=-10;
    // let unsinged:u8=10;
    // let float:f32=1.2;
    // let letter="c";
    // let emoji="\u{1F600}";

    // println!("This is some values {},{},{},{},{}->",singed,unsinged,float,letter,emoji);
    
    // let arr:[u8;3]=[1,2,3];
    // let other_arr:[u8;5]=[100;5];
    // println!("index {},length:{}",arr[0],other_arr.len());
    // println!("{:?}",other_arr);

    // let tuple:(u8,bool,f32)=(5,true,2.1);
    // let tuple2=(3,5);
    // println!("first{},second{},third{}",tuple.0,tuple.1,tuple.2);
    // println!("{:?}",tuple2);
    // let (a,b,c)=tuple;
    // println!("first{},second{},third{}",a,b,c);


    // println!("{}",is_even(3));

    // let mut num=10;
    // num=20;
    // println!("THE NUMBER IS->{}",num)

    // let arr=[0,1,2,3];
    // let slice=&arr[1..3];

    // println!("{:?}",slice);
    // let string="hello boi";
    // let slice=&string[..6];
    // println!("{:?}",slice);

    // let n=5;

    // if n>0 {
    //     println!("Greater Than 0");
    // }
    // else if n==0 {
    //     println!("EQUAL TO ZERO");
    // }
    // else{
    //     println!("Less Than Zero");
    // }

    // for i in 0..6{
    //     println!("{}",i);
    // }
    
    // let mut i=5;

    // while i>0{
    //     println!("{}",i);
    //     i=i-1;
    // }

    // let i=5;
    // match i {
    //     0=>println!("0"),
    //     1|2=>println!("1 or 2"),
    //     3|4 =>println!("3,4"),
    //     _=>println!("default")
    // }

    // let name=String::from("Bird");
    // let bird=Bird{name,attack:5};
    // bird.print_name();
    // println!("{}",bird.can_fly());
    // println!("{}",bird.is_animal());
    
    // let a:MyEnum=MyEnum::A;
    // let b:MyEnum=MyEnum::B(10);
    // let c:MyEnum=MyEnum::c { x: 100, y: 200 };

    // println!("{:?}",a);
    // println!("{:?}",b);
    // println!("{:?}",c);

        // let mut vec:Vec<i64> =vec![1,2,3,4,5];
        // println!("{}",vec.len());
        // println!("{}",vec[0]);
        // vec.push(6);
        // vec.remove(0);
        // println!("{:?}",vec);
    
    let mut map=HashMap::new();
    map.insert(0, "hi");
    map.insert(1, "hello");
    map.insert(2, "shukla");

    println!("{:?}",map);
}


// #[derive(Debug)]
// enum MyEnum {
//     A,B(i32),c{x:i32,y:i32}
// }

// struct Bird{
//     name:String,
//     attack:u64
// }

// impl Bird {
//     fn print_name(&self) {
//         println!("{}",self.name);
//     }
// }

// impl Animal for Bird {
//     fn can_fly(&self)->bool {
//         false
//     }

//     fn is_animal(&self)->bool {
//         true
//     }
    
// }

// trait Animal {
//     fn can_fly(&self)->bool;
//     fn is_animal(&self)->bool{
//         true
//     }
// }

// pub fn is_even(num:u8)->bool{
//     let digit:u8=num%2;
//     digit==0
// }