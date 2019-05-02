extern crate rand;
use rand::Rng;

pub mod corpora;

fn parse_args_to_vec(input: &str) -> Vec<&str> {
    let args: Vec<&str> = input.split(",").collect();
    return args;
}

/// Returns a random username from a small list
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_username;
/// let user: String = gen_username();
/// // user => ahmadajmi
/// ```
pub fn gen_username() -> String {
    let user = gen_enum(String::from("devankoshal,jesseddy,ahmadajmi,Karimmove,benefritz,meln1ks,shaneIxD,BryanHorsey"));
    return user;
}

/// Generate a random domain name from a small list of predefined values
///
/// Name values:
/// * some
/// * random
/// * names
/// * we
/// * make
/// * up
/// * for
/// * testing
///
/// Possible Top-Level Domains (TLDs):
/// * de
/// * org
/// * us
/// * com
/// * net
/// * eu
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_domain;
/// let domain = gen_domain();
/// // domain => names.eu
/// ```
pub fn gen_domain() -> String {
    let tld = gen_enum(String::from("de,org,us,com,net,eu"));
    let domain = gen_enum(String::from("some,random,names,we,make,up,for,testing"));
    return format!("{}.{}", &domain, &tld);
}

/// Return a randomly generated e-Mail address. This generator combines the `gen_username` and
/// `gen_domain` generator.
///
/// ## Example
/// ```rust
///use fakedata_generator::gen_email;
///let email: String = gen_email();
/// // email => devankosha@lnames.org
/// ```
pub fn gen_email() -> String {
    let user = gen_username();
    let domain= gen_domain();

    return format!("{}@{}", &user, &domain);
}

/// Return random string from set of specified strings. Specify a comma separated list as argument.
///
/// ## Example
/// ```rust
///use fakedata_generator::gen_enum;
///let word: String = gen_enum("some,random,words".to_string());
/// // word = "random"
/// ```
pub fn gen_enum(input: String) -> String {
    let args = parse_args_to_vec(&input);
    let mut rnd = rand::thread_rng();
    let mut index: usize = 0;
    if args.len()-1 > 0 {
        index = rnd.gen_range(0, args.len() - 1);
    }

    return format!("{}", args[index]); //String::from(args[index]);
}

/// Return random HTTP Method.
/// Possible values:
/// * DELETE
/// * GET
/// * HEAD
/// * OPTION
/// * PATCH
/// * POST
/// * PUT
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_http_method;
/// let method: String = gen_http_method();
/// // method = "GET"
/// ```
pub fn gen_http_method() -> String {
    let args = vec!["DELETE", "GET", "HEAD", "OPTION", "PATCH", "POST", "PUT"];
    let mut rnd = rand::thread_rng();
    let mut index: usize = 0;
    if args.len()-1 > 0 {
        index = rnd.gen_range(0, args.len() - 1);
    }

    return format!("{}", args[index]); // String::from(args[index]);
}

/// Return random integer in range. Must specify 1 or 2 numbers separated by comma.
/// If 1 argument is specified it is handled as "highest" value and `0` is used as lowest value.
///
/// _Note: The return type is `String`!_
/// ## Example
///```rust
///use fakedata_generator::gen_int;
///let i: String = gen_int("1,100".to_string());
/// // i = 42
/// ```
pub fn gen_int(input: String) -> String {
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let args = parse_args_to_vec(&input);
    let mut rnd = rand::thread_rng();

    if args.len() == 0 {
        return String::from("0");
    }

    if args.len() == 2 {
        i1 = args[0].parse().unwrap();
        i2 = args[1].parse().unwrap();
    }

    if args.len() == 1 {
        i1 = 0;
        i2 = args[0].parse().unwrap();
    }


    let rand_number = rnd.gen_range(i1, i2);

    return rand_number.to_string();
}

/// Generate IP address in `v4` format (`xxx.xxx.xxx.xxx`)
///
/// ## Example
/// ```
/// use fakedata_generator::gen_ipv4;
/// let ipv4 = gen_ipv4();
/// // => ipv4 = 172.129.23.201
/// ```
pub fn gen_ipv4() -> String {
    let mut rnd = rand::thread_rng();
    let a = rnd.gen_range(1, 255);
    let b = rnd.gen_range(1, 255);
    let c = rnd.gen_range(1, 255);
    let d = rnd.gen_range(1, 255);

    return format!("{}.{}.{}.{}", a, b, c, d);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_int() {
        let mut res = gen_int("1,10".to_string()).parse::<i32>().unwrap();
        assert_eq!(true, (res >= 1 && res <= 10));

        res = gen_int("10,300".to_string()).parse::<i32>().unwrap();
        assert_eq!(true, (res >= 10 && res <= 300));

        res = gen_int("300000,999999".to_string()).parse::<i32>().unwrap();
        assert_eq!(true, (res >= 300000 && res <= 999999));

        res = gen_int("99999999,1000000000".to_string()).parse::<i32>().unwrap();
        assert_eq!(true, (res >= 99999999 && res <= 1000000000));
    }

    #[test]
    fn test_gen_enum() {
        let mut words: String = gen_enum("hello,hola,hallo".to_string());
        let mut res = match words.as_str() {
            "hello" => true,
            "hola" => true,
            "hallo" => true,
            _ => false
        };
        assert_eq!(true, res);

        words = gen_enum("a,b,c,d,e,f,g,h,i,j".to_string());
        res = match words.as_str() {
            "a" => true,
            "b" => true,
            "c" => true,
            "d" => true,
            "e" => true,
            "f" => true,
            "g" => true,
            "h" => true,
            "i" => true,
            "j" => true,
            _ => false
        };
        assert_eq!(true, res);

        words = gen_enum("Hallo Welt,Hello world,Hola mundo".to_string());
        res = match words.as_str() {
            "Hallo Welt" => true,
            "Hello world" => true,
            "Hola mundo" => true,
            _ => false
        };
        assert_eq!(true, res);
    }

    // @TODO: Check if there's a better way to find if a value is in a Vec
    #[test]
    fn test_gen_http_method() {
        let possible_values: Vec<&str> = vec!["DELETE", "GET", "HEAD", "OPTION", "PATCH", "POST", "PUT"];
        let mut method: String = gen_http_method();

        for v in possible_values.to_owned() {
            if v == method.as_str() {
                assert_eq!(true, true);
                break;
            }
        }

        method = gen_http_method();
        for v in possible_values.to_owned() {
            if v == method.as_str() {
                assert_eq!(true, true);
                break;
            }
        }

        method = gen_http_method();
        for v in possible_values.to_owned() {
            if v == method.as_str() {
                assert_eq!(true, true);
                break;
            }
        }

        method = gen_http_method();
        for v in possible_values.to_owned() {
            if v == method.as_str() {
                assert_eq!(true, true);
                break;
            }
        }
    }
}