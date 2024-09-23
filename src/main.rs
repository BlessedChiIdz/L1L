
use std::{io::{self, stdin, BufRead, Read}, sync::BarrierWaitResult};
use rand::Rng;
use std::string::String;
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[allow(non_snake_case)]
fn main(){
    let mut rng = rand::thread_rng();
    let mut grammar = String::new();
    println!("Моя граматика(1), ваша(2) ");
    io::stdin().read_line(&mut grammar).expect("F");
    let mut maxLn = String::new();
    println!("MaxLn: ");
    io::stdin().read_line(&mut maxLn).expect("F");
    let maxLn:i32 = maxLn.trim().parse().expect("convErr");
    let mut minLn = String::new();
    println!("MinLn");
    io::stdin().read_line(&mut minLn).expect("F");
    let minLn:i32 = minLn.trim().parse().expect("errConvMl");
    let grInt: i32 = grammar.trim().parse().expect("convErr");
    if grInt == 1{
        let mRl: Vec<&str> = vec!["aA","bB"];
        let rls:Vec<Vec<&str>> = vec![
            vec!["A", "aA", "b"],
            vec!["B", "aA", "b"]
        ];
        let mut res: String = String::new();
        let rand: i16 = rng.gen_range(0..mRl.len().try_into().unwrap());
        mainRuleF(&mut res, &rand, &mRl);
        pRes(res, rls, maxLn, minLn);
    }else
    if grInt == 2{
        let mut sttSim = String::new();
        sttSim.push('S');
        // println!("startSim: ");
        // io::stdin().read_line(&mut sttSim).expect("F");
        println!("MainRule");
        let mut mRl:Vec<String> = Vec::new();
        for _ in 0..2{
            let mut mRI = String::new();
            io::stdin().read_line(&mut mRI).expect("F");
            mRI.pop();
            mRI.pop();
            mRl.push(mRI);
        }
       // println!("{:#?}",mRl);
        println!("count of rules");
        let mut cR = String::new();
        io::stdin().read_line(&mut cR).expect("F");
        let cRI:i16 = cR.trim().parse().expect("F");
        let mut rls:Vec<Vec<String>> = Vec::new();
        for _ in 0..cRI{
            let mut v:Vec<String> = Vec::new();
            let mut i1 = String::new();
            io::stdin().read_line(&mut i1).expect("F");
            i1.pop();
            i1.pop();
            v.push(i1);
            let mut i2 = String::new();
            io::stdin().read_line(&mut i2).expect("F");
            i2.pop();
            i2.pop();
            v.push(i2);
            let mut i3 = String::new();
            io::stdin().read_line(&mut i3).expect("F");
            i3.pop();
            i3.pop();
            v.push(i3);
            rls.push(v);
        }
        let mut res: String = String::new();
        let rlsConv: Vec<Vec<&str>> = rls.iter().map(|v| v.iter().map(|s| s.as_str()).collect()).collect();
        let rand: i16 = rng.gen_range(0..mRl.len().try_into().unwrap());
        let mRlConv = mRl.iter().map(|s| s.as_str()).collect();
        mainRuleF(&mut res, &rand, &mRlConv);
        pRes(res,rlsConv,maxLn,minLn);
    }else{
        panic!("ne to");
    }
}

#[allow(non_snake_case)]
fn mainRuleF(res:&mut String, rand:&i16, mRule:&Vec<&str>){
    res.push_str(mRule[*rand as usize]);
}

#[allow(non_snake_case)]
fn pRes(mut res:String, rls:Vec<Vec<&str>>, maxL:i32, minL:i32) {
    let mut rng = rand::thread_rng();
    // println!("res: {}",res);
    // println!("rls: {:#?}",rls);
    // println!("maxL: {}",maxL);
    // println!("minL: {}",minL);
    'mainL: loop {
        //println!("res: {:?}",res);
        if res.len() < minL.try_into().unwrap() {
            //println!("1");
            'outer1: for c in res.chars() {
                for i in 0..rls.len() {
                    let rule = rls[i][0].chars().next().unwrap();
                    if  rule == c {
                        //println!("Matched1: {}", c);
                        res = res.replace(c, rls[i][1]);
                        break 'outer1;
                    }
                }
            }
        }else
        if res.len() == (maxL-1).try_into().unwrap() {
           // println!("2");
             for c in res.chars() {
                for i in 0..rls.len() {
                    let rule = rls[i][0].chars().next().unwrap();
                    if  rule == c {
                        //println!("Matched2: {}", c);
                        res = res.replace(c, rls[i][2]);
                        break 'mainL;
                    }
                }
            }
        }else
        if res.len() >= minL.try_into().unwrap() && res.len() < (maxL-1).try_into().unwrap() {
            //println!("3");
            let rand: i16 = rng.gen_range(0..2);
            'outer3: for c in res.chars() {
                for i in 0..rls.len() {
                    let rule = rls[i][rand as usize].chars().next().unwrap();
                    if  rule == c {
                        //println!("Matched3: {}", c);
                        res = res.replace(c, rls[i][(rand+1) as usize]);
                        break 'outer3;
                    }
                }
            } 
        }
    }
    println!("res: {:?}",res);
}

