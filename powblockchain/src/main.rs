use std::io;
use std::process::exit;
use sha256::digest;
use std::collections::HashMap;
use serde::de::Unexpected::Str;
//use serde_json::Value::String;

mod mychain;

use mychain::{Chain,Wallet,Accounts};
// adding 
// pow 
// 

fn main(){



    let mut chain:Chain;

    let mut acc = Accounts{wallets:HashMap::new()};


    println!("Create your wallet first !");

     println!("Enter your name:");
                  let mut name = String::new();
                  io::stdin().read_line(&mut name);

                  println!("Enter your wallet password:");
                  let mut  pass = String::new();
                  io::stdin().read_line(&mut pass);
                  let wallet = Wallet::create_wallet(name.clone(),pass);

                  acc.push(name,wallet);
                  acc.print_accounts();
 
    println!("enter the miner name")   ;
    let mut miner_name = String::new();
    let mut miner_addr = String::new();


    io::stdin().read_line(&mut miner_name);
    let mut  flag = 0;
    miner_addr = match acc.wallets.get_key_value(&miner_name)
    {
           Some((a,b)) => {
               let w = b.clone();
               flag = 1;
               w.pubaddr.to_string()

           }
           None => {
               println!("wallet address not found ");
               exit(0)


           }


    };




    println!("enter the difficulty")   ;
    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty);
    
    let parsed_difficulty:u32 = difficulty.trim().parse().unwrap();

   println!("generating genesis block! 
   ");

   // instance of chain 
   // genesis block 
   // miner addd 
   // diffi
   // Vec transactions 
   // merkle 
   // time_stamp
   // nonce
   //



        chain= Chain::new(miner_addr.as_str(), parsed_difficulty, 100.0);

  loop{
      println!("Menu");
      println!("1-Create Wallet");
      println!("6-Show Accounts");
      println!("2-New Transaction");
      println!("3-Mine Block");
      println!("4-Change difficulty");
      println!("5-Change Reward");
      println!("0-Exit");
 
    let mut choice = String::new();
    io::stdin().read_line(&mut choice); 
 
    // 
    let parsed_choice:u32 = choice.trim().parse().unwrap() ;
    match parsed_choice {
              1 => {
                  println!("Enter your name:");
                  let mut name = String::new();
                  io::stdin().read_line(&mut name);
                  println!("Enter your wallet password:");
                  let mut  pass = String::new();
                  io::stdin().read_line(&mut pass);
                  let wallet = Wallet::create_wallet(name.clone(),pass);
                  acc.push(name.clone(),wallet);
                  acc.print_accounts();



              }
 
               2 => {
                   let mut sender = String::new();
                   let mut rec = String::new();
                   let mut amount = String::new();

                   // add a transaction\
                   println!("enter the sender's name:");

                   io::stdin().read_line(&mut sender);

                   println!("enter the receiver's name:");
                   io::stdin().read_line(&mut rec);

                   println!("enter the  amount :");
                   // let a  = convert_to_ref(sender);
                   io::stdin().read_line(&mut amount);
                   // dangle
                   println!("Enter your password to make the transaction:");
                   let mut pass:String = String::new();
                   io::stdin().read_line(&mut pass);
                   let hash_pass:String = digest(pass);


                   let parse_amount: f64 = amount.trim().parse().unwrap();
                   let p:f32 =amount.trim().parse().unwrap();
                  let mut f=0;
                   let mut saddr :String = String::new();
                   let mut  raddr:String = String::new();

                  if acc.wallets.contains_key(&sender)
                  {
                      if acc.wallets.contains_key(&rec)
                      {
                          let sendw  = acc.wallets.get_mut(&sender).unwrap();
                          if sendw.pass_hash==hash_pass
                          {
                              if sendw.bal_validate(parse_amount) == 1
                              {
                                  f = 1;

                                  sendw.sub_bal(parse_amount);
                                  saddr = sendw.pubaddr.to_string();
                              }
                              else { println!("Insufficient Balance!"); }
                          }
                          else { println!("Wrong Password!"); }
                      }
                      else {  println!("Receiver wallet not found!");
                       continue; }



                  }
                   else{
                       println!("Sender wallet not found!");
                       continue;
                   }

                   if f==1
                   {
                       let recw = acc.wallets.get_mut(&rec).unwrap();
                       raddr = recw.pubaddr.to_string();
                       recw.add_bal(parse_amount);
                       let res = chain.add_transaction(saddr, raddr, p);
                       match res {
                           Ok(_) => {
                               println!("Transaction is added");
                           },
                           Err(_) => {
                               println!("Transaction is reverted");
                           }
                       }
                   }
                   else { println!("Transaction is reverted"); }






               },

   

 
               3=>{
 
                 // mine a block => generating block
                 println!("generating a block"); 
                 let res = chain.generate_new_block();
 
 
              match res {
 
                     Ok( _) =>{
                         println!("block is added");
                     },
                     Err(_) =>{
                         println!("block is reverted");
                     }
                 }

               },
               4=>{
                 // change difficulty
                   println!("enter the new difficulty:") ;
                 let mut new_difficulty = String::new() ;
                 // method 
 
                 io::stdin().read_line(&mut new_difficulty );
                    let parsed_new_difficulty:u32 = new_difficulty.trim().parse().unwrap();
 
                     let res = chain.change_diff(parsed_new_difficulty);
 
                     match res {
 
                         Ok( _) =>{
                             println!("difficulty changed");
                         },
                         Err(_) =>{
                             println!(" there is a issue changing difficulty");
                         }
                     }
 
               },
               5=> {
                   println!("enter the new reward:") ;
                 let mut new_reward = String::new() ;
                 io::stdin().read_line(&mut new_reward );
                 let parsed_new_reward:f32 = new_reward.trim().parse().unwrap();
 
                  let res = chain.change_reward(parsed_new_reward);
 
                  match res {
 
                      Ok( _) =>{
                          println!("Reward changed");
                      },
                      Err(_) =>{
                          println!(" there is a issue changing Reward");
                      }
                  }
                 // change reward
               }
               0 => {
                 // exit 
 
                 println!("exiting ..... ");
                 exit(0);
               }
              6 => {acc.show_accounts();},

               _ => {
                 // exit

                 println!("exiting ..... ");
                 exit(0);
               }
 
    }
 

  }
  


    



}


// fn add_trans(c:&mut Chain) {

//     let mut sender  = String::new() ;
//     let mut  rec =  String::new() ;
//     let mut amount = String::new();

//     io::stdin().read_line(&mut sender);
 
       
//     io::stdin().read_line(&mut rec);


//    // let a  = convert_to_ref(sender);
//     io::stdin().read_line(&mut amount);
//       // dangle 
//     let parse_amount:f32 = amount.trim().parse().unwrap();

//     let res = c.add_transaction("a", rec.as_str(), parse_amount);

//     match res {

//         Ok( _) =>{
//             println!("Transaction is added");
//         },
//         Err(_) =>{
//             println!("Transaction is reverted");
//         }
//     }
// }


// fn convert_to_ref<'a>(s:String) -> &'a str {
// // s  
//     s.as_str() // dangle 
// }// 

// fn get_sender() -> &str {

// }


