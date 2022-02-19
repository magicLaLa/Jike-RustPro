use futures::Stream;
use pin_project::pin_project;
use std::{
  task::{Context, Poll},
  pin::Pin,
};
use tokio::sync::mpsc::{Receiver};

struct ReceiverStream<T> {
  inner: Receiver<T>,
}

impl<T> ReceiverStream<T> {
    pub fn new(recv: Receiver<T>) -> Self {
      Self {
        inner: recv,
      }
    }
}

impl<T> Stream for ReceiverStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_recv(cx)
    }
}

impl<T> AsRef<Receiver<T>> for  ReceiverStream<T> {
    fn as_ref(&self) -> &Receiver<T> {
        &self.inner
    }
}

impl<T> AsMut<Receiver<T>> for  ReceiverStream<T> {
  fn as_mut(&mut self) -> &mut Receiver<T> {
      &mut self.inner
  }
}

impl<T> From<Receiver<T>> for  ReceiverStream<T> {
  fn from(recv: Receiver<T>) -> Self {
      Self::new(recv)
  }
}