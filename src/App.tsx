import { useMemo, useState } from "react";
import reactLogo from "./assets/react.svg";
import { open } from "@tauri-apps/api/dialog";
import { useLocalStorage, useSet } from "react-use";

import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button, Callout, H1, Section, Spinner } from "@blueprintjs/core";
import { Card, CardList } from "@blueprintjs/core";
import { Play, Stop } from "@blueprintjs/icons";

function App() {
  const [packs, setPacks] = useLocalStorage<any[]>("PKG-LIST", []);
  const [pkg, setPkg] = useLocalStorage("CURRENT_PKG", "");
  const [out, setOut] = useState("");
  const [runingCmd, { add: addRuning, remove: removeRuning }] =
    useSet<string>();

  const scripts = useMemo(() => {
    const pkgObject = packs!.find((item) => {
      return item.filePath === pkg;
    });

    const scripts = pkgObject?.scripts;

    if (!scripts) return [];

    return Object.keys(scripts);
  }, [packs, pkg]);

  const handleClick = async () => {
    // 打开一个目录选择对话框
    const selected = await open({
      directory: true, // 设置为 true 表示选择目录而非文件
      multiple: true, // 如果需要允许选择多个目录，则设置为 true
      // 可以设置 defaultPath 来指定对话框打开时的默认路径
    });

    if (Array.isArray(selected)) {
      const result = await invoke("get_files", { dir: selected[0] });
      if (result) {
        const packageObject = JSON.parse(result as string);

        packageObject.filePath = selected[0];

        setPkg(selected[0]);

        setPacks((arr) => {
          const oldPath = arr!.find((a) => a.filePath === selected[0]);

          if (oldPath) {
            Object.assign(oldPath, packageObject);
            return [...arr!];
          }
          return [...arr!, packageObject];
        });
      }
      // 用户选择了多个目录
    } else if (selected === null) {
      // 用户取消了选择
    } else {
      // 用户选择了单个目录
    }
  };

  const runScript = async (path: string, cmd: string) => {
    console.log("cmd: ", cmd);
    try {
      addRuning(path + cmd);
      const result = await invoke("run_script", { path: path, script: cmd });
      removeRuning(path + cmd);
      setOut(result as string);
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <div className="container">
      <div className="flex flex-1 overflow-auto">
        <CardList className="w-1/2">
          {packs!.map((pack, index) => (
            <Card
              key={pack.filePath}
              onClick={() => setPkg(pack.filePath)}
              selected={pkg === pack.filePath}
            >
              <div>{pack.name}</div>
              <div>{pack.version}</div>
            </Card>
          ))}
        </CardList>
        <Card className="flex-1">
          {scripts.map((script) => {
            return (
              <Card
                key={script}
                className="flex justify-center items-center gap-2"
              >
                <Spinner size={12} />
                <div>{script}</div>
                <Button
                  minimal={true}
                  intent="primary"
                  icon={!runingCmd.has(pkg + script) ? <Play /> : <Stop />}
                  onClick={() => runScript(pkg, script)}
                />
              </Card>
            );
          })}
        </Card>
      </div>

      <Callout className="h-40 overflow-auto" title="终端信息" intent="primary">
        <pre>{out}</pre>
      </Callout>
      <Button onClick={handleClick} intent="primary" className="h-10">
        Select Foloder
      </Button>
    </div>
  );
}

export default App;
