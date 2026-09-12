use std::fs::read_to_string;

fn eval(ast:&Vec<Ast>) {
 let len=ast.len();
 let mut i=0;
 while i<len{
  println!("{} {:?}",i,ast[i]);
  i+=1;
 }
 return;
}

#[derive(Debug,Clone)]
enum Ast {
 Str(String),
 Int(i32),
 Name(String),
 List(Vec<Ast>),
}

fn parse_from_tokens(tokens:&Vec<Token>)->Vec<Ast> {
 let len=tokens.len();
 let mut i=0;
 let mut res=Vec::<Ast>::with_capacity(16);
 while i<len {
  match tokens[i] {
   Token::Str(ref s)=>res.push(Ast::Str((*s.clone()).to_string())),
   Token::Int(i)=>res.push(Ast::Int(i)),
   Token::Name(ref s)=>res.push(Ast::Name((*s.clone()).to_string())),
   Token::Paren(c)=>{
    if c=='(' {
     i+=1;
     let mut b=1;
     let mut a=Vec::<Token>::with_capacity(16);
     while i<len {
      match tokens[i] {
       Token::Paren(c)=>{
        if c=='(' { b+=1; }
        else if c==')' { b-=1; }
        if b==0 { break; }
        a.push(tokens[i].clone());
       },
       _=>a.push(tokens[i].clone()),
      }
      i+=1;
     }
     res.push(Ast::List(parse_from_tokens(&a).clone()));
    } else {
     panic!("Unexpected ')'");
    }
   },
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

#[derive(Debug,Clone)]
enum Token {
 Str(String),
 Int(i32),
 Name(String),
 Paren(char),
}

fn lex(code:&String)->Vec<Token> {
 let chars=code.chars().collect::<Vec<char>>();
 let len=code.len();
 let mut i=0;
 let mut buf=String::with_capacity(16);
 let mut res=Vec::<Token>::with_capacity(16);
 let paren=String::from("()");
 while i<len {
  if chars[i]=='"' {
   i+=1;
   let mut c=1;
   while i<len {
    if chars[i]=='"' { c-=1; }
    if c==0 { break; }
    buf.push(chars[i]);
    i+=1;
   }
   res.push(Token::Str(buf.clone()));
   buf.clear();
  } else if chars[i].is_ascii_digit() {
   while i<len && chars[i].is_ascii_digit() {
    buf.push(chars[i]);
    i+=1;
   }
   res.push(Token::Int(buf.parse::<i32>().unwrap()));
   buf.clear();
   i-=1;
  } else if paren.find(chars[i]).is_some() {
   res.push(Token::Paren(chars[i]));
  } else if !chars[i].is_ascii_whitespace() {
   while i<len && !chars[i].is_ascii_whitespace() && paren.find(chars[i]).is_none() {
    buf.push(chars[i]);
    i+=1;
   }
   res.push(Token::Name(buf.clone()));
   buf.clear();
   i-=1;
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn main() {
 let code=read_to_string("main.cbat").unwrap();
 let tokens=lex(&code);
 let ast=parse_from_tokens(&tokens);
 eval(&ast);
 return;
}
