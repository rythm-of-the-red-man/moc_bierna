import { invoke } from "@tauri-apps/api";

const convertAsMeasurement = async (pathIn: string, pathOut: string) => {
  await invoke("convert", {
    pathIn,
    pathOut,
  });
};

export { convertAsMeasurement };
