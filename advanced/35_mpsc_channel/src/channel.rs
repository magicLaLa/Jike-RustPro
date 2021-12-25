use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
};

const ININIAL_SIZE: usize = 32;

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(ININIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

impl<T> Sender<T> {
    /// 生产者写入一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receiver left"));
        }
        // 加锁，访问 VecDeque，压入数据，然后立刻释放锁
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };

        // 通知任意一个被挂起等待的消费者有数据
        if was_empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        // 无锁 fast path
        if let Some(v) = self.cache.pop_front() {
            return Ok(v);
        }
        // 拿到队列的锁
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                // 读到数据返回，锁被释放
                Some(t) => {
                    if !inner.is_empty() {
                        std::mem::swap(&mut self.cache, &mut inner);
                    }
                    return Ok(t);
                }
                None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
                None => {
                    // 当 Condvar 被唤醒后会返回 MutexGuard，我们可以 loop 回去拿数据
                    // 这是为什么 Condvar 要在 loop 使用
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("lock poisoned"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

/// 克隆 sender
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

/// Drop sender
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        // sender 走光了，唤醒 receiver 读取数据（如果队列中还有的话），读不到就出错
        if old <= 1 {
            // 因为我们实现的是 MPSC，receiver 只有一个，所以 notify_all 实际等价 notify_one self.shared.available.notify_all(); } }}
            self.shared.available.notify_all();
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

/// 创建一个 unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(ININIAL_SIZE),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world!");
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        result.sort();

        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn recevier_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据，确保它和发送的数据一致
                assert_eq!(idx, i);
            }
            // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        // 1ms 足够让生产者发完 100 个消息，消费者消费完100 个消息并阻塞
        thread::sleep(Duration::from_millis(1));

        // 再次发送数据，唤醒消费者
        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        // 留点时间 让 receiver 处理
        thread::sleep(Duration::from_millis(1));

        // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        // sender 即用即抛弃
        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
                // sender 在此被丢弃
            })
            .join()
            .unwrap();
        }

        // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而，读取更多数据时会出错
        assert!(r.recv().is_err());
    }

    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn receiver_shall_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        let (mut sender, mut receiver) = unbounded::<usize>();
        let t1 = thread::spawn(move || {
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
        });

        thread::spawn(move || {
            receiver.recv().unwrap();
            drop(s);
        });

        t1.join().unwrap();
    }

    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded();
        for i in 0..10usize {
            s.send(i).unwrap();
        }

        assert!(r.cache.is_empty());
        // 读取一个数据，此时应该会导致 swap，cache 中有数据
        assert_eq!(0, r.recv().unwrap());
        // 还有 9 个数据在 cache 中
        assert_eq!(r.cache.len(), 9);
        // 在 queue 里没有数据了
        assert_eq!(s.total_queued_items(), 0);

        // 从 cache 里读取剩下的数据
        for (idx, i) in r.into_iter().take(9).enumerate() {
            assert_eq!(idx + 1, i);
        }
    }
}
