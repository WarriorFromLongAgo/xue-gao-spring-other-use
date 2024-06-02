#[cfg(test)]
mod struct_test_1 {
//     pub struct Post {
//         state: Option<Box<dyn State>>,
//         content: String,
//     }
//
//     impl Post {
//         pub fn new() -> Post {
//             Post {
//                 state: Some(Box::new(Draft {})),
//                 content: String::new(),
//             }
//         }
//         pub fn content(&self) -> &str {
//             self.state.as_ref().unwrap().content(self)
//         }
//         pub fn add_text(&mut self, text: &str) {
//             self.content.push_str(text);
//         }
//         // --snip--
//         pub fn request_review(&mut self) {
//             if let Some(s) = self.state.take() {
//                 self.state = Some(s.request_review())
//             }
//         }
//         pub fn approve(&mut self) {
//             if let Some(s) = self.state.take() {
//                 self.state = Some(s.approve())
//             }
//         }
//     }
//
//     trait State {
//         fn request_review(self: Box<Self>) -> Box<dyn State>;
//         fn approve(self: Box<Self>) -> Box<dyn State>;
//     }
//
//     struct Draft {}
//
//     impl State for Draft {
//         fn request_review(self: Box<Self>) -> Box<dyn State> {
//             Box::new(PendingReview {})
//         }
//         // --snip--
//         fn approve(self: Box<Self>) -> Box<dyn State> {
//             self
//         }
//     }
//
//     struct PendingReview {}
//
//     impl State for PendingReview {
//         fn request_review(self: Box<Self>) -> Box<dyn State> {
//             self
//         }
//         fn approve(self: Box<Self>) -> Box<dyn State> {
//             self
//         }
//     }
//
//     #[test]
//     fn struct_test_1() {
//         let mut post = Post::new();
//
//         post.add_text("I ate a salad for lunch today");
//         assert_eq!("", post.content());
//
//         post.request_review();
//         assert_eq!("", post.content());
//
//         post.approve();
//         assert_eq!("I ate a salad for lunch today", post.content());
//     }
}