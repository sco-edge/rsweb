use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;

// 예시로 실행 시간이 오래 걸리는 함수
fn long_running_function() {
    // 여기에 실행 시간이 오래 걸리는 코드 작성
    // 이 함수는 예시이므로 실제로는 사용자의 함수로 대체해야 합니다.
    thread::sleep(Duration::from_secs(5)); // 5초 동안 대기
}

fn main() {
    let (sender, receiver) = mpsc::channel();

    // 함수 실행 쓰레드
    let sender_clone = sender.clone();
    let handle = thread::spawn(move || {
        long_running_function();
        sender_clone.send("completed").unwrap(); // 실행 완료 메시지 전송
    });

    // 타임아웃 설정 쓰레드
    let timeout_duration = Duration::from_secs(1); // 타임아웃 기간: 3초
    let timeout_clone = sender.clone();
    thread::spawn(move || {
        thread::sleep(timeout_duration);
        if timeout_clone.send("timeout").is_ok() {
            println!("Function execution completed before timeout.");
        }
    });

    // 쓰레드가 완료되거나 타임아웃 메시지를 받을 때까지 대기
    if let Ok(msg) = receiver.recv() {
        match msg {
            "completed" => println!("Function completed within the timeout."),
            "timeout" => println!("Function execution timed out."),
            _ => println!("Unknown message."),
        }
    }

    // 쓰레드 핸들을 기다려 종료
    handle.join().unwrap();
}