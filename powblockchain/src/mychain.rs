use rand::thread_rng;
use serde_json::to_string;
use sha256::digest;
// use rsa::{PublicKey,RsaPrivateKey,RsaPublicKey,PaddingScheme};
use chrono::prelude::*; 
use sha2::{Digest,Sha256};
use std::fmt::{Debug, Pointer, Write};
use std::fmt::Display;
use solana_sdk::signer::keypair::Keypair;
use std::collections::HashMap;


use serde_derive::{Serialize,Deserialize};
/*

block ? 
private
Block{
header:BlockHeader
transaction:vec<Transaction>
count:u32 -> Transaction length 
}

BlockHeader{
    time_stamp:i64,
    nonce:u64, 
    prev_hash:String, 
    merkle_hash:String, 
    difficulty:u32
}

Transaction{
    sender:&str,
    reciever:&str ,
    amount:f32
}

public 
1->2->3->4  -> Chain 
struct Chain{
    chain:Vec<Block>,
    miner_addr:&str,
    reward:u32,
    difficulty:u32,
    curr_trans:Vec<Transaction>
}


*/

use std::vec;
use rsa::pkcs8::der::Encode;
// use rsa::pkcs1::LineEnding;
// use rsa::pkcs8::{EncodePublicKey, SecretDocument};
use serde::de::Unexpected::Str;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;

#[derive(Debug)]
struct Block{
    header:BlockHeader,
    transaction:Vec<Transaction>,
    count:u32 
  }
#[derive(Debug)]
pub enum Token{
    SOL,
    ETH,
    BTC
}
#[derive(Debug)]
pub struct Wallet{
    name:String,
    pub pubaddr:String,
    pvtaddr:[u8;32],
    pub balance:f64,
    pub pass_hash:String,
    pub token:Token
}

impl Wallet {
    pub fn create_wallet(name:String,pass:String)->Self
    {
        let bits = 2048;
        let mut rand = thread_rng();
        let kpair = Keypair::new();
        let private_key= kpair.secret().to_bytes();
        let public_key=kpair.pubkey().to_string();
        // let pubkey:String = public_key.clone()
        // let prvkey:String = private_key.
        let hashed_pass:String = digest(pass);
        Self {name,pubaddr:public_key,pvtaddr:private_key,balance:100_f64,pass_hash:hashed_pass,token:Token::SOL}



    }
    pub fn bal_validate(&self,amount:f64)->i32
    {
        if self.balance<amount
        {
            0
        }
        else { 1 }
    }

    pub fn add_bal(&mut self,amount:f64)
    {
        self.balance = self.balance+amount;
    }
     pub fn sub_bal(&mut self,amount:f64)
    {
        self.balance = self.balance-amount;
    }

}

#[derive(Debug)]
pub struct Accounts{
    pub wallets:HashMap<String,Wallet>
}
impl Accounts{
    pub fn push(&mut self,name:String,w:Wallet)
    {
     self.wallets.insert(name,w)  ;
    }
    pub fn print_accounts(&self)
    {
        println!("Accounts :");
        println!("{{");
        for (k,i) in self.wallets.iter()
        {
            println!("{{");
            println!("name:{}",i.name.trim());
            println!("public address :{}",i.pubaddr);
            println!("balance:{:?}",i.balance);
            println!("Token:{:?}",i.token);
            println!("}}");
            println!("\n");

        }
        println!("}}");
    }

    pub fn show_accounts(&self)
    {
        println!("Accounts :");
        println!("{{");
       for (i,j) in self.wallets.iter()
       {   println!("{{");
           println!("name:{}",i.trim());
           println!("public address:{}",j.pubaddr);
           println!("Balance:{}",j.balance);
           println!("Token:{:?}",j.token);
           println!("}}");
           println!("\n");
       }
        println!("}}");

    }

}

  
  #[derive(Debug,Serialize,Deserialize)]
 struct    BlockHeader{
        time_stamp:i64,
        nonce:u64, 
        prev_hash:String, 
        merkle_hash:String, 
        difficulty:u32
    }


 #[derive(Debug,Serialize,Deserialize,Clone)]
    struct  Transaction{
        sender:String,
        reciever:String ,
        amount:f32
}
#[derive(Debug)]
pub struct Chain<'a>{
    chain:Vec<Block>,
    miner_addr:&'a str,
    reward:f32,
    difficulty:u32,
    curr_trans:Vec<Transaction>
}

// generating block  -> method 


impl<'a>Chain<'a> {


 pub fn new(miner_addr:&'a str,difficulty:u32,reward:f32)-> Self{
        let m = miner_addr.trim();
        let mut chain = Chain{

            chain:vec![],
            miner_addr:m,
            reward,
            difficulty,
            curr_trans:vec![]
        }; 
     

        // generate_new_block //
        chain.generate_new_block(); // genesis block 
        chain 

    }


 pub fn add_transaction(&mut self,sender:String,rec:String,amount:f32) -> Result<(),String> {
 
// 
/*
    struct  Transaction<'a>{
        sender:&'a str,
        reciever:&'a str ,
        amount:f32
} 

*/

let trans = Transaction{

      sender:sender.trim().to_owned(),
      reciever:rec.trim().to_owned(),
      amount
};

self.curr_trans.push(trans);


Ok(())

 }



