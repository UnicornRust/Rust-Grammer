
//
// 介绍怎样使用 rust 的优势，利用类型系统来编写状态模式的代码
//
// 1. 
//
//


pub fn run() {

    let mut  post = Post::new();
    post.add_text("hello world");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("hello world", post.content());

}

// 使用不同的结构体来表示不同的状态，他们具有不同的行为（方法）
// 在不同的状态所有的方法不同，这样依赖，在某个状态(就是某个具体的结构体)
// 所使用的方法是严格隔离开来的，如果用错了，编译器会报错

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

//代表已经发布的状态
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    // 只有处于审核通过的状态才有文档内容查看的方法
    pub fn content(&self) -> &str {
        &self.content
    }
}

// 代表草稿状态的
impl DraftPost {
    // 只有处于草稿状态的结构体才有添加内容的方法
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 调用该方法就直接获取所有权同时返回一个代表新状态的结构体
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
