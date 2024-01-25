//
// 探讨设计模式在 Rust 中怎样实现(本章设计模式使用的>>"状态模式"<<)
// -----------------------------------------------------------------------
//
//
// > 对比：
//   1. 如果需要一个不使用状态模式的替代实现，我们可能需要使用在 Post 中的方法
//      或者其他调用 Post 方法的地方使用 match 来处理不同状态的行为，这样我们
//      需要维护一个新的状态的时候，我们可能需要理解这些相互依赖的match分支的意图
//   2. 使用状态模式的时候，当我们需要了解某个状态的行为只需要查看对应状态的State
//      trait 方法实现，同时当我们需要增加新的状态的时候只需要添加一个新的结构体
//      来实现 State trait 并修改对应的它的方法(行为)。
// > 缺点:
//   1. 状态之间的转换是相互关联的，如果在中间增加状态，需要修改前后关联的对象的
//   2. 重复的代码逻辑, request_review 和 approve是高度重复的
//      (如果有很多这样的代码，考虑使用宏来消除这些重复)
//

pub fn  run() {
    let mut post = Post::new();
    post.add_text("hello world");

    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("hello world", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 暴露一个方法可以往 content 中传入一段文本而不是
    // 直接将 content 文本暴露到外部
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 返回博文内容
    pub fn content(&self) -> &str {
        //1.这里调用Option 的 as_ref 方法是因为需要Option 中值的引用而不是获取
        //  其所有权，因为 state 是一个Option<Box<dyn State>> 将调用 as_ref 会
        //  返回一个 Option<&Box<dyn State>>. 如果不调用 as_ref() 将会得到一个
        //  错误，因为不能将 state 移动出借用的 &self 函数参数.
        //2.得到一个&Box<dyn State>, 当调用其content时，Deref 强制转换会作用于
        //  & 和 Box, 这样最终会调用实现 State trait 类型的 content 方法
        self.state.as_ref().unwrap().content(self)
    }

    // 1.这里需要获取状态值的所有权, 这就是 Post 的 state 字段中 Option 的来历
    //   调用 take() 方法将state 字段中的 Some 值取出并留下一个None, 因为Rust
    //   不允许结构体实例中存在值为空的字段，这使得我们将state 的值移出 Post 而
    //   不是借用它，接着我们将博文的 state 值设置为这个操作的结果。
    // 2.我们需要将state临时设置为 None 来获取state值，即老状态的所有权，这样的
    //   代码直接更新状态值，者确保了当 Post 被转换为新的状态后不能再使用老状态的值
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // approve 与 request_review 类似
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 为所有实现了State trait 的结构体实现的默认实现，如果必要则不必覆写这个方法
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
