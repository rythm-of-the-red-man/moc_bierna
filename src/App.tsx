import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button } from "@mui/material";
import { open,save } from "@tauri-apps/api/dialog";
import { convertAsMeasurement } from "./api/as_measurements";


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }
  const openASMeasurementFile = async () => {
    const toOpen = await open({
      title:"Wybierz plik z pomiarami z licznika AS",
      multiple: false,
      filters: [
        {
          name: "Pomiar",
          extensions: ["LP"],
        },
      ],
    });
    const toSave = await save({
      title:"Wybierz plik do którego zapisane będą przekonwertowane dane.",
      filters: [
        {
          name: "Przekonwertowany pomiar",
          extensions: ["csv"],
        },
      ],
    });
    if (toOpen && toSave)
    await convertAsMeasurement(toOpen as string, toSave);
  };

  return (
    <div className="container">
      <div className="row">
        <Button onClick={openASMeasurementFile} variant="contained">
          Konwertuj plik z licznika AS
        </Button>
      </div>
    </div>
  );
}

export default App;
