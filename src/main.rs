#[derive(Debug, PartialEq)]
struct Container (Vec<Container>);

pub struct Stream {
    ptr: usize,
    cnt: Vec<char>
}

impl Stream {
    
    pub fn next(&mut self) -> Option<char> {
        if self.ptr == self.cnt.len() {
            None
        } else {
            Some(self.cnt[self.ptr])
        }
    }
    
    pub fn set(&mut self, ptr: usize) {
        self.ptr = ptr
    }
    
    pub fn new<T: IntoIterator<Item=char>> (v: T) -> Stream {
        Stream {
            cnt: v.into_iter().collect(),
            ptr: 0
        }
    }
}

trait Parser<T> {
    fn get_parser(&self) -> &Fn(&mut Stream) -> Result<T, ()>;
}

macro_rules! make {
    
    ($name:ident: $ok:ty, $s:ident-> $cnt:block) => (
        struct $name;
        impl $name {
            fn function($s: &mut Stream) -> Result<$ok, ()> {
                $cnt
            }
         }
         impl Parser<$ok> for $name {
             fn get_parser(&self) -> &'static Fn(&mut Stream) -> Result<$ok, ()> {
                 static M: fn(&mut Stream) -> Result<$ok, ()> = $name::function;
                 &M
             }
         }
    )
}

macro_rules! one_char {
    ($name:ident: $c:expr) => 
    (make! { $name: (), s -> {
            if s.next() == Some($c) {
                s.ptr += 1;
                Ok(())
            } else {
                Err(())
            }
        }
    })
}

one_char! { LeftBrace: '[' }

one_char! { RightBrace: ']' }

make! { Pair: Container, s -> {
    let ptr = s.ptr;
    if s.next() == None || s.next() == Some(']') {
        return Ok(Container(Vec::new()))
    }
    if LeftBrace.get_parser()(s).is_ok() {
        if let Ok(v) = Pair.get_parser()(s) {
            if RightBrace.get_parser()(s).is_ok() {
                return Ok(Container(vec![v]))
            }
        }
    }
    s.ptr = ptr;
    return Err(())
}}

fn main() {
    macro_rules! parse_pair {
        ($str:expr) => ({
            let mut s = Stream::new($str.chars());
            Pair.get_parser()(&mut s)
        })
    }
    println!("{:?}", parse_pair!("[[]]"));
    assert_eq!(Ok(Container(Vec::new())), parse_pair!(""));
    assert_eq!(Err(()), parse_pair!("["));
}
