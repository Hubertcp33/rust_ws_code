use std::{env, sync::Arc, collections::HashMap};
use tokio::sync::{mpsc, RwLock};
use warp::Filter;
use warp::ws::{Message, WebSocket};
use tokio_stream::wrappers::UnboundedReceiverStream;
use log::info;
use std::convert::Infallible;
use futures::StreamExt;


static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);//原子加1，多线程加1

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;


#[tokio::main]
async fn main() {
    env::set_var("RUST_APP_LOG", "debug");//设置环境变量 RUST_APP_LOG 为 debug，用于配置日志记录的级别
    pretty_env_logger::init_custom_env("RUST_APP_LOG"); //初始化日志记录器，使用环境变量 RUST_APP_LOG 的值来配置日志记录的级别。
    let users = Users::default();//创建一个 Users 类型的实例，用于管理用户信息
    let chat = warp::path("ws") //定义一个路由，用于处理 WebSocket 连接。当路径为 /ws 且请求包含 WebSocket 升级头时，将用户信息传递给 connect 函数，并在 WebSocket 连接升级后调用 connect 函数。
        .and( warp::ws())
        .and( with_users(users))
        .map( |ws: warp::ws::Ws, users |ws.on_upgrade(move |socket| connect
                (socket, users)));
       
        

    let files = warp::fs::dir("static"); //  创建一个静态文件服务器，指向static目录
    let routes = chat.or(files); //  将聊天服务和静态文件服务合并
    warp::serve(routes).run(([127, 0, 0, 1], 7070)).await; //  启动服务，监听127.0.0.1:7070

    
    
}

fn with_users(users: Users) -> impl Filter<Extract =  (Users,), Error = Infallible> + Clone {//回一个实现了 Filter trait 的对象，该对象在提取参数时返回一个 Users 对象，并且在错误处理时返回 Infallible。同时，该对象还实现了 Clone trait，可以克隆。
    warp::any().map(move || users.clone()) //  使用map函数，将用户信息传递给过滤器
}

async fn connect(ws: WebSocket, users: Users){
    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed); //获取id    
    info!("Connect User {}", my_id); //  打印连接的用户id
    let (user_tx, mut user_rx) = ws.split(); //  将WebSocket拆分为发送和接收通道
    let (tx, rx ) =mpsc::unbounded_channel(); //  创建一个无界通道，用于接收消息
    let rs = UnboundedReceiverStream::new(rx); //  创建一个UnboundedReceiverStream，用于接收rx中的消息
    tokio::spawn(rs.forward(user_tx)); //  使用tokio的spawn函数，将rs中的消息转发到user_tx中
    users.write().await.insert(my_id, tx); //  将my_id和tx插入到users中
    while let Some(result) = user_rx.next().await { //  循环接收user_rx中的消息
        broadcast(result.unwrap(), &users).await; //  将接收到的消息广播给所有用户
    }
    disconnect(my_id, &users).await; //  断开与my_id用户的连接
}
async fn broadcast(msg: Message, users: &Users) {
    if let Ok(_) = msg.to_str() { //  将消息转换为字符串
        for (&uid, tx) in users.read().await.iter() { //  遍历users中的所有用户
            info!("uid:{} sesnd msg: {:?}", uid, msg.clone()); //  打印日志
            tx.send(Ok(msg.clone())).expect( "Failed to send message"); //  将消息发送给用户
        }
    }

}

async fn disconnect(my_id: usize, users: &Users) {
    info!("Disconnect User {}", my_id); //  打印日志
    users.write().await.remove(&my_id); //  从users中移除my_id用户
}