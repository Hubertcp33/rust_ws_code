// 定义WebSocket连接的URL
const wsUrl = "ws://localhost:7070/ws";

// 创建WebSocket连接
const conn = new WebSocket(wsUrl);
console.log("Connecting to ....");

// 当连接打开时，输出连接成功的消息
conn.onopen = function () {
    console.log("Connected to the server");
};

// 当接收到消息时，输出消息内容，并将消息内容添加到页面的log元素中
conn.onmessage = function (e) {
    console.log("Rec:" + e.data);
    const log = document.getElementById("log");
    log.textContent = log.textContent + "\n" + e.data;
    log.scrollTop = log.scrollHeight; // 自动滚动到底部
};

// 当连接关闭时，输出关闭的消息，并将连接置为null
conn.onclose = function () {
    console.log("closed,");
    conn = null;
};

// 发送消息的函数
function send() {
    // 获取输入框的值
    const input = document.getElementById("input");
    // 如果输入框的值不为空，则发送消息，并将输入框的值置为空
    if (input.value.trim() !== "") {
        conn.send(input.value);
        input.value = ""; // 清空输入框
    }
}

// 给按钮添加点击事件，点击按钮时发送消息
document.getElementById("btn").addEventListener("click", send);

// 支持按下回车键发送消息
document.getElementById("input").addEventListener("keypress", function (e) {
    if (e.key === "Enter") {
        send();
    }
});