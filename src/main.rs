use rand::Rng; //Rng trait defines methods that random number generators implement 
use std::io; //input output library from the std or standard library 
use colored::Colorize;
use ansi_term::Style;

struct Game<'a> {
    name:           &'a String,
    opponent:       &'a String,
    player_number:  i32,
    user_total:     &'a mut i32,
    opponent_total: &'a mut i32,
    set_dice:       i32,
    dice:           Vec<i32>,
    message:        String,
    key_press:      String,
    answer:         String,
    dice_tot:       i32
}

impl<'a> Game<'a> {
    pub fn get_name(    
        name:           &'a String,
        opponent:       &'a String,
        player_number:  i32,
        user_total:     &'a mut i32,
        opponent_total: &'a mut i32,
        set_dice:       i32,
        dice:           Vec<i32>,
        message:        String,
        key_press:      String,
        answer:         String,
        dice_tot:       i32) -> Game<'a> {

        Game {name, 
              opponent, 
              player_number, 
              user_total, 
              opponent_total,
              set_dice,
              dice,
              message,
              key_press,
              answer,
              dice_tot}
    }

    pub fn inner_game_loop(&mut self){
        loop { 
            self.player_turn();

            if *self.user_total <= 0 {
                self.display_player_names();
                println!("{} Won!\n", self.opponent);
                break;
            }
            else if *self.opponent_total <= 0 {
                self.display_player_names();
                println!("{} Won!\n", self.name);
                break;
            }
            self.player_number += 2;
        }
    }
 
    pub fn player_turn<'b>(&mut self){
        let mut _player_name = String::new();
        if self.player_number % 4 == 2 {
            _player_name = self.name.to_string();
        }
        else {
            _player_name = self.opponent.to_string();
        }
        self.set_dice = 0;
        self.dice = vec![0; 6];
        self.dice_tot = 0;
        self.display_player_names();
        println!(" {}'s Turn\n-----------------", _player_name);
        //self.rand_dice(); //roll dice
        //self.display_dice(); //display dice

        //self.sort(); //sort dice smallest to largest
        //self.dice_total(); //total dice and display
        //self.display_dice(); //display dice from largest to smallest

        self.key_press = "y/n".to_string();


        'outer: loop {
            self.rand_dice();
            self.sort();
            self.dice_total(); //total dice and display
            println!("Kept die {}", self.dice[self.set_dice as usize]); //last position
            //self.display_dice(); //display dice from largest to smallest
            if self.set_dice >= 5 {
                break 'outer;
            }
            self.keep_dice(); // self.set_dice = 1
            self.display_dice(); 

            self.message = "Keep die ".to_string() + &self.dice[self.set_dice as usize].to_string() + "? (y/n)"; //second to last position
            while self.process_answer() {
                self.keep_dice(); //self.set_dice = 2
                println!("");
                println!("Kept die {}", self.dice[self.set_dice as usize - 1]); //second to last position
                self.dice_total(); //total dice and display
                if self.set_dice >= 6 {
                    break 'outer;
                }
                self.display_dice(); 
                self.message = "Keep die ".to_string() + &self.dice[self.set_dice as usize].to_string() + "? (y/n)";//third to last position
            }
            println!("");
        }
        self.attack()
    }

    
    pub fn attack(&mut self) {
        let attack_points = self.dice_tot - 30;
        let mut attack_tot = 0;
        
        if attack_points > 0 {
            println!("\n You get to attack!\n--------------------");
            self.set_dice = 0;
            while self.set_dice <= 6 {
                self.rand_dice();
                self.sort();
                self.dice_tot = 0;
                if self.dice.contains(&attack_points){
                    for i in self.set_dice..((self.dice.len() as i32)) {
                        if attack_points == self.dice[i as usize] {
                            attack_tot += attack_points;
                            self.keep_dice();
                        }
                    }
                    println!("Attack Total: {}", attack_tot);
                    self.display_dice();
                }
                else {
                    println!("Attack Total: {}", attack_tot);
                    self.display_dice();
                    self.message = "Press any key to move on".to_string();
                    self.key_press = "Any".to_string();
                    self.process_answer();
                    println!("");
                    break;
                }
               
                self.message = "Press any key to move on".to_string();
                self.key_press = "Any".to_string();
                self.process_answer();
                println!("");
                
            }

            println!("\nYour opponent lost {} points :)", attack_tot);

            if self.player_number % 4 == 2 {
                *self.opponent_total = *self.opponent_total - attack_tot;
            }
            else {
                *self.user_total = *self.user_total - attack_tot;
            }

        }
        else if attack_points == 0 {
            println!("You Broke Even Bro.");
        }
        else {
            if self.player_number % 4 == 2 {
                *self.user_total = *self.user_total + attack_points;
            }
            else {
                *self.opponent_total = *self.opponent_total + attack_points;
            }
            println!("You lost {} points :(", -1 * attack_points);
        }
    }

    pub fn display_player_names(&self){
        println!("\nPlayer {} = {}, Points: {}\nPlayer {} = {}, Points: {}\n",1, Style::new().underline().paint(self.name).black().on_white(), self.user_total, 2, Style::new().underline().paint(self.opponent), self.opponent_total);
    } 

    pub fn rand_dice(&mut self){
        let mut secret_number;
        for i in self.set_dice..((self.dice.len() as i32)) { //
            secret_number = rand::thread_rng() //thread_rng is a function - selects our particular random number gen
                                .gen_range(1..=6); //method  on our rand num gen rng //start..end inclusive on the lower bound (one included) exclusive on the upper bound (101 not included) //we could also 1..=100 which is inclusive
            self.dice[i as usize] = secret_number;
        }
    }

    pub fn sort(&mut self) {
        let mut i;
        let mut j:usize = self.set_dice as usize;
        let mut switch;
        while(j as usize) < self.dice.len() {
            i = self.set_dice as usize;
            while (i as usize) < self.dice.len() {
                if i != (self.dice.len() - 1) && self.dice[i] < self.dice[i+1] {
                    switch = self.dice[i+1];
                    self.dice[i + 1] = self.dice[i];
                    self.dice[i] = switch;
                }
                i = i + 1;
            }
            j = j + 1;
        }
    }

    pub fn dice_total(&mut self) {

        self.dice_tot = 0;

        for die in self.dice.iter() {
            self.dice_tot += die;
        }
        println!("Dice total: {}", self.dice_tot);

    }

    pub fn display_dice(&mut self) {
        println!("{:?}", &self.dice[self.set_dice as usize..self.dice.len() as usize]); //very rude.
        /*print!("[");
        let mut i = self.set_dice as usize;

        while i <= self.dice.len() - (1 as usize) {
            if i == self.dice.len() - (1 as usize) {
                println!("{}]", self.dice[i as usize]);
                break;
            }
            //if i == self.set_dice as usize 
            print!("{}, ", self.dice[i as usize]);
            i += 1;
        }*/
    }

    pub fn user_input(&self, answer: &mut String) -> String { //keepDice?
        println!("{}", self.message);
        io::stdin().read_line(answer).expect("Failed to read line");
        //answer = answer.to_lowercase().trim().to_string();
        return answer.to_lowercase().trim().to_string();
    }

    pub fn process_answer(&mut self) -> bool{
        let mut answer_user = String::new();
        self.answer = self.user_input(&mut answer_user);
        if self.answer == "n" && self.key_press != "Any" {
            return false;
        }
        else if self.key_press == "Any" || self.answer == "y" {
            return true;
        }
        else {
            println!("Enter y/n");
            return false;
        }
    }

    pub fn keep_dice(&mut self) {
        self.set_dice += 1;
    } //just don't display the first stuff which will make counting points so much easier
}

