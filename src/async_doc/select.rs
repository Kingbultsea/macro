use futures::{
    stream::{Stream, StreamExt, FusedStream},
    select,
};

struct FusedRangeStream {
    range: std::ops::RangeInclusive<u8>,
}

impl Stream for FusedRangeStream {
    type Item = u8;

    fn poll_next(self: std::pin::Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.range.next() {
            Some(item) => std::task::Poll::Ready(Some(item)),
            None => std::task::Poll::Ready(None),
        }
    }
}

impl FusedStream for FusedRangeStream {
    fn is_terminated(&self) -> bool {
        self.range.end() < self.range.start()
    }
}

pub async fn main() -> u8 {
    let mut s1 = FusedRangeStream { range: 1..=5 };
    let mut s2 = FusedRangeStream { range: 6..=10 };
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    println!("select配合loop使用{}", total);

    total
}
