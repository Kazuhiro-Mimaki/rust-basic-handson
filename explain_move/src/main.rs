// ==========================
// rustの所有権について
// ==========================
// rustにはGCがなく、リソースの解放は自動で行われる
// 関数やブロックの終了タイミングでリソースが解放される
// rustでは値の所有者が必ず1つ存在する

// struct User {
//     name: String,
// }

// fn some_action_to_user(user: User) {
//     let tmp = user;
//     // 何かアクションをする
//     println!("{}", tmp.name);
// }

// fn main() {
//     let user = User {
//         name: String::from("user_a"),
//     };
//     // 関数に user を渡すと、所有権がこの関数に移ってしまう
//     some_action_to_user(user);
//     // なので、この時点で user はもう main 関数には所有権がなくなっている -> コンパイルエラー
//     println!("one more: {}", user.name);
// }


// 所有権は貸し出しすることができ、これを借用と呼ぶ
// 変数に&をつけることで、その変数の所有権を貸し出すことができる

struct User {
    name: String,
}

fn some_action_to_user(user: &User) {
    let tmp = user;
    // 何かアクションをする
    println!("{}", tmp.name);
}

fn main() {
    let user = User {
        name: String::from("user_a"),
    };
    // 関数に user を渡すと、所有権がこの関数に移ってしまう
    some_action_to_user(&user);
    // なので、この時点で user はもう main 関数には所有権がなくなっている -> コンパイルエラー
    println!("one more: {}", user.name);
}
