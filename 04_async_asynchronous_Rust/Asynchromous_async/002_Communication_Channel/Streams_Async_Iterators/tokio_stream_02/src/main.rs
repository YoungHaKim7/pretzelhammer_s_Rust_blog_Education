use tokio_stream::StreamExt;

struct Doubler {
    counter: u32,
}

impl tokio_stream::Stream for Doubler {
    type Item = u32;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.counter += 1;

        for _ in 0..1000 {
            if self.counter < 1000 {
                return std::task::Poll::Ready(Some(self.counter));
            } else {
                return std::task::Poll::Ready(None);
            }
        }
        std::task::Poll::Ready(None)
    }
}

#[tokio::main]
async fn main() {
    let mut stream = Doubler { counter: 1 };
    while let Some(n) = stream.next().await {
        println!("{n:?}");
    }
}
