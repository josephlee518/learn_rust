/*
 * 2019-11-12 Tue
 * https://doc.rust-lang.org/1.0.0/book/dining-philosophers.html
 */

use std::thread;
use std::time;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        /*
         * If the mutex is currently being accessed by someone else
         * we’ll block until it becomes available
         * 
         * 변수에 언더스코어(_)를 다는 이유는 언더스코어를 빼고 이름을 짓게 되면 워닝이 뜸
         * 왜냐? 만든 변수를 다른 부분에서 사용하지 않으니까!
         * 
         * .lock() 코드 뒤에 .unwrap() 함수를 추가한 이유는 다음을 참고
         * https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
         */
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is started eating!", self.name);
        /*
         * std::thread::sleep_ms 함수를 사용할 수 있지만
         * warning: use of deprecated item 'std::thread::sleep_ms': replaced by `std::thread::sleep`
         * 이라는 문구가 떠서
         * thread::sleep() 함수를 사용하여야 한다
         * sleep_ms를 사용할때는 인풋값에 따라 밀리세컨드를 중지했지만
         * thread::sleep을 사용할 경우에는 인풋값을 밀리세컨드로 바꾸어주어야 한다
         * time::Duration::from_millis 를 사용해야 하기 때문에 위에 use std::time 을 입력해 주어야 한다
         */
        let hundered_millis = time::Duration::from_millis(1000);
        thread::sleep(hundered_millis);
        println!("{} is Done eating!", self.name);
    }
}

fn main() {
    /*
     * 만약에 프로그램 실행 시 변수가 변경된다면 let {변수명}을 입력하지 말고
     * let mut {변수명} 을 입력해야 한다
     */
    // let philosophers = vec![
    //     Philosopher::new("Baruch Spinoza"),
    //     Philosopher::new("Gilles Deleuze"),
    //     Philosopher::new("Karl Marx"),
    //     Philosopher::new("Friedrich Nietzsche"),
    //     Philosopher::new("Michel Foucault"),
    // ];

    /*
     * 위의 코드를 주석처리하고 다음의 코드를 재 작성하여 table 코드를 만든다
     * 이 코드는 각 쓰레드의 변수를 공유하기 위하여 만들었다.
     * (ARC는 Atomic Reference Count를 의미하다)
     */

    let table = Arc::new(Table {forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    
    /*
     * Vector 은 다음과 같이 vec! 표시를 사용해서 생성할 수 있다
     */
    // let a = vec![
    //     1,
    //     2,
    //     3,
    // ];

    /*
     * 벡터를 println! 할 경우 다음과 같이 {:?}를 사용하여 표시할 수 있다
     * 하지만 {:?}는 순수한 벡터일 경우에나 표시할 수 있지, 위에 있는 예제외 같이
     * 뭔가에 의해서 쌓여있는 벡터는 표시하지 못한다
     * 
     * 라고 작성했지만, 테스트해보니 다른 벡터는 표시할 수 있다, {:?} 와 같이 표시하면
     * 되는 걸로 봐서는 되는데.. 
     * 
     * TODO: 왜 이런지 한번 찾아봐야 되겠다
     */
    // println!("{:?}", a);

    // 주석처리된 아래의 코드를 지우면 한개당 하나씩의 작업을 수행한다
    // 하지만 동시성(concurrently)을 추구한다면 어떨까?
    /*
     * for p in &philosophers {
     *    p.eat();
     * }
     */

    /*
     * 코드를 다음과 같이 작성하는데, into_iter() 이라는 함수가 눈에 띈다
     * iterator로 만들어주는 함수인데, 이것에 대해서는 Document를 참고
     * https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
     * 그리고 iterator 이라는 것에 대해서는 다음을 참고
     * http://occamsrazr.net/tt/78
     * 
     * 눈에 띄는 점은 벡터를 생성하는데, 벡터 안에 언더바 혹은 언더스코어(_)가 포함되어 있다
     * 이는 handle 이라는 변수는 타입이 "실행될때 판단"하겠다 라는 이야기이다
     * 
     * 그 이후에는 .map 이라는 함수를 사용하여 map 을 사용하여 리눅스에서 빠이프를 사용하여
     * 집어넣듯이 사용할 수 있는 것으로 보인다.
     * 
     * 
     */
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        /*
        thread::spawn(move || {
            p.eat();
        })
        * 데드락을 방지하기 위하여 다음과 같은 코드를 추가한다
        */
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    /*
     * 그 이후에는 handles<T>안에 들어있던 값을 꺼내서 하나씩 join 시킨다
     */
    for h in handles {
        h.join().unwrap();
    }
}