fn main() {
    //'Global Variables' lol
    //initialize all variables
    let name = String::new();
    let opponent = String::new();
    let mut user_total = 30;
    let mut opponent_total = 30;
    let set_dice = 0;
    let dice = vec![0; 6];
    let message = String::new();
    let key_press = String::new();
    let answer = String::new();
    let dice_tot = 0;

    let mut player_one = String::new();
    let mut player_two = String::new();

    // define player_one struct
    let mut game = Game::get_name(
         &name,
         &opponent,
         2, 
         &mut user_total, 
         &mut opponent_total,  
         set_dice,
         dice, 
         message,
         key_press, 
         answer,
         dice_tot);

    //User Names
    game.message = "Please input your name:".to_string();
    player_one = game.user_input(&mut player_one);
    player_one = player_one.to_uppercase();
    player_two = game.user_input(&mut player_two);
    player_two = player_two.to_uppercase();

    game.name = &player_one;
    game.opponent = &player_two;


    //Game Play
    game.inner_game_loop();

}
/* Deletes array at request
struct Game<'a> {
    name: &'a String,
    opponent: &'a String,
    player_number: i32,
    user_total: &'a i32,
    opponent_total: &'a i32
}

impl<'a> Game<'a> {
    pub fn get_name(name: &'a String, opponent: &'a String, player_number: i32, user_total: &'a i32, opponent_total: &'a i32) -> Game<'a> {
        Game {name, opponent, player_number, user_total, opponent_total}
    }

    pub fn display(&self){
        println!("\nPlayer {} = {}, Points: {}\nPlayer {} = {}, Points: {}\n",self.player_number, self.name, self.user_total, 2, self.opponent, self.opponent_total);
    } 

    pub fn rand_dice(&self, dice: &mut Vec<i32>){
        let mut secret_number;
        for i in dice.iter_mut() {
            secret_number = rand::thread_rng() //thread_rng is a function - selects our particular random number gen
                                .gen_range(1..=6); //method  on our rand num gen rng //start..end inclusive on the lower bound (one included) exclusive on the upper bound (101 not included) //we could also 1..=100 which is inclusive
            *i = secret_number;
        }
    }

    pub fn sort(&self, dice: &mut Vec<i32>) {
        let mut i;
        let mut j:usize = 0;
        let mut switch;
        while(j as usize) < dice.len() {
            i = 0;
            while (i as usize) < dice.len() {
                if i != (dice.len() - 1) && dice[i] > dice[i+1] {
                    switch = dice[i+1];
                    dice[i + 1] = dice[i];
                    dice[i] = switch;
                }
                i = i + 1;
            }
            j = j + 1;
        }
    }

    pub fn display_dice(&self, dice_tot: &i32, dice: &Vec<i32>) {
        print!("[");
        let mut i = dice.len() - 1;

        while i >= 0 {
            if i == 0 {
                println!("{}]", dice[i as usize]);
                break;
            }
            print!("{}, ", dice[i as usize]);
            i -= 1;
        }
    }

    pub fn user_input(&self, message: &String, key_press: &String) -> bool { //keepDice?
        let mut answer = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        answer = answer.to_lowercase().trim().to_string();
        if answer == "n" && key_press != "Any" {
            return false;
        }
        else if key_press == "Any" || answer == "y" {
            return true;
        }
        else {
            println!("Enter y/n");
            return false;
        }
    }

    pub fn keep_dice(&self, dice_tot: &mut i32, dice: &mut Vec<i32>){
        dice.pop();
    } //just don't display the first stuff which will make counting points so much easier
}

fn main() {
    //'Global Variables' lol
    let mut player_one_name = String::new();
    let mut player_two_name = String::new();
    let mut user_total_one = 30;
    let mut user_total_two = 30;


    //Get User Names
    println!("Please input your name:");
    io::stdin().read_line(&mut player_one_name).expect("Failed to read line");
    //Remove newline
    player_one_name = player_one_name.trim().to_string();

    println!("Please input your name:");
    io::stdin().read_line(&mut player_two_name).expect("Failed to read line");
    player_two_name = player_two_name.trim().to_string();


    // define player_one struct
    let mut game = Game::get_name(&player_one_name, &player_two_name, 1, &user_total_one, &user_total_two);
    
    let mut message = String::new();
    let mut key_press = String::new();
    let mut dice_tot = 6;
    //Game Play
    //loop {

        game.display();
        println!("Player One's Turn\n-----------------");
        let mut dice = vec![0; 7];
        game.rand_dice(&mut dice);
        game.display_dice(&dice_tot, &dice);

        game.sort(&mut dice);
        
        game.display_dice(&dice_tot, &dice);

        message = "Keep dice ".to_string() + &dice[dice.len() - 1].to_string() + "?";
        key_press = "y/n".to_string();

        
        while game.user_input(&message, &key_press) {
            game.keep_dice(&mut dice_tot, &mut dice);
            message = "Keep die ".to_string() + &dice[dice.len() - 1].to_string() + "? (y/n)";
            game.display_dice(&dice_tot, &dice); 
        }

        if user_total_one == 0 || user_total_two == 0 {
            //break;
        }
    //}

}*/

