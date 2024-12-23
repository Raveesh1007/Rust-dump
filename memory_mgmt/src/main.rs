fn main() {
    stack();
    heap();
   update_string();
   
   let  mut s1 : String = String::from("Hwelllo");
   s1 = takes_ownership(s1);
   println!("{}",s1);

   let mut s3 : String = String::from("borrowing");
   borrow_string(&mut s3);
   println!("{}",s3)
}

fn stack(){
    let a : i8 = 32;
    let b : i8 = 20;
    let c = a+b;

    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap(){
     let _s1 : String = String::from("Hey");
    let _s2 : String = String::from("there");

   println!("Heap fn: String s1 {} and s2 {}",_s1,_s2);
}

fn update_string(){

    let mut _str : String = String::from("Initial String");
    
    println!("Before adding {}",_str);
    _str.push_str("addtional string");

    println!("after additon{}",_str);

    for _ in 0..10{
        _str.push_str("looping");
        println!("Looping for heap ptr{}, capacity:{}, length:{}, pointer:{:p}",_str,_str.capacity(),_str.len(),_str.as_ptr())
    }
    }
   fn takes_ownership(some_str : String)-> String{
     println!("{}",some_str);
     return some_str;
   }

   fn borrow_string(s : &mut String){
       s.push_str("pushed")
   }