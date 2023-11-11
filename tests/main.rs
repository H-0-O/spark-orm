use std::env;
use ORM::*;


// #[test]
// fn the(){
//     let tas = async {
//         let d = OrmMain::new().await;
//
//         panic!("hiedd {:?} " , d);
//     };
//     let mut tasw = task::spawn(tas);
//
//     let res = tasw.poll().is_ready();
//     panic!("the FF {:?} " , res);
// }

#[test]
fn theard_test(){
    // env::set_var("RUST_BACKTRACE", "full");
    // let start_block = async {
    //     let f = OrmMain::connect("cc" , "fdf" , "df" , "fdf").await;
    //     // println!("the Number is : {:?} " , f.number);
    //     println!("the F is {:?} " ,f );
    //     f.set_db(5);
    //     println!("the Number is : {:?} " , f.number)
    // };
    // let mut the_task = task::spawn(start_block);
    // the_task.poll().is_ready();
    // panic!("Finish Test ");
}

#[tokio::test]
async fn ne_test() {
    env::set_var("RUST_BACKTRACE", "full");
    RoMM::create_global_instance(
        "manager_admin", "admin_root_123", "localhost", "27018", "CRM",
    ).await;

    println!("Finish Test ");
}