  pub   fn generate_new_block(&mut self) -> Result<(),String>{

        //
        // ? 
        // insatance of block 
        /*
        struct Block<'a>{
    header:BlockHeader,
    transaction:Vec<Transaction<'a>>,
    count:u32 
  }
        */
// chrono 
let header = BlockHeader { 
    time_stamp:Utc::now().timestamp(),
    nonce:0,
     prev_hash:self.get_prev_hash(), 
     merkle_hash:String::new(),
      difficulty:self.difficulty
    };
 // hash -> prev_block 
    // miner -> reward 
    // 

    let reward_trans = Transaction{
        sender:String::from("Root"),
        reciever:self.miner_addr.trim().to_owned(),
        amount:100.0

    };

   

        let mut block = Block{
        header,
        transaction:vec![],
        count:0,
        };
   block.transaction.push(reward_trans);
   block.transaction.append(&mut self.curr_trans)  ;
   block.count = block.transaction.len() as u32 ;
block.header.merkle_hash = Self::get_merkle(block.transaction.clone());

//how to get the prev hash ?
// how to get the merkle ?
// 
/*

struct Block<'a>{
    header:BlockHeader,
    transaction:Vec<Transaction<'a>>,
    count:u32 
  }

*/
  Self::proof_of_work(&mut  block.header)  ;

     println!("{:#?}",block);

     // pow 
     self.chain.push(block);

// pow ? 

Ok(())
        
    }


//?? 
// Transactions
// 

fn proof_of_work(header:&mut BlockHeader){


    //hash ->
    // block_num 
    // nonce -> 
   // 004938yr347378r626395632758235
    // hash -> 2 -> 00 
    //  000000000000
    //slices 

    loop {
        let hash = Self::Hash(header);
        let slice = &hash[..header.difficulty as usize]; // &str "00"

        let parsed_slice = slice.parse::<u32>();
    
        match parsed_slice {
            Ok(val) =>{
                     
                if val != 0{
                    //
                    header.nonce+=1;
    
                }else {
                       
                    println!("{:?}",hash);
                    break; 
    
                }
                //
            },
            Err(_) =>{
                     header.nonce+=1;
                     continue;
            }
        };
    
    }
   



}

// method 

pub fn change_diff(&mut self,new_diff:u32)->Result<(),String>
{

self.difficulty = new_diff;


Ok(())
} 


pub fn change_reward(&mut self,new_reward:f32)->Result<(),String>
{

self.reward = new_reward;


Ok(())
} 


fn get_merkle(curr_trans:Vec<Transaction>) -> String {
  let mut  merkle = Vec::new();

  for t in &curr_trans {
        let hash = Self::Hash(t);
        merkle.push(hash)
  }
  // binary tree -> 1,3,5
  // merkle -> 
  if merkle.len()%2==1 {
     let last = merkle.last().cloned().unwrap();
     merkle.push(last);
  }

  // loop 
  // while loop 
  // 1 
  while merkle.len() >1 {
  //how to make pairs 
  // remove 
  // ["a","b","c","d"]
  let mut  h1 = merkle.remove(0); // ["b","c","d"]
  let mut  h2 = merkle.remove(0);  // ["c","d"]
 // combine 
 h1.push_str(&mut h2); //"ab"
 // string of two hashes 
 // hash 
 let combined_hash = Self::Hash(&h1);
 merkle.push(combined_hash)



  }


   // 1 hash
   merkle.pop().unwrap()

}

    fn  get_prev_hash(&self) -> String{

        // prev_block 
        /*
          let v = Vec::new();
          v.push(3);
          v.push(4);

          v.last() 
        
        */
        let prev_block = match  self.chain.last(){
              // is there is a block 
            Some(block )=> {
                block
            },
            // if no block => genesis block 
            None =>  return String::from("0000000000000000000000000000000000000000000000000000000000000000"),

        };
     
  // generate the hash 


    
  //  header -> indentifier -> hash 
  //hash ->  header 
    Self::Hash(&prev_block.header)
 }
// copy clone -> expensive 
/*
  BlockHeader -> T 

  T:Serialize
*/
 fn Hash<T:serde::ser::Serialize>(item:&T) -> String{
   
    // blockheader -> Transaction 
    // to string 
    //serde 
    // json -> serde_json
    // derive -> serde_derive 
    let item_string = serde_json::to_string(&item).unwrap();
    // hash -> 
    // how to hash ?? sha2

    /*
    trait cal
    {
     fn calculate(&self)->Self;

    }
    impl Math for square
    {
    fn calculate_perimeter(&self)->i32
    {
      self.side*4

    }
    impl Math for triangle
    {
    fn calculate_perimeter(&self)->i32
    {
      self.side_a+self.side_b+self.side_c+self.side_d

    }

    }
    / create a Sha256 object
let mut hasher = Sha256::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let result = hasher.finalize();
    
    */
    let mut hasher = Sha256::new();
    hasher.update(item_string.as_bytes());
    let result = hasher.finalize();
    // result hash Sha256  -> string 
      let res_vec = result.to_vec();
     // let mut v1:Vec<u8> = match res_vec
     // {
     //     Ok(v)=>v,
     //     _ =>Vec::<u8>::new()
     //
     // };
    // write!  -> Vec<u8>
     // res_vec -> string -> write!


    Self::hex_to_string(res_vec)
 }

 fn hex_to_string(item:Vec<u8>) ->String {


    let mut s= String::new();
    for b in item {
        write!(&mut s,"{:x}",b).expect("unable to convert");
    }
    return s
 }

  // merkle -> hashing Transaction
  //  

}



