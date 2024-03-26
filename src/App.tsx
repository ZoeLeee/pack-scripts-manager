import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { open } from "@tauri-apps/api/dialog";

import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  const handleClick = async () => {
    // 打开一个目录选择对话框
    const selected = await open({
      directory: true, // 设置为 true 表示选择目录而非文件
      multiple: true, // 如果需要允许选择多个目录，则设置为 true
      // 可以设置 defaultPath 来指定对话框打开时的默认路径
    });

    console.log("selected: ", selected);

    if (Array.isArray(selected)) {
      const result = await invoke("get_files", { dir: selected[0] });
      console.log("result: ", result);
      // 用户选择了多个目录
    } else if (selected === null) {
      // 用户取消了选择
    } else {
      // 用户选择了单个目录
    }
  };

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>

      <button onClick={handleClick}>Select</button>
    </div>
  );
}

export default App;