/* Sources
 * https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4547762e00f785f3abcfdd8b44a5d077
 * https://programming-idioms.org/idiom/226/delete-last-element-from-list/4074/rust
 * https://www.codegrepper.com/code-examples/rust/convert+i32+to+usize+rust
 * https://stackoverflow.com/questions/39785597/how-do-i-get-a-slice-of-a-vect-in-rust
 * https://www.joshmcguigan.com/blog/array-initialization-rust/
 * https://stackoverflow.com/questions/23920968/why-does-the-binary-operator-not-work-with-two-mut-int
 * https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
 * https://whimsical.com/rust-30s-game-UPAFCWM4Kqu7D7n9Q6DkZq
 * https://www.tutorialspoint.com/rust/rust_slices.htm
 * https://users.rust-lang.org/t/why-cant-i-increment-a-variable-like-this/18287/2
 * https://docs.rs/crate/ansi_term/0.6.3
 * https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector
*/

/* Struct with lifetimes and kept ownership!
use std::io; //input output library from the std or standard library 

struct  Player<'a> {
    name: &'a String,
    opponent: &'a String,
    player_number: i32,
    user_total: &'a i32
}

impl<'a> Player<'a> {
    pub fn get_name(name: &'a String, opponent: &'a String, player_number: i32, user_total: &'a i32) -> Player<'a> {
        Player {name, opponent, player_number, user_total}
    }

    pub fn display(&self){
        if self.player_number == 1{
            println!("Player {} = {} Player {} = {}",self.player_number, self.name, 2, self.opponent);
        }
    } 
}

fn main() {
    let mut player_one_name = String::new();
    let mut player_two_name = String::new();
    let mut user_total_one = 30;
    let mut user_total_two = 30;


    println!("Please input your name:");
    io::stdin().read_line(&mut player_one_name).expect("Failed to read line");
    player_one_name = player_one_name.trim().to_string();

    println!("Please input your name:");
    io::stdin().read_line(&mut player_two_name).expect("Failed to read line");
    player_two_name = player_two_name.trim().to_string();


    let mut player_one = Player::get_name(&player_one_name, &player_two_name, 1, &user_total_one);
    player_one.display();
    println!("Player {} = {} Player {} = {}",1, player_one_name, 2, player_two_name);

}*/

/* Example of a struct
struct Point {
    x: i32,
    y: i32,
 }
 impl Point {
    //static method that creates objects of the Point structure
    fn get_instance(x: i32, y: i32) -> Point {
       Point {x, y}
    }
    //display values of the structure's field
    fn display(&self){
       println!("x ={} y={}",self.x,self.y );
    }
 }
 fn main(){
    // Invoke the static method
    let p1 = Point::get_instance(10,20);
    p1.display();
 }*/

/* Command Line Commands
 * cd..
 * ls
 * cargo new file_name
 * cargo build - builds
 * cargo run - builds and runs
 * cargo check - check for errors
*/