extern crate rand;
use passt::Passt;
use rand::Rng;

pub mod data;

fn parse_args_to_vec(input: &str) -> Vec<&str> {
    let args: Vec<&str> = input.split(",").clone().collect();
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
    let user = gen_enum(String::from(
        "devankoshal,jesseddy,ahmadajmi,KarimMove,benefritz,meln1ks,shaneIxD,BryanHorsey,AnthraX,AmbientTech,CrucifiX,BronzeGamer,Scarface,b0rnc0nfused,XxX_SlAyEr_XxX",
    ));
    return user;
}

/// Returns a random password (= string of random chars)
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_password;
/// let pw: String = gen_password(32);
/// // pw => gXPMWpCYRbMexDxRdjRGPA2oyR0ABIJv
/// assert!(pw.len() == 32);
/// ```
pub fn gen_password(password_length: usize) -> String {
    let password = Passt::random_password(password_length as i32, None);
    return password;
}

/// Returns a random password (= string of random chars) with special chars
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_password_with_special_chars;
/// let pw: String = gen_password_with_special_chars(64);
/// // pw => ;w7f`av-f4l:1&n/010&ap0bPnlLiRn0S+.%C+)X?I9N_5=)uO)<:3+°iQH?T(y-
/// assert!(pw.chars().collect::<Vec<_>>().len() == 64);
/// ```
pub fn gen_password_with_special_chars(password_length: usize) -> String {
    let password = Passt::random_password(password_length as i32, Some(true));
    return password;
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
/// Possible Top-Level Domains (TLDs): list based on <https://data.iana.org/TLD/tlds-alpha-by-domain.txt>
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_domain;
/// let domain = gen_domain();
/// // domain => names.eu
/// ```
pub fn gen_domain() -> String {
    let tld = gen_enum(data::gen_switch("tlds".to_string()));
    let domain = gen_enum(String::from("some,random,names,we,make,up,for,testing"));
    return format!("{}.{}", &domain, &tld);
}

/// Return a randomly generated e-Mail address. This generator uses the `gen_username` generator.
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_email;
/// let email: String = gen_email();
/// assert_ne!(email, "");
/// assert!(email.contains("@"));
/// ```
pub fn gen_email() -> String {
    let user = gen_username();
    let tld: String = gen_enum("de,org,com,net,io,email,dev".to_string());
    let domain: String = gen_enum(
        "mail-services,postfach,box.mail,mail.cyberspace,hmail,coldmail,nahoo,mail".to_string(),
    );

    let email: String = format!("{}.{}", &domain, &tld);

    return format!("{}@{}", &user, &email);
}

/// Return random string from set of specified strings. Specify a comma separated list as argument.
///
/// ## Example
/// ```rust
///use fakedata_generator::gen_enum;
///let word: String = gen_enum("some,random,words".to_string());
/// // word = "random"
/// ```
pub fn gen_enum(input: impl ToString) -> String {
    let input_var = input.to_string();
    let args = parse_args_to_vec(input_var.as_str());
    let mut rnd = rand::thread_rng();
    let mut index: usize = 0;
    if args.len() - 1 > 0 {
        index = rnd.gen_range(0..args.len() - 1);
    }

    return format!("{}", args[index]);
}

/// Return random HTTP Method, taken from <https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods>
/// Possible values:
/// * GET
/// * HEAD
/// * POST
/// * PUT
/// * DELETE
/// * CONNECT
/// * OPTIONS
/// * TRACE
/// * PATCH
///
/// ## Example
/// ```rust
/// use fakedata_generator::gen_http_method;
/// let method: String = gen_http_method();
/// assert!(["GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH"].contains(&method.as_str()));
/// ```
pub fn gen_http_method() -> String {
    let args = vec![
        "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
    ];
    let mut rnd = rand::thread_rng();
    // the length of the args vec doesn't change so we don't need to calculate it.
    let index: usize = rnd.gen_range(0..8);

    return format!("{}", args[index]); // String::from(args[index]);
}

/// Return random integer in range. Must specify 1 or 2 numbers separated by comma.
/// If 1 argument is specified it is handled as "highest" value and `0` is used as lowest value.
///
/// _Note: The return type is `String`!_
/// ## Example
///```rust
///use fakedata_generator::gen_int;
///let i = gen_int("1,100".to_string()).parse::<i32>().unwrap();
/// assert!(i <= 100 && i >= 1)
/// ```
pub fn gen_int(input: impl ToString) -> String {
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let input_val = input.to_string();
    let args = parse_args_to_vec(&input_val.as_str());
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

    let rand_number = rnd.gen_range(i1..i2);

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
    let a = rnd.gen_range(1..255);
    let b = rnd.gen_range(1..255);
    let c = rnd.gen_range(1..255);
    let d = rnd.gen_range(1..255);

    return format!("{}.{}.{}.{}", a, b, c, d);
}
/// Generate a private IP address.
///
/// There's 3 blocks of private IP addresses:
/// 10.0.0.0 – 10.255.255.255
/// 172.16.0.0 – 172.31.255.255
/// 192.168.0.0 – 192.168.255.255
///
/// Choose a block by providing the starting number as range:
/// gen_private_ipv4(10) -> 10.x.x.x
/// gen_private_ipv4(172) -> 172.16.x.x
/// gen_private_ipv4(192) -> 192.168.x.x
/// ## Example
/// ```
/// use fakedata_generator::gen_private_ipv4;
/// let private_ipv4 = gen_private_ipv4(10);
/// // => private_ipv4 = 10.128.20.21
/// ```
pub fn gen_private_ipv4(starting_range: usize) -> String {
    let mut rnd = rand::thread_rng();
    let a = match starting_range {
        10 => 10,
        172 => 172,
        192 => 192,
        _ => 10,
    };
    let b = match a {
        10 => rnd.gen_range(1..255),
        172 => rnd.gen_range(16..31),
        192 => 168,
        _ => 0,
    };
    let c = rnd.gen_range(1..255);
    let d = rnd.gen_range(1..255);

    return format!("{}.{}.{}.{}", a, b, c, d);
}

#[cfg(test)]
mod tests {
    use crate::data::gen_prime;

    use super::*;

    #[test]
    fn test_gen_int() {
        let mut res = gen_int("1,10").parse::<i32>().unwrap();
        assert_eq!(true, (res >= 1 && res <= 10));

        res = gen_int("10,300").parse::<i32>().unwrap();
        assert_eq!(true, (res >= 10 && res <= 300));

        res = gen_int("300000,999999").parse::<i32>().unwrap();
        assert_eq!(true, (res >= 300000 && res <= 999999));

        res = gen_int("99999999,1000000000")
            .parse::<i32>()
            .unwrap();
        assert_eq!(true, (res >= 99999999 && res <= 1000000000));
    }

    #[test]
    fn test_gen_enum() {
        let mut words: String = gen_enum("hello,hola,hallo");
        let mut res = match words.as_str() {
            "hello" => true,
            "hola" => true,
            "hallo" => true,
            _ => false,
        };
        assert_eq!(true, res);

        words = gen_enum("a,b,c,d,e,f,g,h,i,j");
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
            _ => false,
        };
        assert_eq!(true, res);

        words = gen_enum("Hallo Welt,Hello world,Hola mundo".to_string());
        res = match words.as_str() {
            "Hallo Welt" => true,
            "Hello world" => true,
            "Hola mundo" => true,
            _ => false,
        };
        assert_eq!(true, res);
    }

    // @TODO: Check if there's a better way to find if a value is in a Vec
    #[test]
    fn test_gen_http_method() {
        let possible_values: Vec<&str> =
            vec!["DELETE", "GET", "HEAD", "OPTION", "PATCH", "POST", "PUT"];
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

    #[test]
    fn test_gen_private_ipv4_first_block() {
        let rand_ip_10 = gen_private_ipv4(10);
        let rand_ip_172 = gen_private_ipv4(172);
        let rand_ip_192 = gen_private_ipv4(192);
        assert!(rand_ip_10.starts_with("10"));
        assert!(rand_ip_172.starts_with("172"));
        assert!(rand_ip_192.starts_with("192"));
    }

    #[test]
    fn test_gen_prime() {
        // very much not validating primes here, 
        // but instead just the bounds of the array.
        let prime = gen_prime();
        assert!(prime > 1);
        assert!(prime <= 8017);
    }

    #[test]
    fn test_gen_tvshows() {
        // just validating we get something back here.
        let show = data::gen_switch("tvshow".into());
        assert_ne!(show, "");
        assert_ne!(show, "Error: dataset not found");
    }

    #[test]
    fn test_gen_not_available() {
        let show = data::gen_switch("does-not-exist".into());
        assert_eq!(show, "Error: dataset not found");
    }
        
}
