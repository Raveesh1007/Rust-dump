/*fn main() {
  let vec : Vec<i32> = vec![2,4,1,3];
  let vec_iter = vec.iter();

  for i in vec_iter{
     println!("values:{}",i)
  }
}
*/

/*fn main(){
    let mut vec = vec![1,3,4,5];
    let vec_iter = vec.iter_mut();
    for i in vec_iter{
        *i = *i +1;
        println!("values:{}",i)
    }
}
    */

/*fn main() {
    let vec = vec![1, 3, 8, 6];
    let mut vec_iter = vec.iter();
    while let Some(val) = vec_iter.next() {
        println!("values:{}", val);
    }
    println!("orginal:{:?}", vec)
}
*/
// takes oweneship of the original variable.
/*
fn main() {
    let vec = vec![4, 2, 1, 9];
    let vec_iter = vec.into_iter();
    for i in vec_iter {
        println!("value: {}", i)
    }
}

*/
/*
fn main(){
    let vec = vec![4, 2, 1, 9];
    let vec_iter = vec.iter();
    let vec_iter2 = vec_iter.map(|x| x+1);
    for i in vec_iter2{

        println!("Values are: {}",i)
    }
}
    */

fn main() {
    let vec = vec![4, 2, 1, 7, 2, 9];
    let vec_iter = vec.iter();
    let vec_iter2 = vec_iter.filter(|x| *x % 2 != 0);
    let vec_iter3 = vec_iter2.map(|x| x * 2);
    let new_vec: Vec<i32> = vec_iter3.collect();

    println!("New vec: {:?}", new_vec)
}
