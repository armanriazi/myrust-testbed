
#[derive(Debug,PartialEq)]
pub struct GBP(i32);

/* fn money_pointer(i:i32)-> & GBP{
    let g= GBP(i);
    &g
}
 */

/* fn on_money_pointer(i:i32, b:i32)-> GBP{
    let g= GBP(i);
    let r= &g;
    let res=GBP(r.0+b);
    res
}

*/

/* fn money_pointer(i:i32)-> & GBP{
    let g= GBP(i);
    &g
}
 */

 fn on_money_pointer(a:i32, b:i32)-> GBP{
    let r;
    {   
        let mut g= GBP(a);
        r=&g;
       
    }

    let res=GBP(r.0+b);
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_work(){
       //let g= money_pointer(3);
       //assert_eq!(*g,GBP(3));
       let g= on_money_pointer(3, 4);
       assert_eq!(g,GBP(7));
       
       
    }
}